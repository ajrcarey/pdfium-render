# Idiomatic Rust bindings for Pdfium

`pdfium-render` provides an idiomatic high-level Rust interface to Pdfium, the C++ PDF library
used by the Google Chromium project. Pdfium can render pages in PDF files to bitmaps, load, edit,
and extract text and images from existing PDF files, and create new PDF files from scratch.

```rust
    use pdfium_render::prelude::*;

    fn export_pdf_to_jpegs(path: &str, password: Option<&str>) -> Result<(), PdfiumError> {
        // Renders each page in the PDF file at the given path to a separate JPEG file.

        // Bind to a Pdfium library in the same directory as our Rust executable;
        // failing that, fall back to using a Pdfium library provided by the operating system.

        let pdfium = Pdfium::new(
            Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
                .or_else(|_| Pdfium::bind_to_system_library())?,
        );

        // Load the document from the given path...

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
* Document attachment creation and introspection.
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

_Note: Upcoming version 0.8.0 will include a breaking change._ The `PdfDocument::pages()` function,
which currently returns an owned `PdfPages` instance, will be changed so that it returns
an immutable `&PdfPages` reference instead. A new `PdfDocument::pages_mut()` function
will return a mutable `&mut PdfPages` reference. It will no longer be possible to retrieve
an owned `PdfPages` instance. For more information on the motivation behind this change,
see <https://github.com/ajrcarey/pdfium-render/issues/47>.

Version 0.7.27 adjusts the WASM example to take into account upstream packaging changes in the
WASM builds of Pdfium published at <https://github.com/paulocoutinhox/pdfium-lib/releases>,
and adds the `image` crate feature, making the `image` crate an optional dependency instead of
a mandatory one.

Version 0.7.26 fixes a lifetime bug in the `Pdfium::load_pdf_from_bytes()` function,
adds the `sync` crate feature, providing implementations of the `Send` and
`Sync` traits for the `Pdfium` struct that allow it to be shared across threads safely,
and adds implementations of the `std::fmt::Display` and `std::error::Error` traits to the
`PdfiumError` enum, so that it can be used by error handling libraries such as `anyhow`.

Version 0.7.25 adds the `PdfPageAnnotation::objects()` function, allowing inspection of all page
objects attached to an annotation, and the `PdfPageInkAnnotation::objects_mut()` and
`PdfPageStampAnnotation::objects_mut()` functions, allowing adding page objects to, and removing
page objects from, ink and stamp annotations.

Version 0.7.24 adds bindings to Pdfium functions related to individual segments of a Path page object,
and adds the `PdfPagePathObjectSegments` and `PdfFontGlyphs` collections to the high-level interface,
along with functions for retrieving path segments for individual font glyphs and page path objects.

## Binding to Pdfium

`pdfium-render` does not include Pdfium itself. You have several options:

* Bind to a dynamically-built Pdfium library provided by the operating system.
* Bind to a dynamically-built Pdfium library packaged alongside your Rust executable.
* Bind to a statically-built Pdfium library linked to your executable at compile time.

When compiling to WASM, packaging an external build of Pdfium as a separate WASM module is essential.

## Dynamic linking

Binding to a pre-built Pdfium dynamic library at runtime is the simplest option. On Android, a pre-built
`libpdfium.so` is packaged as part of the operating system (although recent versions of Android no
longer permit user applications to access it); alternatively, you can package a dynamic library
appropriate for your operating system alongside your Rust executable.

Pre-built Pdfium dynamic libraries suitable for runtime binding are available from several sources:

* Native (i.e. non-WASM) builds of Pdfium for all major platforms: <https://github.com/bblanchon/pdfium-binaries/releases>
* Android, iOS, macOS, and WASM builds of Pdfium: <https://github.com/paulocoutinhox/pdfium-lib/releases>

If you are compiling a native (i.e. non-WASM) build, and you place an appropriate Pdfium library
in the same folder as your compiled application, then binding to it at runtime is as simple as:

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

Depending on how your Pdfium library was built, you may need to also link against a C++ standard library.
To link against the GNU C++ standard library (`libstdc++`), use the optional `libstdc++` feature.
`pdfium-render` will pass the following additional flag to `cargo`:

```rust
    cargo:rustc-link-lib=dylib=stdc++
