//! Concurrency soundness suite for the default `thread_safe` feature.
//!
//! With `thread_safe` (a default feature) `Pdfium` is `unsafe impl Send + Sync`,
//! yet pdfium's C API is non-reentrant. These tests are 100% safe Rust (no
//! `unsafe` block) but drive pdfium concurrently in the ways the `Send + Sync`
//! bounds permit. On an unsound implementation they race pdfium's C state and
//! crash (SIGSEGV / heap corruption / abort) or return corrupted results; on a
//! sound implementation every FFI operation is serialized and each test passes.
//!
//! The first test renders and discards its output, so it only exercises the
//! render path. The remaining tests cover the scenarios that render-and-discard
//! misses:
//!
//! * `concurrent_bitmap_buffer_reads` reads pdfium's rendered pixel buffer back
//!   out (the borrowed-memory return path the discarding test never touches),
//!   and checks the bytes are deterministic across threads.
//! * `concurrent_text_extraction` exercises the multi-call get-length-then-get
//!   text extraction path and checks it is deterministic under contention.
//! * `concurrent_same_page_shared_across_threads` shares one `&PdfPage` across
//!   threads and renders plus extracts text from it at the same time, checking
//!   that operations on a single shared object stay atomic and correct.
//! * `concurrent_load_failure_reports_correct_error` loads a mix of valid and
//!   invalid buffers and checks every failed load reports its own definite
//!   error, proving the load-then-`FPDF_GetLastError` sequence stays atomic
//!   (pdfium's last-error is process-global state).
//! * `concurrent_form_render_with_highlight` renders form field data with
//!   highlighting, exercising the multi-call set-highlight-then-`FPDF_FFLDraw`
//!   sequence that touches pdfium's process-global form state.
//! * `thread_local_pdfium_object_drops_cleanly_at_teardown` stores a document in
//!   a `thread_local!` and lets it drop while the thread is being torn down, so
//!   its Drop acquires the FFI lock during teardown. The lock keeps only a
//!   destructor-free depth counter in thread-local storage (the mutex guard lives
//!   in the lock value), so this must not trip a "TLS accessed during/after
//!   destruction" panic.

use once_cell::sync::OnceCell;
use pdfium_render::prelude::*;
use std::cell::RefCell;
use std::sync::Arc;
use std::thread;

/// Returns the single process-wide [Pdfium] instance shared by every test.
///
/// `Pdfium::new` asserts that the global bindings have not yet been set, so it
/// can only be called once per process. Integration tests in the same file
/// share a process, so all tests here bind through this `OnceCell`.
fn pdfium() -> &'static Pdfium {
    static PDFIUM: OnceCell<Pdfium> = OnceCell::new();

    PDFIUM.get_or_init(|| {
        Pdfium::new(
            Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
                .or_else(|_| Pdfium::bind_to_system_library())
                .expect("bind pdfium"),
        )
    })
}

/// A stable FNV-1a hash of a byte buffer, used to compare rendered output
/// across iterations and threads without storing the whole buffer.
fn checksum(bytes: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;

    for &byte in bytes {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }

    hash
}

