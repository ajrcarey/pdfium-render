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
//! call (`<ref>.FPDF_x(...)` or `<ref>.FORM_x(...)`, however the bindings
//! reference is obtained) without acquiring the lock. A function that
//! legitimately does not need the lock, or that is guaranteed to run under a
//! caller-held lock, can opt out with an explicit `// ffi-lock-exempt: <reason>`
//! marker in its body.
//!
//! This is a mechanical net, not a proof. It reliably catches the common failure
//! (a function that calls into Pdfium with no lock at all) and rejects the known
//! footguns (a `let _ = acquire()` that drops the guard immediately; a file with
//! a raw string literal that its lexer cannot analyze). It does not, and a text
//! scan cannot, prove ordering: it does not verify the acquire lexically precedes
//! the call, sits at function scope rather than in an inner block, or that a
//! closure holding the lock is not returned and run later. It also does not see
//! FFI made by fully-qualified path (`PdfiumLibraryBindings::FPDF_x(b)`). Those
//! shapes do not occur in the source today; the runtime tests in
//! `thread_safety_soundness.rs` are the backstop that would catch them.

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
    body.contains("FfiLock::acquire") && !has_bare_underscore_acquire(body)
}

/// Detects `let _ = FfiLock::acquire();`, which binds the guard to `_` and so
/// drops it immediately, leaving the FFI call it was meant to protect unlocked.
/// The correct form binds a named `let _ffi = ...` held to the end of the scope.
fn has_bare_underscore_acquire(body: &str) -> bool {
    let mut rest = body;
    while let Some(pos) = rest.find("let _ =") {
        let after = &rest[pos + "let _ =".len()..];
        let stmt = &after[..after.find(';').unwrap_or(after.len())];
        if stmt.contains("FfiLock::acquire") {
            return true;
        }
        rest = after;
    }
    false
}

/// Raw string literals (`r"..."`, `r#"..."#`, `br"..."`, ...) are not understood
/// by `strip_noise`, whose simple string state machine would desynchronize on the
/// inner quotes and silently stop analyzing the rest of the file. There are none
/// in the source today; rather than analyze a file we cannot lex correctly, the
/// gate fails loudly so the lexer is extended before a raw string is introduced.
fn contains_raw_string(raw: &str) -> bool {
    let bytes = raw.as_bytes();
    for i in 0..bytes.len() {
        // A raw string opens with `r` or `br` at a token boundary (the preceding
        // character is not part of an identifier), then zero or more `#`, then `"`.
        let boundary = i == 0 || !is_ident_char(bytes[i - 1]);
        let r = if bytes[i] == b'r' && boundary {
            Some(i)
        } else if bytes[i] == b'b' && bytes.get(i + 1) == Some(&b'r') && boundary {
            Some(i + 1)
        } else {
            None
        };
        if let Some(r) = r {
            let mut j = r + 1;
            while bytes.get(j) == Some(&b'#') {
                j += 1;
            }
            if bytes.get(j) == Some(&b'"') {
                return true;
            }
        }
    }
    false
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
        let rel = file.strip_prefix(&src).unwrap_or(file);
        let rel_str = rel.to_string_lossy().replace('\\', "/");

        // Skip the bindings implementation layer (the trait, its dynamic / static
        // / wasm impls, and generated bindgen). Those forward into the C API and
        // run under the high-level caller's lock; they do not self-lock. Match
        // exact paths, not substrings, so an unrelated future file such as
        // `pdf/appearance_bindings.rs` is not silently exempted.
        if rel_str == "bindings.rs"
            || rel_str.starts_with("bindings/")
            || rel_str == "bindgen.rs"
            || rel_str.starts_with("bindgen/")
        {
            continue;
        }

        let raw = fs::read_to_string(file).unwrap();

        if contains_raw_string(&raw) {
            violations.push(format!(
                "  src/{rel_str}: contains a raw string literal, which strip_noise \
                 cannot lex; extend the lexer before adding one"
            ));
            continue;
        }

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
