# Idiomatic Rust bindings for Pdfium

`pdfium-render` provides an idiomatic high-level Rust interface around the low-level bindings to
Pdfium exposed by the excellent `pdfium-sys` crate.

```
    // Renders each page in the given test PDF file to a separate JPEG file.

    use pdfium_render::prelude::*;

    // Bind to the system-provided Pdfium library.
    
    let pdfium = Pdfium::new(Pdfium::bind_to_system_library().unwrap());

    // Load a PDF file.
    
    let document = pdfium.load_pdf_from_file("test.pdf", None).unwrap();
    
    // Set our desired bitmap rendering options.
 
    let bitmap_render_config = PdfBitmapConfig::new()
        .set_target_width(2000)
        .set_maximum_height(2000)
        .rotate_if_landscape(PdfBitmapRotation::Degrees90, true);

    // Render each page to a bitmap image, then export each image to a JPEG file.
 
    document.pages().iter().for_each(|page| {
        page.get_bitmap_with_config(&bitmap_render_config).unwrap()
            .as_image() // Renders this page to an Image::DynamicImage
            .as_rgba8().unwrap()
            .save_with_format(
              format!("test-page-{}.jpg", page.index()),
              image::ImageFormat::Jpeg
            ).unwrap();
    });
```

In addition to providing a more natural interface to Pdfium, `pdfium-render` differs from
`pdfium-sys` in several other important ways:

* `pdfium-render` uses `libloading` to late bind to a Pdfium library at run-time, whereas
  `pdfium-sys` binds to a library at compile-time. By binding to Pdfium at run-time instead
  of compile-time, `pdfium-render` can dynamically switch between bundled libraries and
  system libraries and provide idiomatic Rust error handling in situations where a Pdfium
  library is not available.
* Late binding to Pdfium means that `pdfium-render` can be compiled to WASM for running in a
  browser; this is not possible with `pdfium-sys`.
* Pdfium is composed as a set of separate modules, each covering a different aspect of PDF creation,
  rendering, and editing. `pdfium-sys` only provides bindings for the subset of functions exposed
  by Pdfium's view module; `pdfium-render` aims to ultimately provide bindings to all non-interactive
  functions exposed by all Pdfium modules, including document creation and editing functions.
  This is a work in progress. 
* Pages rendered by Pdfium can be exported as instances of `Image::DynamicImage` for easy,
  idiomatic post-processing.

Examples demonstrating page rendering, text extraction, page object introspection, and
compiling to WASM are available at <https://github.com/ajrcarey/pdfium-render/tree/master/examples>.

## What's new

Version 0.5.7 adds the ability to bind to a build of Pdfium that has been statically linked
into the final Rust executable.

Version 0.5.6 added the `pdfium_render::prelude`, adds bindings to Pdfium's `FPDFAnnot_*()`
and `FPDFPage_*Annot*()` functions, and adds the `PdfPageAnnotations` collection and
`PdfPageAnnotation` enum to the `pdfium-render` high-level interface. Not all annotation-related
functionality is currently available through the high-level interface; this will be added
gradually over time.
 
## Porting existing Pdfium code from other languages

The high-level idiomatic Rust interface provided by the `Pdfium` struct is entirely optional;
the `Pdfium` struct wraps around raw FFI bindings defined in the `PdfiumLibraryBindings`
trait, and it is completely feasible to simply use the FFI bindings directly
instead of the high level interface. This makes porting existing code that calls FPDF_* functions
trivial, while still gaining the benefits of late binding and WASM compatibility.
For instance, the following code snippet (taken from a C++ sample):

```
    string test_doc = "test.pdf";

    FPDF_InitLibrary();
    FPDF_DOCUMENT doc = FPDF_LoadDocument(test_doc, NULL);
    // ... do something with doc
    FPDF_CloseDocument(doc);
    FPDF_DestroyLibrary();
```

would translate to the following Rust code:

```
    let bindings = Pdfium::bind_to_system_library().unwrap();
    
    let test_doc = "test.pdf";

    bindings.FPDF_InitLibrary();
    let doc = bindings.FPDF_LoadDocument(test_doc, None);
    // ... do something with doc
    bindings.FPDF_CloseDocument(doc);
    bindings.FPDF_DestroyLibrary();
```