/// Classifies a load result into a stable label so tests can assert on the
/// exact failure reason without requiring the error types to implement `Eq`.
fn classify_load(result: &Result<PdfDocument<'_>, PdfiumError>) -> &'static str {
    match result {
        Ok(_) => "ok",
        Err(PdfiumError::PdfiumLibraryInternalError(inner)) => match inner {
            PdfiumInternalError::FileError => "file",
            PdfiumInternalError::FormatError => "format",
            PdfiumInternalError::PasswordError => "password",
            PdfiumInternalError::SecurityError => "security",
            PdfiumInternalError::PageError => "page",
            PdfiumInternalError::Unknown => "unknown",
        },
        Err(_) => "other",
    }
}

/// Baseline test: many threads share one [Pdfium], each loading its own
/// document and rendering concurrently. Renders are discarded, so this only
/// proves the render path does not corrupt pdfium's C state. It crashes on an
/// implementation that does not serialize the render calls.
#[test]
fn concurrent_render_shared_pdfium_races_pdfium_c_state() {
    let num_threads = 8;
    let iters = 200;

    let handles: Vec<_> = (0..num_threads)
        .map(|_| {
            let pdfium = pdfium();
            thread::spawn(move || {
                let config = PdfRenderConfig::new().set_target_width(300);
                for _ in 0..iters {
                    let doc = pdfium
                        .load_pdf_from_file("test/export-test.pdf", None)
                        .expect("load document");
                    for page in doc.pages().iter() {
                        let _bitmap = page.render_with_config(&config).expect("render page");
                    }
                }
            })
        })
        .collect();

    for h in handles {
        h.join().expect("worker thread panicked");
    }
}

/// Reads pdfium's rendered pixel buffer back out under contention.
///
/// The baseline test discards its bitmaps and so never reads the buffer that
/// pdfium fills. This test renders the same page and reads its pixels via
/// `as_raw_bytes`, which returns data derived from pdfium-owned memory. The
/// rendered bytes are fully deterministic, so a torn read (pixels observed
/// while another thread mutates pdfium's state) shows up as a checksum that
/// disagrees with a single-threaded baseline.
#[test]
fn concurrent_bitmap_buffer_reads() {
    let baseline = {
        let doc = pdfium()
            .load_pdf_from_file("test/export-test.pdf", None)
            .expect("load document");
        let page = doc.pages().get(0).expect("first page");
        let config = PdfRenderConfig::new().set_target_width(300);
        let bytes = page
            .render_with_config(&config)
            .expect("render page")
            .as_raw_bytes();

        assert!(!bytes.is_empty(), "rendered pixel buffer must not be empty");

        checksum(&bytes)
    };

    let num_threads = 8;
    let iters = 80;

    let handles: Vec<_> = (0..num_threads)
        .map(|_| {
            let pdfium = pdfium();
            thread::spawn(move || {
                let config = PdfRenderConfig::new().set_target_width(300);
                for _ in 0..iters {
                    let doc = pdfium
                        .load_pdf_from_file("test/export-test.pdf", None)
                        .expect("load document");
                    let page = doc.pages().get(0).expect("first page");
                    let bytes = page
                        .render_with_config(&config)
                        .expect("render page")
                        .as_raw_bytes();

                    assert!(!bytes.is_empty(), "rendered pixel buffer must not be empty");
                    assert_eq!(
                        checksum(&bytes),
                        baseline,
                        "concurrent read of the rendered buffer disagreed with the \
                         single-threaded baseline, indicating a torn read of pdfium memory"
                    );
                }
            })
        })
        .collect();

    for h in handles {
        h.join().expect("worker thread panicked");
    }
}

/// Extracts text under contention, exercising the multi-call extraction path.
///
/// Extracting page text is a get-length-then-get-contents sequence over
/// pdfium's non-reentrant text API. Per-call locking leaves that sequence open
/// to interleaving, which can panic (a length read on one thread paired with a
/// contents read reflecting another thread's page) or silently return the wrong
/// string. The extracted text is deterministic, so this test compares every
/// concurrent extraction against a single-threaded baseline.
#[test]
fn concurrent_text_extraction() {
    let baseline = {
        let doc = pdfium()
            .load_pdf_from_file("test/text-test.pdf", None)
            .expect("load document");
        let page = doc.pages().get(0).expect("first page");
        let text = page.text().expect("load text page").all();

        assert!(
            !text.is_empty(),
            "fixture page must contain extractable text"
        );

        text
    };

    let num_threads = 8;
    let iters = 120;

    let handles: Vec<_> = (0..num_threads)
        .map(|_| {
            let pdfium = pdfium();
            let expected = baseline.clone();
            thread::spawn(move || {
                for _ in 0..iters {
                    let doc = pdfium
                        .load_pdf_from_file("test/text-test.pdf", None)
                        .expect("load document");
                    let page = doc.pages().get(0).expect("first page");
                    let text = page.text().expect("load text page");

                    assert_eq!(
                        text.all(),
                        expected,
                        "concurrent text extraction returned a different string than the \
                         single-threaded baseline, indicating a torn multi-call read"
                    );
                }
            })
        })
        .collect();

    for h in handles {
        h.join().expect("worker thread panicked");
    }
}

/// Shares a single `&PdfPage` across threads and uses it concurrently.
///
/// The baseline test gives each thread its own document, so no two threads ever
/// touch the same pdfium object. Because `PdfPage` is `Send + Sync`, safe code
/// can share one page across threads and render plus extract text from it at the
/// same time. Each such operation is a sequence of C calls against shared page
/// state; only holding the lock across the whole operation keeps the results
/// correct. Both operations are deterministic, so a wrong result here means an
/// operation on the shared object was not atomic.
#[test]
fn concurrent_same_page_shared_across_threads() {
    let doc = pdfium()
        .load_pdf_from_file("test/text-test.pdf", None)
        .expect("load document");
    let page = doc.pages().get(0).expect("first page");

    let render_baseline = {
        let config = PdfRenderConfig::new().set_target_width(300);
        checksum(
            &page
                .render_with_config(&config)
                .expect("render page")
                .as_raw_bytes(),
        )
    };
    let text_baseline = page.text().expect("load text page").all();

    assert!(!text_baseline.is_empty(), "fixture page must contain text");

    let page_ref = &page;
    let render_baseline = &render_baseline;
    let text_baseline = &text_baseline;

    let num_threads = 8;
    let iters = 150;

    thread::scope(|scope| {
        for tid in 0..num_threads {
            scope.spawn(move || {
                for _ in 0..iters {
                    if tid % 2 == 0 {
                        let config = PdfRenderConfig::new().set_target_width(300);
                        let bytes = page_ref
                            .render_with_config(&config)
                            .expect("render shared page")
                            .as_raw_bytes();

                        assert_eq!(
                            checksum(&bytes),
                            *render_baseline,
                            "rendering a shared page concurrently produced a different \
                             image than the single-threaded baseline"
                        );
                    } else {
                        let text = page_ref.text().expect("load text page").all();

                        assert_eq!(
                            &text, text_baseline,
                            "extracting text from a shared page concurrently produced a \
                             different string than the single-threaded baseline"
                        );
                    }
                }
            });
        }
    });
}

/// Loads a mix of valid and invalid buffers concurrently and checks that every
/// failed load reports its own definite error.
///
/// pdfium reports load failures through the process-global `FPDF_GetLastError`,
/// read immediately after the load call. If those two calls are not held under
/// one lock, another thread's successful load can reset the global error code in
/// between, so a genuine parse failure is misreported as `Unknown` (pdfium's
/// "no error recorded" fallback). This test asserts that valid buffers always
/// load and invalid buffers always report a definite parse or file error, never
/// the `Unknown` fallback and never spuriously succeeding.
#[test]
fn concurrent_load_failure_reports_correct_error() {
    let valid = Arc::new(std::fs::read("test/export-test.pdf").expect("read fixture"));
    let invalid: Arc<Vec<u8>> =
        Arc::new(b"this is deliberately not a valid pdf document, only plain ascii text".to_vec());

    let num_threads = 8;
    let iters = 150;

    let handles: Vec<_> = (0..num_threads)
        .map(|_| {
            let pdfium = pdfium();
            let valid = Arc::clone(&valid);
            let invalid = Arc::clone(&invalid);
            thread::spawn(move || {
                for _ in 0..iters {
                    let good = pdfium.load_pdf_from_byte_vec((*valid).clone(), None);
                    assert_eq!(
                        classify_load(&good),
                        "ok",
                        "a valid document failed to load under contention"
                    );
                    drop(good);

                    let bad = pdfium.load_pdf_from_byte_vec((*invalid).clone(), None);
                    let label = classify_load(&bad);
                    assert!(
                        label == "format" || label == "file",
                        "an invalid document reported `{label}` instead of a definite parse or \
                         file error, indicating the load and last-error read were not atomic"
                    );
                }
            })
        })
        .collect();

    for h in handles {
        h.join().expect("worker thread panicked");
    }
}

/// Renders one shared page with two different highlight colors concurrently.
///
/// Rendering form data sets the document form environment's highlight color and
/// then calls `FPDF_FFLDraw` to draw the fields. Those are separate C calls, and
/// the color is stored in state shared by every render of the same document.
/// When one page is shared across threads, holding the lock only per call lets
/// one thread's set-color land between another thread's set-color and draw, so
/// the wrong color is drawn. Each render is deterministic, so a leaked color
/// shows up as a checksum that does not match the single-threaded baseline for
/// that thread's own color. This is the multi-call, shared-state case that the
/// plain render path never exercises.
#[test]
fn concurrent_form_highlight_color_race() {
    let doc = pdfium()
        .load_pdf_from_file("test/form-test.pdf", None)
        .expect("load document");
    let page = doc.pages().get(0).expect("first page");

    let render_with = |color: PdfColor| -> u64 {
        let config = PdfRenderConfig::new()
            .set_target_width(400)
            .render_form_data(true)
            .highlight_all_form_fields(color);
        checksum(
            &page
                .render_with_config(&config)
                .expect("render page")
                .as_raw_bytes(),
        )
    };

    let red = PdfColor::new(255, 0, 0, 200);
    let blue = PdfColor::new(0, 0, 255, 200);
    let red_baseline = render_with(red);
    let blue_baseline = render_with(blue);

    assert_ne!(
        red_baseline, blue_baseline,
        "the form highlight color must change the rendered output for this test to be \
         meaningful; the fixture may lack form fields or a form environment"
    );

    let page_ref = &page;
    let red_baseline = &red_baseline;
    let blue_baseline = &blue_baseline;

    let num_threads = 8;
    let iters = 60;

    thread::scope(|scope| {
        for tid in 0..num_threads {
            let (color, expected) = if tid % 2 == 0 {
                (red, red_baseline)
            } else {
                (blue, blue_baseline)
            };
            scope.spawn(move || {
                let config = PdfRenderConfig::new()
                    .set_target_width(400)
                    .render_form_data(true)
                    .highlight_all_form_fields(color);
                for _ in 0..iters {
                    let bytes = page_ref
                        .render_with_config(&config)
                        .expect("render shared page with form highlight")
                        .as_raw_bytes();

                    assert_eq!(
                        checksum(&bytes),
                        *expected,
                        "a concurrent render with a different highlight color leaked into \
                         this render, indicating the set-color and draw calls were not atomic"
                    );
                }
            });
        }
    });
}

thread_local! {
    // A Pdfium object held in thread-local storage, dropped at thread teardown.
    static THREAD_LOCAL_DOC: RefCell<Option<PdfDocument<'static>>> =
        const { RefCell::new(None) };
}

/// Drops a `thread_local!`-held [PdfDocument] during thread teardown.
///
/// The document's Drop calls `FPDF_CloseDocument` while holding the FFI lock, so
/// the lock is acquired *during* thread teardown. If the lock kept its outermost
/// mutex guard in a thread-local (a destructor-bearing slot), that acquisition
/// could run after the slot's own destructor and panic with "cannot access a
/// Thread Local Storage value during or after destruction" — an abort inside a
/// Drop. Keeping only a destructor-free depth counter in thread-local storage, and
/// the guard in the lock value, avoids that. Every spawned thread must join
/// cleanly.
#[test]
fn thread_local_pdfium_object_drops_cleanly_at_teardown() {
    let bytes = Arc::new(std::fs::read("test/export-test.pdf").expect("read fixture"));

    for _ in 0..64 {
        let bytes = Arc::clone(&bytes);
        thread::spawn(move || {
            THREAD_LOCAL_DOC.with(|slot| {
                let doc = pdfium()
                    .load_pdf_from_byte_vec((*bytes).clone(), None)
                    .expect("load document");

                // Touch the document under the lock so its handle is live, then
                // leave it in thread-local storage to be dropped during teardown.
                let _ = doc.pages().len();
                *slot.borrow_mut() = Some(doc);
            });
        })
        .join()
        .expect("thread panicked while tearing down a thread_local Pdfium object");
    }
}
