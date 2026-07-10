//! Enforces the FFI serialization invariant for the `thread_safe` feature.
//!
//! Pdfium's C API is not reentrant, so under `thread_safe` every call that
//! reaches into the bindings must run while the process-wide `FfiLock` is held.
//! The convention is that any function which performs an FFI call acquires the
//! lock as its first statement:
//!
//! ```ignore
//! #[cfg(feature = "thread_safe")]
//! let _ffi = crate::pdfium::FfiLock::acquire();
//! ```
//!
//! This test scans the crate source and fails if any function performs an FFI
//! call (through the `bindings()` accessor or a stored `bindings` field) without
//! acquiring the lock. A function that legitimately does not need the lock, or
//! that is guaranteed to run under a caller-held lock, can opt out with an
//! explicit `// ffi-lock-exempt: <reason>` marker in its body.
//!
//! The check is a mechanical, greppable safety net: it cannot be silently
//! defeated by adding an unlocked FFI call, the way per-statement locking hidden
//! inside a smart pointer could be.

use std::fs;
use std::path::{Path, PathBuf};

/// Replaces line comments, block comments, and string and character literals
/// with equivalent-length runs of spaces (preserving newlines), so that neither
/// brace counting nor token search is fooled by braces, `bindings()` mentions in
/// doc comments, or quoted text.
fn strip_noise(src: &str) -> String {
    #[derive(PartialEq)]
    enum State {
        Normal,
        LineComment,
        BlockComment,
        Str,
        Char,
    }

    let bytes = src.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut state = State::Normal;
    let mut i = 0;

    while i < bytes.len() {
        let b = bytes[i];
        let next = bytes.get(i + 1).copied().unwrap_or(0);

        match state {
            State::Normal => {
                if b == b'/' && next == b'/' {
                    state = State::LineComment;
                    out.push(b' ');
                    out.push(b' ');
                    i += 2;
                    continue;
                } else if b == b'/' && next == b'*' {
                    state = State::BlockComment;
                    out.push(b' ');
                    out.push(b' ');
                    i += 2;
                    continue;
                } else if b == b'"' {
                    state = State::Str;
                    out.push(b' ');
                    i += 1;
                    continue;
                } else if b == b'\'' {
                    // Distinguish a character literal ('x', '\n', '\'') from a
                    // lifetime ('a). A lifetime is an apostrophe followed by an
                    // identifier character and NOT closed by a following quote.
                    let is_char_literal = if next == b'\\' {
                        true
                    } else {
                        bytes.get(i + 2).copied() == Some(b'\'')
                    };

                    if is_char_literal {
                        state = State::Char;
                        out.push(b' ');
                        i += 1;
                        continue;
                    } else {
                        out.push(b);
                        i += 1;
                        continue;
                    }
                } else {
                    out.push(b);
                    i += 1;
                    continue;
                }
            }
            State::LineComment => {
                if b == b'\n' {
                    state = State::Normal;
                    out.push(b'\n');
                } else {
                    out.push(b' ');
                }
                i += 1;
            }
            State::BlockComment => {
                if b == b'*' && next == b'/' {
                    state = State::Normal;
                    out.push(b' ');
                    out.push(b' ');
                    i += 2;
                } else {
                    out.push(if b == b'\n' { b'\n' } else { b' ' });
                    i += 1;
                }
            }
            State::Str => {
                if b == b'\\' {
                    out.push(b' ');
                    out.push(b' ');
                    i += 2;
                } else if b == b'"' {
                    state = State::Normal;
                    out.push(b' ');
                    i += 1;
                } else {
                    out.push(if b == b'\n' { b'\n' } else { b' ' });
                    i += 1;
                }
            }
            State::Char => {
                if b == b'\\' {
                    out.push(b' ');
                    out.push(b' ');
                    i += 2;
                } else if b == b'\'' {
                    state = State::Normal;
                    out.push(b' ');
                    i += 1;
                } else {
                    out.push(b' ');
                    i += 1;
                }
            }
        }
    }

    String::from_utf8(out).unwrap()
}

fn is_ident_char(b: u8) -> bool {
    b == b'_' || b.is_ascii_alphanumeric()
}

/// A single function body extracted from a source file.
struct Function {
    line: usize,
    body: String,
}

/// Splits stripped source into function bodies by finding each `fn` keyword and
/// brace-matching its body. Bodyless trait method declarations (ending in `;`
/// before any `{`) and `fn` pointer types are skipped.
fn functions(stripped: &str) -> Vec<Function> {
    let bytes = stripped.as_bytes();
    let mut result = Vec::new();
    let mut i = 0;

    while i + 2 <= bytes.len() {
        // Find a `fn` keyword on a word boundary.
        if &bytes[i..i + 2] == b"fn"
            && (i == 0 || !is_ident_char(bytes[i - 1]))
            && bytes.get(i + 2).map_or(true, |&b| !is_ident_char(b))
        {
            // Require whitespace then an identifier start (a definition), not a
            // `fn(` pointer type.
            let mut j = i + 2;
            while j < bytes.len() && (bytes[j] == b' ' || bytes[j] == b'\n' || bytes[j] == b'\t') {
                j += 1;
            }
            if j >= bytes.len() || !(bytes[j] == b'_' || bytes[j].is_ascii_alphabetic()) {
                i += 2;
                continue;
            }

            // Scan forward to the body-opening brace, bailing if a `;` shows the
            // declaration has no body.
            let mut k = j;
            let mut found_brace = false;
            while k < bytes.len() {
                match bytes[k] {
                    b'{' => {
                        found_brace = true;
                        break;
                    }
                    b';' => break,
                    _ => k += 1,
                }
            }
            if !found_brace {
                i = j;
                continue;
            }

            // Brace-match the body.
            let start = k;
            let mut depth = 0i32;
            while k < bytes.len() {
                match bytes[k] {
                    b'{' => depth += 1,
                    b'}' => {
                        depth -= 1;
                        if depth == 0 {
                            break;
                        }
                    }
                    _ => {}
                }
                k += 1;
            }
            let end = (k + 1).min(bytes.len());
            let line = stripped[..start].bytes().filter(|&b| b == b'\n').count() + 1;
            result.push(Function {
                line,
                body: stripped[start..end].to_string(),
            });

            i = end;
        } else {
            i += 1;
        }
    }

    result
}