Pdfium's API uses three different string types: classic C-style null-terminated char arrays,
UTF-8 byte arrays, and a UTF-16LE byte array type named `FPDF_WIDESTRING`. For functions that take a
C-style string or a UTF-8 byte array, `pdfium-render`'s binding will take the standard Rust `&str` type.
For functions that take an `FPDF_WIDESTRING`, `pdfium-render` exposes two functions: the vanilla
`FPDF_*()` function that takes an `FPDF_WIDESTRING`, and an additional `FPDF_*_str()` helper function
that takes a standard Rust `&str` and converts it internally to an `FPDF_WIDESTRING` before calling
Pdfium. Examples of functions with additional `_str()` helpers include `FPDFBookmark_Find()`,
`FPDFAnnot_SetStringValue()`, and `FPDFText_SetText()`.

The `PdfiumLibraryBindings::get_pdfium_utf16le_bytes_from_str()` and
`PdfiumLibraryBindings::get_string_from_pdfium_utf16le_bytes()` utility functions are provided
for converting to and from `FPDF_WIDESTRING` in your own code.

Note that the `FPDF_LoadDocument()` function is not available when compiling to WASM.
Either embed the target PDF document directly using Rust's `include_bytes!()`
macro, or use Javascript's `fetch()` API to retrieve the bytes of the target document over
the network, then load those bytes into Pdfium using the `FPDF_LoadMemDocument()` function.

## Binding to Pdfium

`pdfium-render` does not include Pdfium itself. You have several options:

* Bind to a dynamically-built Pdfium library provided by the operating system.
* Bind to a dynamically-built Pdfium library packaged alongside your Rust executable.
* Bind to a statically-built Pdfium library linked to your executable at compile time.

When compiling to WASM, packaging an external build of Pdfium as a separate WASM module is essential.

## Dynamic linking

Binding to a dynamically-built Pdfium library is the simplest option. On Android, a system-provided
`libpdfium.so` is packaged as part of the operating system; alternatively, you can package a pre-built
dynamic library appropriate for your operating system alongside your Rust executable.

* Native builds of Pdfium for all major platforms: <https://github.com/bblanchon/pdfium-binaries/releases>
* WASM builds of Pdfium: <https://github.com/paulo-coutinho/pdfium-lib/releases>

At the time of writing, the WASM builds at <https://github.com/bblanchon/pdfium-binaries/releases>
are compiled with a non-growable WASM heap memory allocator. This means that attempting to open
a PDF document longer than just a few pages will result in a unrecoverable out of memory error.
The WASM builds at <https://github.com/paulo-coutinho/pdfium-lib/releases> are recommended as they
do not have this problem.

## Static linking

If you prefer link Pdfium directly into your executable at compile time, use the optional `static`
crate feature. This enables the `Pdfium::bind_to_statically_linked_library()` function which binds
directly to the Pdfium functions included in your executable:

```
    use pdfium_render::prelude::*;

    let pdfium = Pdfium::new(Pdfium::bind_to_statically_linked_library().unwrap());
```

As a convenience, `pdfium-render` can instruct `cargo` to link a statically-built Pdfium
library for you. Set the path to the directory containing your pre-built library using
the `PDFIUM_STATIC_LIB_PATH` environment variable when you run `cargo build`, like so:

```
    PDFIUM_STATIC_LIB_PATH="/path/containing/your/static/pdfium/library" cargo build
```

`pdfium-render` will pass the following flags to `cargo`:

```
    cargo:rustc-link-lib=static=pdfium
    cargo:rustc-link-search=native=$PDFIUM_STATIC_LIB_PATH
```

This saves you writing a custom `build.rs` yourself. If you have your own build pipeline
that links Pdfium statically into your executable, simply leave the `PDFIUM_STATIC_LIB_PATH`
environment variable unset.

Note that the path you set in `PDFIUM_STATIC_LIB_PATH` should not include the filename of the
library itself; it should just be the path of the containing directory. You must make sure your
statically-built library is named in the appropriate way for your target platform
(`libpdfium.a` on Linux and macOS, for example) in order for the Rust compiler to locate it.