```

To link against the LLVM C++ standard library (`libc++`), use the optional `libc++` feature.
`pdfium-render` will pass the following additional flag to `cargo`:

```rust
    cargo:rustc-link-lib=dylib=c++
```

Alternatively, use the `link-cplusplus` crate to link against a C++ standard library. `link-cplusplus`
offers more options for deciding which standard library should be selected, including automatically
selecting the build platform's installed default.

`pdfium-render` will not build Pdfium for you; you must build Pdfium yourself, or source a
pre-built static archive from elsewhere. For an overview of the build process, including a sample
build script, see <https://github.com/ajrcarey/pdfium-render/issues/53>.

## Compiling to WASM

See <https://github.com/ajrcarey/pdfium-render/tree/master/examples> for a full example that shows
how to bundle a Rust application using `pdfium-render` alongside a pre-built Pdfium WASM module for
inspection and rendering of PDF files in a web browser.

Certain functions that access the file system are not available when compiling to WASM. In all cases,
browser-specific alternatives are provided, as detailed at the link above.

At the time of writing, the WASM builds of Pdfium at <https://github.com/bblanchon/pdfium-binaries/releases>
are compiled with a non-growable WASM heap memory allocator. This means that attempting to open
a PDF document longer than just a few pages will result in an unrecoverable out of memory error.
The WASM builds of Pdfium at <https://github.com/paulocoutinhox/pdfium-lib/releases> are recommended
as they do not have this problem.

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
* `image`: controls whether the `image` crate should be used by `pdfium-render` to provide page 
  rendering functionality. This lets projects that do not need to render pages or page objects
  to bitmaps to avoid the need to compile the `image` crate into their binaries.
* `static`: enables binding to a statically-linked build of Pdfium. See the "Static linking" section above.
* `libstdc++`: links against the GNU C++ standard library when compiling. Requires the `static` feature. See the "Static linking" section above.
* `libc++`: links against the LLVM C++ standard library when compiling. Requires the `static` feature. See the "Static linking" section above.
* `thread_safe`: wraps access to Pdfium behind a mutex to ensure thread-safe access to Pdfium.
  See the "Multithreading" section above.
* `sync`: provides an implementation of the `Send` and `Sync` traits for the Pdfium struct. This allows
  a `Pdfium` instance to be shared across threads. Requires the `thread_safe` feature.

The `image`, `thread_safe`, and `sync` features are enabled by default.
All other features are disabled by default.

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
`FPDFText_SetText()`, `FPDFText_FindStart()`, `FPDFDoc_AddAttachment()`, `FPDFAnnot_SetStringValue()`,
and `FPDFAttachment_SetStringValue()`.

The `PdfiumLibraryBindings::get_pdfium_utf16le_bytes_from_str()` and
`PdfiumLibraryBindings::get_string_from_pdfium_utf16le_bytes()` utility functions are provided
for converting to and from `FPDF_WIDESTRING` in your own code.

Some Pdfium functions return classic C-style integer boolean values, aliased as `FPDF_BOOL`.
The `PdfiumLibraryBindings::TRUE()`, `PdfiumLibraryBindings::FALSE()`,
`PdfiumLibraryBindings::is_true()`, and `PdfiumLibraryBindings::bool_to_pdfium()` utility functions
are provided for converting to and from `FPDF_BOOL` in your own code.

Image pixel data in Pdfium is encoded in either three-channel BGR or four-channel BGRA.
The `PdfiumLibraryBindings::bgr_to_rgba()`, `PdfiumLibraryBindings::bgra_to_rgba()`,
`PdfiumLibraryBindings::rgb_to_bgra()`, and `PdfiumLibraryBindings::rgba_to_bgra()` utility functions
are provided for converting between RGB and BGR image data in your own code. 

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

There are 368 `FPDF_*` functions in the Pdfium API. As of version 0.7.27, 316 (86%) have
bindings available in `PdfiumLibraryBindings`, with the functionality of the vast majority of
these exposed through the `pdfium-render` high-level interface.

Some functions and type definitions in the high-level interface have been renamed or revised since
their initial implementation. The initial implementations are still available but are marked as
deprecated. These deprecated items will be removed in release 0.9.0.

If you need a binding to a Pdfium function that is not currently available, just raise an issue.

## Version history

* 0.7.27: adjusts `examples/index.html` to take into account upstream packaging changes in the
  WASM builds of Pdfium published at <https://github.com/paulocoutinhox/pdfium-lib/releases>;
  adds the `image` crate feature.
* 0.7.26: adds `sync` default crate feature providing `Send` and `Sync` implementations for `Pdfium`
  struct; adds `Display` and `Error` trait implementations to `PdfiumError` for `anyhow` compatibility;
  adjusts WASM example to account for upstream changes in Emscripten packaging of Pdfium WASM builds;
  corrects a lifetime problem in `Pdfium::load_pdf_from_bytes()` and deprecates `Pdfium::load_pdf_from_bytes()`
  in favour of `Pdfium::load_pdf_from_byte_slice()` and `Pdfium::load_pdf_from_byte_vec()`.
  Deprecated items will be removed in release 0.9.0.
* 0.7.25: adds the `PdfPageAnnotationObjects` collection and the `PdfPageAnnotation::objects()`,
  `PdfPageInkAnnotation::objects_mut()`, and `PdfPageStampAnnotation::objects_mut()` functions
  to the high-level interface.
* 0.7.24: adds bindings for `FPDFClipPath_CountPathSegments()`, `FPDFClipPath_GetPathSegment()`,
  `FPDFPath_CountSegments()`, `FPDFPath_GetPathSegment()`, and `FPDFPathSegment_*()` functions;
  adds `PdfFontGlyphs` and `PdfPagePathObjectSegments` collections to the high-level interface,
  along with accessor functions in `PdfFont` and `PdfPagePathObject`; adds the `PdfPathSegments` trait;
  introduces some infrastructure necessary for the future implementation of a `PdfClipPath` object;
  adds `PdfPages::first()`, `PdfPages::last()`, and `PdfPage::fonts()` convenience functions.
* 0.7.23: removes some unnecessary mutable bindings in `PdfBitmap`; uses `#[cfg(doc)]` declarations
  to ensure `cargo doc` generates documentation for all functionality, irrespective of the platform.