/// Blanks out `mod tests { ... }` and `mod test { ... }` blocks (preserving
/// newlines) so unit tests, which run single-threaded and legitimately call the
/// bindings without the lock, are not scanned.
fn remove_test_blocks(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out = bytes.to_vec();
    let mut i = 0;

    while i + 3 < bytes.len() {
        let is_mod = &bytes[i..i + 3] == b"mod"
            && (i == 0 || !is_ident_char(bytes[i - 1]))
            && bytes.get(i + 3).map_or(false, |&b| b == b' ');
        if !is_mod {
            i += 1;
            continue;
        }

        let mut j = i + 4;
        while j < bytes.len() && bytes[j] == b' ' {
            j += 1;
        }
        let name_start = j;
        while j < bytes.len() && is_ident_char(bytes[j]) {
            j += 1;
        }
        let name = &s[name_start..j];
        if name != "tests" && name != "test" {
            i += 3;
            continue;
        }

        while j < bytes.len() && bytes[j] != b'{' && bytes[j] != b';' {
            j += 1;
        }
        if j >= bytes.len() || bytes[j] == b';' {
            i += 3;
            continue;
        }

        let block_start = j;
        let mut depth = 0i32;
        while j < bytes.len() {
            match bytes[j] {
                b'{' => depth += 1,
                b'}' => {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
                _ => {}
            }
            j += 1;
        }
        let block_end = (j + 1).min(bytes.len());

        for b in out.iter_mut().take(block_end).skip(block_start) {
            if *b != b'\n' {
                *b = b' ';
            }
        }

        i = block_end;
    }

    String::from_utf8(out).unwrap()
}

fn rust_files(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            rust_files(&path, out);
        } else if path.extension().map_or(false, |e| e == "rs") {
            out.push(path);
        }
    }
}

/// Returns true if the (noise-stripped) function body makes a Pdfium FFI call.
///
/// Every Pdfium C function the crate calls is named `FPDF*` or `FORM_*` and is
/// invoked as `<ref>.FPDF_x(...)` or `<ref>.FORM_x(...)`. Matching the call
/// itself, rather than the `bindings()` accessor, catches FFI made through the
/// accessor, a stored `bindings` field, or a passed `bindings` parameter alike,
/// including a call chained onto the next line, and does not flag pure threading
/// constructors that merely hand the bindings to a child.
fn performs_ffi_call(body: &str) -> bool {
    body.contains(".FPDF") || body.contains(".FORM_")
}

fn holds_lock(body: &str) -> bool {
    body.contains("FfiLock::acquire")
}

fn is_exempt(body: &str) -> bool {
    // The marker lives in the ORIGINAL source as a comment; look it up there.
    body.contains("ffi-lock-exempt")
}

#[test]
fn every_ffi_call_holds_the_ffi_lock() {
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut files = Vec::new();
    rust_files(&src, &mut files);
    files.sort();

    let mut violations = Vec::new();

    for file in &files {
        // Skip the bindings implementation layer (the trait, its dynamic /
        // static / wasm impls, and generated bindgen). Those forward into the C
        // API and run under the high-level caller's lock; they do not self-lock.
        let path_str = file.to_string_lossy();
        if path_str.ends_with("bindings.rs")
            || path_str.contains("/bindings/")
            || path_str.contains("bindgen")
        {
            continue;
        }

        let raw = fs::read_to_string(file).unwrap();
        let stripped = remove_test_blocks(&strip_noise(&raw));

        // Map exemption markers back onto the stripped body by keeping the marker
        // token visible: re-check against the raw text per function line range.
        for func in functions(&stripped) {
            if !performs_ffi_call(&func.body) {
                continue;
            }
            if holds_lock(&func.body) {
                continue;
            }
            // Exemptions are comments, which strip_noise removed; look them up in
            // the raw source within this function's line span.
            let raw_lines: Vec<&str> = raw.lines().collect();
            let body_lines = func.body.lines().count();
            let start = func.line.saturating_sub(1);
            let end = (start + body_lines).min(raw_lines.len());
            let raw_body = raw_lines[start..end].join("\n");
            if is_exempt(&raw_body) {
                continue;
            }

            let rel = file.strip_prefix(&src).unwrap_or(file).display();
            violations.push(format!("  src/{}:{}", rel, func.line));
        }
    }

    assert!(
        violations.is_empty(),
        "Found {} function(s) that make a Pdfium FFI call without acquiring the FfiLock.\n\
         Add `#[cfg(feature = \"thread_safe\")] let _ffi = crate::pdfium::FfiLock::acquire();` as \
         the first statement, or mark an intentional exception with `// ffi-lock-exempt: <reason>`:\n{}",
        violations.len(),
        violations.join("\n")
    );
}