`pdfium-render` will not build Pdfium for you; you must build Pdfium yourself, or source a
pre-built static archive from elsewhere.

## Compiling to WASM

See <https://github.com/ajrcarey/pdfium-render/tree/master/examples> for a full example that shows
how to bundle a Rust application using `pdfium-render` alongside a pre-built Pdfium WASM module for
inspection and rendering of PDF files in a web browser.

## Optional features

This crate provides the following optional features:

* `bindings`: uses `cbindgen` to generate Rust bindings to the Pdfium functions defined in the
  `include/*.h` files each time `cargo build` is run. If `cbindgen` or any of its dependencies
  are not available then the build will fail.
* `static`: enables binding to a statically-linked build of Pdfium.
  See the "Static linking" section above.

Neither feature is enabled by default.

## Development status

The initial focus of this crate has been on rendering pages in a PDF file; consequently, `FPDF_*`
functions related to bitmaps and rendering have been prioritised. By 1.0, the functionality of all
`FPDF_*` functions exported by all Pdfium modules will be available, with the exception of certain
functions specific to interactive scripting, user interaction, and printing.

* Releases numbered 0.4.x added support for all page rendering Pdfium functions to `pdfium-render`.
* Releases numbered 0.5.x-0.6.x aim to progressively add support for all read-only Pdfium functions to `pdfium-render`.
* Releases numbered 0.7.x-0.8.x aim to progressively add support for all Pdfium editing functions to `pdfium-render`.
* Releases numbered 0.9.x aim to fill any remaining gaps in the high-level interface prior to 1.0.0.

There are 368 `FPDF_*` functions in the Pdfium API. As of version 0.5.7, 187 (51%) have
bindings available in `pdfium-render`, with the functionality of roughly two-thirds of these
available via the high-level interface.

If you need a binding to a Pdfium function that is not currently available, just raise an issue.

## Version history

* 0.5.9: corrects a bug in the statically linked bindings implementation. Adjusted tests
  to cover both dynamic and statically linked bindings implementations.
* 0.5.8: corrects a bug in the WASM implementation of certain `FPDFAnnot_*()` functions. Resolves
  a potential memory leak affecting the WASM implementation of various `FPDF_*()` functions.
* 0.5.7: adds support for binding to a statically-linked build of Pdfium, adds `bindgen` and `static` crate features.
* 0.5.6: adds `pdfium_render::prelude`, adds bindings for `FPDFAnnot_*()` and `FPDFPage_*Annot*()`
  functions, adds `PdfPageAnnotations` collection and `PdfPageAnnotation` struct
  to the high-level interface.
* 0.5.5: fixes two bugs in the WASM implementation, one to do with colors,
  one to do with text extraction.
  See <https://github.com/ajrcarey/pdfium-render/issues/9> and
  <https://github.com/ajrcarey/pdfium-render/issues/11> for more information.
* 0.5.4: changes default setting of `PdfBitmapConfig::set_reverse_byte_order()` to `true` to
  switch from Pdfium's default BGRA8 pixel format to RGBA8. This is necessary since
  the `image` crate dropped support for BGRA8 in version 0.24. See
  <https://github.com/ajrcarey/pdfium-render/issues/9> for more information.
* 0.5.3: adds bindings for `FPDFBookmark_*()`, `FPDFPageObj_*()`, `FPDFText_*()`, and
  `FPDFFont_*()` functions, adds `PdfPageObjects`, `PdfPageText`, and `PdfBookmarks` collections
  to the high-level interface.
* 0.5.2: adds bindings for `FPDF_GetPageBoundingBox()`, `FPDFDoc_GetPageMode()`,
  `FPDFPage_Get*Box()`, and `FPDFPage_Set*Box()` functions, adds `PdfPageBoundaries` collection
  to the high-level interface.
* 0.5.1: adds bindings for `FPDFPage_GetRotation()` and `FPDFPage_SetRotation()` functions,
  adds `PdfMetadata` collection to the high-level interface.
* 0.5.0: adds rendering of annotations and form field elements, thanks to an excellent contribution
  from <https://github.com/inzanez>
* 0.4.2: bug fixes in `PdfBitmapConfig` implementation
* 0.4.1: improvements to documentation and READMEs
* 0.4.0: initial release