* 0.7.22: attempts to work around two problems in Pdfium's bitmap generation when retrieving
  processed renderings of page image objects. See <https://github.com/ajrcarey/pdfium-render/issues/52>
  for more information.
* 0.7.21: adds bindings for `FPDF_GetPageAAction()`, `FPDF_GetFileIdentifier()`, and all remaining
  `FPDFDest_*()` and `FPDFLink_*()` functions; adds `PdfAttachment::len()` and
  `PdfAttachment::is_empty()` convenience functions; adds `libstdc++` and `libc++` crate features;
  adds color conversion functions to `PdfiumLibraryBindings`; corrects bugs in color conversion
  when working with `PdfPageImageObject`, as detailed in <https://github.com/ajrcarey/pdfium-render/issues/50>;
  fixes a bug in the WASM implementation of `FPDFAnnot_GetAttachmentPoints()`; corrects
  some small typos in examples.
* 0.7.20: adds bindings for `FPDFPage_*Thumbnail*()`, `FPDFLink_*()`, and `FPDFText_Find*()` functions;
  adds `PdfAttachments::create_attachment_from_bytes()`, `PdfAttachments::create_attachment_from_file()`,
  `PdfAttachments::create_attachment_from_reader()`, `PdfAttachments::create_attachment_from_fetch()`,  
  `PdfAttachments::create_attachment_from_blob()`, `PdfAttachments::delete_at_index()`,
  `PdfAttachment::save_to_writer()`, `PdfAttachment::save_to_file()`, `PdfAttachment::save_to_blob()`,
  `PdfPage::has_embedded_thumbnail()`, `PdfPage::embedded_thumbnail()`, and `PdfPage::boundaries_mut()`
  functions to the high-level interface; renames `PdfAttachment::bytes()` function introduced in 0.7.19
  to `PdfAttachment::save_to_bytes()`.
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
