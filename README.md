# Idiomatic Rust bindings for Pdfium

`pdfium-render` provides an idiomatic high-level Rust interface to Pdfium, the C++ PDF library
used by the Google Chromium project. With this library, you can render pages in PDF files to
bitmaps, load, edit, and extract text and images from existing PDF files, and create new PDF files
from scratch.

```rust
    use pdfium_render::prelude::*;

    fn export_pdf_to_jpegs(path: &str, password: Option<&str>) -> Result<(), PdfiumError> {
        // Renders each page in the given test PDF file to a separate JPEG file.

        // Bind to a Pdfium library in the same directory as our Rust executable;
        // failing that, fall back to using a Pdfium library provided by the operating system.

        let pdfium = Pdfium::new(
            Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
                .or_else(|_| Pdfium::bind_to_system_library())?,
        );

        // Load the document...

        let document = pdfium.load_pdf_from_file(path, password)?;

        // ... set rendering options that will be applied to all pages...

        let render_config = PdfRenderConfig::new()
            .set_target_width(2000)
            .set_maximum_height(2000)
            .rotate_if_landscape(PdfBitmapRotation::Degrees90, true);

        // ... then render each page to a bitmap image, saving each image to a JPEG file.

        for (index, page) in document.pages().iter().enumerate() {
            page.render_with_config(&render_config)?
                .as_image() // Renders this page to an image::DynamicImage...
                .as_rgba8() // ... then converts it to an image::Image...
                .ok_or(PdfiumError::ImageError)?
                .save_with_format(
                    format!("test-page-{}.jpg", index), 
                    image::ImageFormat::Jpeg
                ) // ... and saves it to a file.
                .map_err(|_| PdfiumError::ImageError)?;
        }

        Ok(())
    }
```

`pdfium-render` binds to a Pdfium library at run-time, allowing for flexible selection of
system-provided or bundled Pdfium libraries and providing idiomatic Rust error handling in
situations where a Pdfium library is not available. A key advantage of binding to Pdfium at run-time
rather than compile-time is that a Rust application using `pdfium-render` can be compiled to WASM
for running in a browser alongside a WASM-packaged build of Pdfium.

`pdfium-render` aims to eventually provide bindings to all non-interactive functionality provided
by Pdfium. This is a work in progress that will be completed by version 1.0 of this crate.

## Examples

Short, commented examples that demonstrate all the major Pdfium document handling features are
available at <https://github.com/ajrcarey/pdfium-render/tree/master/examples>. These examples demonstrate:

* Rendering pages to bitmaps.
* Text and image extraction.
* Document signature introspection.
* Document attachment introspection.
* Document concatenation.
* Page object introspection.
* Page annotation introspection.
* Creation of new documents and new pages.
* Creation of page objects for text, paths, and bitmaps.
* Multi-page tiled output.
* Watermarking.
* Thread safety.
* Compiling to WASM.

## What's new

Version 0.7.19 adds bindings to all Pdfium functions related to document attachments, and adds
the `PdfAttachments` and `PdfSignatures` collections to the high-level interface. 

Version 0.7.18 adds convenience functions for returning the Pdfium bindings used by `PdfDocument`,
`PdfPage`, `PdfBitmap`, `PdfFont`, and various other interfaces, thanks to an excellent
contribution from <https://github.com/LU15W1R7H>.

Version 0.7.17 relaxes some unnecessarily restrictive lifetime bounds in `PdfPageObjectPath`.

Version 0.7.16 adds additional convenience functions for quickly creating stand-alone
cubic Bézier path page objects and adding them to a mutable collection of page objects.

Version 0.7.15 adds additional functions for working with page annotations, including retrieval
of annotation names, author names, comments, creation and modification timestamps,
and text and character ranges linked to annotations.

## Binding to Pdfium

`pdfium-render` does not include Pdfium itself. You have several options:

* Bind to a dynamically-built Pdfium library provided by the operating system.
* Bind to a dynamically-built Pdfium library packaged alongside your Rust executable.
* Bind to a statically-built Pdfium library linked to your executable at compile time.

When compiling to WASM, packaging an external build of Pdfium as a separate WASM module is essential.

## Dynamic linking

Binding to a dynamically-built Pdfium library is the simplest option. On Android, a system-provided
`libpdfium.so` is packaged as part of the operating system (although recent versions of Android no
longer permit user applications to access it); alternatively, you can package a pre-built
dynamic library appropriate for your operating system alongside your Rust executable.

* Native (i.e. non-WASM) builds of Pdfium for all major platforms: <https://github.com/bblanchon/pdfium-binaries/releases>
* WASM builds of Pdfium: <https://github.com/paulocoutinhox/pdfium-lib/releases>

If you are compiling a native (i.e. non-WASM) build, and you place an appropriate Pdfium library
in the same folder as your compiled application, then binding to it dynamically at runtime is
as simple as:

```rust
    use pdfium_render::prelude::*;

    let pdfium = Pdfium::new(
        Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./")).unwrap()
    );
```

A common pattern used in the examples at <https://github.com/ajrcarey/pdfium-render/tree/master/examples>
is to first attempt to bind to a Pdfium library in the same folder as the compiled example, and
attempt to fall back to a system-provided library if that fails:

```rust
    use pdfium_render::prelude::*;

    let pdfium = Pdfium::new(
        Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
            .or_else(|_| Pdfium::bind_to_system_library())
            .unwrap() // Or use the ? unwrapping operator to pass any error up to the caller
    );
```

At the time of writing, the WASM builds at <https://github.com/bblanchon/pdfium-binaries/releases>
are compiled with a non-growable WASM heap memory allocator. This means that attempting to open
a PDF document longer than just a few pages will result in an unrecoverable out of memory error.
The WASM builds at <https://github.com/paulocoutinhox/pdfium-lib/releases> are recommended as they
do not have this problem.

## Static linking

If you prefer to link Pdfium directly into your executable at compile time, use the optional `static`
crate feature. This enables the `Pdfium::bind_to_statically_linked_library()` function which binds
directly to the Pdfium functions included in your executable:

```rust
    use pdfium_render::prelude::*;

    let pdfium = Pdfium::new(Pdfium::bind_to_statically_linked_library().unwrap());
```

As a convenience, `pdfium-render` can instruct `cargo` to link a statically-built Pdfium
library for you. Set the path to the directory containing your pre-built library using
the `PDFIUM_STATIC_LIB_PATH` environment variable when you run `cargo build`, like so:

```rust
    PDFIUM_STATIC_LIB_PATH="/path/containing/your/static/pdfium/library" cargo build
```

`pdfium-render` will pass the following flags to `cargo`:

```rust
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

The `Pdfium::load_pdf_from_file()` and `Pdfium::load_pdf_from_reader()` functions are not available
when running in the browser. The `Pdfium::load_pdf_from_bytes()` function is available, and
the following additional functions are provided:

* The `Pdfium::load_pdf_from_fetch()` function uses the browser's built-in `fetch()` API
  to download a URL over the network and open it as a PDF document.
* The `Pdfium::load_pdf_from_blob()` function opens a PDF document from the byte data in a Javascript
  `Blob` or `File` object, including `File` objects returned from an `<input type="file">` element. 

The `PdfDocument::save_to_file()` function is not available when running in the browser.
The `PdfDocument::save_to_bytes()` and `PdfDocument::save_to_writer()` functions are
available, and the following additional function is provided:

* The `PdfDocument::save_to_blob()` function returns the byte data for the document as a
  Javascript `Blob` object.

The following additional functions are provided during rendering:

* The `PdfBitmap::as_image_data()` function renders directly to a Javascript `ImageData` object,
  ready to display in an HTML `<canvas>` element.
* The `PdfBitmap::as_array()` function renders directly to a Javascript `Uint8Array` object.
  This function avoids a memory allocation and copy required by both `PdfBitmap::as_bytes()`
  and `PdfBitmap::as_image_data()`, making it preferable for applications where performance is paramount.

The `PdfFont::load_type1_from_file()` and `PdfFont::load_true_type_from_file()` functions are
not available when running in the browser. The following additional functions are provided:

* The `PdfFont::load_type1_from_fetch()` function uses the browser's built-in `fetch()` API
  to download a URL over the network and load it as a Type 1 font.
* The `PdfFont::load_true_type_from_fetch()` function uses the browser's built-in `fetch()` API
  to download a URL over the network and load it as a TrueType font.
* The `PdfFont::load_type1_from_blob()` function loads a Type 1 font from the byte data in a
  Javascript `Blob` or `File` object, including `File` objects returned from an `<input type="file">`
  element.
* The `PdfFont::load_true_type_from_blob()` function loads a TrueType font from the byte data in a
  Javascript `Blob` or `File` object, including `File` objects returned from an `<input type="file">`
  element.

## Multithreading

Pdfium makes no guarantees about thread safety and should be assumed _not_ to be thread safe.
The Pdfium authors specifically recommend that parallel processing, not multi-threading,
be used to process multiple documents simultaneously.

`pdfium-render` achieves thread safety by locking access to Pdfium behind a mutex;
each thread must acquire exclusive access to this mutex in order to make any call to Pdfium.
This has the effect of sequencing all calls to Pdfium as if they were single-threaded,
even when using `pdfium-render` from multiple threads. This approach offers no performance benefit,
but it ensures that Pdfium will not crash when running as part of a multi-threaded application.

An example of safely using `pdfium-render` as part of a multithreaded parallel iterator is
available at <https://github.com/ajrcarey/pdfium-render/tree/master/examples>.

## Optional features

This crate provides the following optional features:

* `bindings`: uses `cbindgen` to generate Rust bindings to the Pdfium functions defined in the
  `include/*.h` files each time `cargo build` is run. If `cbindgen` or any of its dependencies
  are not available then the build will fail.
* `static`: enables binding to a statically-linked build of Pdfium. See the "Static linking" section above.
* `thread_safe`: wraps access to Pdfium behind a mutex to ensure thread-safe access to Pdfium.
  See the "Multithreading" section above. 

The `thread_safe` feature is enabled by default. All other features are disabled by default.

## Porting existing Pdfium code from other languages

The high-level idiomatic Rust interface provided by `pdfium-render` is built on top of 
raw FFI bindings defined in the `PdfiumLibraryBindings` trait. It is completely feasible to use
these raw FFI bindings directly if you wish, making porting existing code that calls `FPDF_*` functions
trivial while still gaining the benefits of late binding and WASM compatibility.
For instance, the following code snippet (taken from a C++ sample):

```cpp
    string test_doc = "test.pdf";

    FPDF_InitLibrary();
    FPDF_DOCUMENT doc = FPDF_LoadDocument(test_doc, NULL);
    // ... do something with doc
    FPDF_CloseDocument(doc);
    FPDF_DestroyLibrary();
```

would translate to the following Rust code:

```rust
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
`FPDFAnnot_SetStringValue()`, `FPDFText_SetText()`, `FPDFDoc_AddAttachment()`, and
`FPDFAttachment_SetStringValue()`.

The `PdfiumLibraryBindings::get_pdfium_utf16le_bytes_from_str()` and
`PdfiumLibraryBindings::get_string_from_pdfium_utf16le_bytes()` utility functions are provided
for converting to and from `FPDF_WIDESTRING` in your own code.

Certain Pdfium functions take or return C-style integer boolean values, aliased as `FPDF_BOOL`.
The `PdfiumLibraryBindings::TRUE()`, `PdfiumLibraryBindings::FALSE()`,
`PdfiumLibraryBindings::is_true()`, and `PdfiumLibraryBindings::bool_to_pdfium()` utility functions
are provided for converting to and from `FPDF_BOOL` in your own code.

## Development status

The initial focus of this crate was on rendering pages in a PDF file; consequently, `FPDF_*`
functions related to page rendering were prioritised. By 1.0, the functionality of all
`FPDF_*` functions exported by all Pdfium modules will be available, with the exception of certain
functions specific to interactive scripting, user interaction, and printing.

* Releases numbered 0.4.x added support for all page rendering Pdfium functions to `pdfium-render`.
* Releases numbered 0.5.x-0.6.x added support for most read-only Pdfium functions to `pdfium-render`.
* Releases numbered 0.7.x aim to progressively add support for all Pdfium page object creation and editing functions to `pdfium-render`. 
* Releases numbered 0.8.x aim to progressively add support for all other Pdfium editing functions to `pdfium-render`.
* Releases numbered 0.9.x aim to fill any remaining gaps in the high-level interface prior to 1.0.

By version 0.8.0, `pdfium-render` should provide useful coverage for the vast majority of common
use cases, whether rendering existing documents or creating new ones.

There are 368 `FPDF_*` functions in the Pdfium API. As of version 0.7.19, 281 (76%) have
bindings available in `pdfium-render`, with the functionality of roughly three-quarters of these
available via the `pdfium-render` high-level interface.

Some functions and type definitions in the high-level interface have been renamed or revised since
their initial implementation. The initial implementations are still available but are marked as
deprecated. These deprecated items will be removed in release 0.9.0.

If you need a binding to a Pdfium function that is not currently available, just raise an issue.

## Version history

* 0.7.19: adds bindings for `FPDFDoc_*Attachment*()` functions; adds `PdfAttachments` and
  `PdfSignatures` collections to the high-level interface.
* 0.7.18: adds convenience `bindings()` accessor functions to `PdfDocument`, `PdfPage`, `PdfBitmap`,
  `PdfFont`, and various other interfaces, thanks to an excellent contribution from
  <https://github.com/LU15W1R7H>; deprecates `Pdfium::get_bindings()` in favour of
  `Pdfium::bindings()` for consistency. Deprecated items will be removed in release 0.9.0.
* 0.7.17: relaxes some unnecessarily restrictive lifetime bounds in `PdfPageObjectPath`. 
* 0.7.16: adds `PdfPageObjects::create_path_object_bezier()` and `PdfPageObjectPath::new_bezier()`
  convenience functions; corrects some typos in documentation.
* 0.7.15: adds `PdfPageAnnotationCommon::name()`, `PdfPageAnnotationCommon::contents()`,
  `PdfPageAnnotationCommon::author()`, `PdfPageAnnotationCommon::creation_date()`,
  and `PdfPageAnnotationCommon::modification_date()` functions for working with annotations;
  adds `PdfPageText::for_annotation()` and `PdfPageText::chars_for_annotation()` for more easily
  extracting text and characters associated with annotations; adds `examples/annotations.rs` and
  `examples/image_extract.rs`; renames `examples/text.rs` to `examples/text_extract.rs`.
* 0.7.14: fixes a bug in the WASM implementation of `FPDF_StructElement_GetStringAttribute()`;
  pins required version of `image` crate to at least 0.24.0 or later to avoid incompatibility between
  the `image::DynamicImage` trait definitions in 0.23.x and 0.24.x; adds compatibility with web workers
  to the WASM implementation, thanks to an excellent contribution from <https://github.com/NyxCode>.
* 0.7.13: adds transformation and clipping functions to `PdfRenderConfig`; adds bindings for
  `FPDF_RenderPageBitmapWithMatrix()`; deprecates `PdfRenderConfig::rotate_if_portait()`
  in favour of the correctly-spelled `PdfRenderConfig::rotate_if_portrait()`. 
  Deprecated items will be removed in release 0.9.0.
* 0.7.12: adds `PdfPage::render_into_bitmap()` and `PdfPage::render_into_bitmap_with_config()`
  functions for higher performance; deprecates `PdfPage::get_bitmap()` in favour of `PdfPage::render()`;
  deprecates `PdfPage::get_bitmap_with_config()` in favour of `PdfPage::render_with_config()`;
  deprecates `PdfBitmapConfig` in favour of `PdfRenderConfig`; deprecates `PdfBitmap::render()`
  as the function is no longer necessary. Deprecated items will be removed in release 0.9.0.
* 0.7.11: adds the new WASM-specific `PdfBitmap::as_array()` function as a higher performance
  alternative to the cross-platform `PdfBitmap::as_bytes()` function, thanks to an excellent
  contribution from <https://github.com/NyxCode>.
* 0.7.10: corrects some typos in documentation; adds additional constructors to `PdfPageImageObject`
  that apply a specified width and/or height at object creation time.
* 0.7.9: adds retrieval of the list of image filters applied to a `PdfPageImageObject`;
  adds the `PdfColorSpace` enum; adds bindings for the `FPDF_*Signature*()`, `FPDFSignatureObj_*()`,
  and `FPDF_StructTree_*()` functions.
* 0.7.8: adds image support to the `PdfPageImageObject` struct, the `PdfPageObjects::add_image_object()`
  and `PdfPageObjects::create_image_object()` functions, additional convenience functions for
  loading fonts from files and readers to `PdfFont`, and bindings for `FPDF_VIEWERREF_Get*()` functions.
* 0.7.7: adds the `thread_safe` crate feature and the accompanying example in `examples/thread_safe.rs`.
* 0.7.6: adds retrieval of text settings on a character-by-character basis to the `PdfPageText` and
  `PdfPageTextObject` objects; adds `PdfPageTextSegment` and `PdfPageTextChar` structs to the 
  high-level interface; adds retrieval of current transformation settings to all page objects;
  adds the `PdfPageTextObject::scaled_font_size()` function and renames `PdfPageTextObject::font_size()`
  to `PdfPageTextObject::unscaled_font_size()` as these names make clearer the differences between
  scaled and unscaled font sizes in text objects; adds bindings for all remaining `FPDFText_*()` functions.
* 0.7.5: corrects a bug in error handling on Windows. See <https://github.com/ajrcarey/pdfium-render/issues/24>
  for more information.
* 0.7.4: adds the `PdfPageGroupObject::remove_objects_from_page()` function; renamed
  `PdfPageObjects::delete_object()` and `PdfPageObjects::delete_object_at_index()` functions to
  `PdfPageObjects::remove_object()` and `PdfPageObjects::remove_object_at_index()` as these
  names better reflect the underlying operation that occurs.
* 0.7.3: corrects a bug in the implementation of `PdfPages::append()` introduced in 0.7.2. 
* 0.7.2: adds object groups for manipulating and transforming groups of page objects as if they
  were a single object, and the `PdfPages::watermark()` function for applying individualized
  watermarks to any or all pages in a document. Fixes a potential double-free bug in `PdfFont::drop()`.
* 0.7.1: adds path segment creation to the `PdfPagePathObject` object, convenience functions for
  quickly creating rectangles, ellipses, and circles, and the `PdfPageObjects::add_path_object()` function.
* 0.7.0: adds `PdfPermissions` collection, adds document loading and saving support, adds
  initial creation and editing support for documents, pages, and text objects,
  and improves WASM document file handling.
* 0.6.0: fixes some typos in documentation, updates upstream Pdfium WASM package source repository name.
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
  from <https://github.com/inzanez>.
* 0.4.2: bug fixes in `PdfBitmapConfig` implementation.
* 0.4.1: improvements to documentation and READMEs.
* 0.4.0: initial release of minimal page rendering functionality.
