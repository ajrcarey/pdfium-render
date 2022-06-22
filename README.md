# Idiomatic Rust bindings for Pdfium

`pdfium-render` provides an idiomatic high-level Rust interface to Pdfium, the C++ PDF library
used by the Google Chromium project.

```
    use pdfium_render::prelude::*;

    fn export_pdf_to_jpegs(path: &str, password: Option<&str>) -> Result<(), PdfiumError> {
        // Renders each page in the given test PDF file to a separate JPEG file.

        // Bind to a Pdfium library in the same directory as our application;
        // failing that, fall back to using a Pdfium library provided by the operating system.

        let pdfium = Pdfium::new(
            Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
                .or_else(|_| Pdfium::bind_to_system_library())?,
        );

        // Open the PDF document...

        let document = pdfium.load_pdf_from_file(path, password)?;

        // ... set rendering options that will apply to all pages...

        let bitmap_render_config = PdfBitmapConfig::new()
            .set_target_width(2000)
            .set_maximum_height(2000)
            .rotate_if_landscape(PdfBitmapRotation::Degrees90, true);

        // ... then render each page to a bitmap image, saving each image to a JPEG file.

        for (index, page) in document.pages().iter().enumerate() {
            page.get_bitmap_with_config(&bitmap_render_config)?
                .as_image() // Renders this page to an Image::DynamicImage...
                .as_rgba8() // ... then converts it to an Image::Image
                .ok_or(PdfiumError::ImageError)?
                .save_with_format(
                    format!("test-page-{}.jpg", index), 
                    image::ImageFormat::Jpeg
                )
                .map_err(|_| PdfiumError::ImageError)?;
        }

        Ok(())
    }
```

`pdfium-render` binds to a Pdfium library at run-time, allowing for run-time selection of
system-provided or bundled Pdfium libraries and providing idiomatic Rust error handling in
situations where a Pdfium library is not available. A key advantage of binding to Pdfium at run-time
rather than compile-time is that a Rust application using `pdfium-render` can be compiled to WASM
for running in a browser alongside a WASM-packaged build of Pdfium.

Pdfium itself is architected as a set of separate modules, each covering a different aspect of
PDF document creation, rendering, and editing. `pdfium-render` aims to ultimately provide bindings
to all non-interactive functions exposed by all Pdfium modules, including document creation and
editing functions. This is a work in progress. 

## Examples

Short, commented examples that demonstrate all the major Pdfium document handling features are
available at <https://github.com/ajrcarey/pdfium-render/tree/master/examples>. These examples cover:

* Rendering pages to bitmaps.
* Text extraction.
* Page object introspection.
* Creation of new documents.
* Document concatenation.
* Multi-page tiled output.
* Watermarking.
* Compiling to WASM.

## What's new

Version 0.7.0 is a substantial release that introduces the first set of document editing functions
into `pdfium-render`. This release includes the following improvements to the high-level interface:

* Adds loading and saving of PDF documents from standard Rust readers and writers.
* Adds creation of new documents.
* Adds adding and deleting of pages to documents, and importing of pages from one document into another.
* Adds adding and deleting of page objects to pages, and importing of page objects from one page into another.
* Adds additional properties and functions to all page objects, including setting and reading of
colors, strokes, fills, and blend modes, and object positioning, rotation, scaling, and skewing.
* Adds the `PdfPermissions` collection, allowing reading of security handlers and permissions for a document.
* Adds additional convenience functions for loading and saving documents when compiling to WASM.

With this release, it is now possible to create a new PDF document from scratch, add pages to it
(either by creating them from scratch, or by importing them from other documents), add new text objects
to those pages, and output the newly created document to a file.

The initial editing focus has been on providing creation and editing support for text objects.
Later 0.7.x releases will add similar support for creating and editing images, paths, and the other
types of page objects supported by Pdfium.

Version 0.7.1 adds path support, construction of both straight and curved path segments, and
convenience functions to easily create filled and stroked rectangles, ellipses, and circles.

Version 0.7.2 adds object groups for manipulating and transforming groups of page objects as if they
were a single object, and the `PdfPages::watermark()` function for applying individualized
watermarks to any or all pages in a document.

Version 0.7.6 adds additional properties to the `PdfPageText` and `PdfPageObject` objects.
 
## Porting existing Pdfium code from other languages

The high-level idiomatic Rust interface provided by `pdfium-render` is entirely optional;
the idiomatic interface is built on top of raw FFI bindings defined in the `PdfiumLibraryBindings`
trait, and it is completely feasible to simply use these raw FFI bindings directly if you prefer.
This makes porting existing code that calls `FPDF_*` functions trivial, while still gaining the
benefits of late binding and WASM compatibility. For instance, the following code snippet
(taken from a C++ sample):

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
* WASM builds of Pdfium: <https://github.com/paulocoutinhox/pdfium-lib/releases>

At the time of writing, the WASM builds at <https://github.com/bblanchon/pdfium-binaries/releases>
are compiled with a non-growable WASM heap memory allocator. This means that attempting to open
a PDF document longer than just a few pages will result in an unrecoverable out of memory error.
The WASM builds at <https://github.com/paulocoutinhox/pdfium-lib/releases> are recommended as they
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

The following additional function is provided during rendering:

* The `PdfBitmap::as_image_data()` function renders directly to a Javascript `ImageData` object,
ready to display in an HTML `<canvas>` element.

## Optional features

This crate provides the following optional features:

* `bindings`: uses `cbindgen` to generate Rust bindings to the Pdfium functions defined in the
  `include/*.h` files each time `cargo build` is run. If `cbindgen` or any of its dependencies
  are not available then the build will fail.
* `static`: enables binding to a statically-linked build of Pdfium. See the "Static linking" section above.

Neither feature is enabled by default.

## Development status

The initial focus of this crate was on rendering pages in a PDF file; consequently, `FPDF_*`
functions related to page rendering were prioritised. By 1.0, the functionality of all
`FPDF_*` functions exported by all Pdfium modules will be available, with the exception of certain
functions specific to interactive scripting, user interaction, and printing.

* Releases numbered 0.4.x added support for all page rendering Pdfium functions to `pdfium-render`.
* Releases numbered 0.5.x-0.6.x added support for most read-only Pdfium functions to `pdfium-render`.
* Releases numbered 0.7.x aim to progressively add support for all Pdfium page object creation and editing functions to `pdfium-render`. 
* Releases numbered 0.8.x aim to progressively add support for all other Pdfium editing functions to `pdfium-render`.
* Releases numbered 0.9.x aim to fill any remaining gaps in the high-level interface prior to 1.0.0.

By version 0.8.0, `pdfium-render` should provide useful coverage for the vast majority of common
use cases, whether rendering existing documents or creating new ones.

There are 368 `FPDF_*` functions in the Pdfium API. As of version 0.7.6, 238 (65%) have
bindings available in `pdfium-render`, with the functionality of roughly three-quarters of these
available via the `pdfium-render` high-level interface.

If you need a binding to a Pdfium function that is not currently available, just raise an issue.

## Version history

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
