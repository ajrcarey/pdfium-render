# Idiomatic Rust bindings for Pdfium

`pdfium-render` provides an idiomatic high-level Rust interface to Pdfium, the C++ PDF library used by the Google Chromium project. Pdfium can render pages in PDF files to bitmaps, load, edit, and extract text and images from existing PDF files, and create new PDF files from scratch.

```rust
    use pdfium_render::prelude::*;

    fn export_pdf_to_jpegs(path: &impl AsRef<Path>, password: Option<&str>) -> Result<(), PdfiumError> {
        // Renders each page in the PDF file at the given path to a separate JPEG file.

        // Bind to a Pdfium library in the same directory as our Rust executable.
        // See the "Dynamic linking" section below.

        let pdfium = Pdfium::default();

        // Load the document from the given path...

        let document = pdfium.load_pdf_from_file(path, password)?;

        // ... set rendering options that will be applied to all pages...

        let render_config = PdfRenderConfig::new()
            .set_target_width(2000)
            .set_maximum_height(2000)
            .rotate_if_landscape(PdfPageRenderRotation::Degrees90, true);

        // ... then render each page to a bitmap image, saving each image to a JPEG file.

        for (index, page) in document.pages().iter().enumerate() {
            page.render_with_config(&render_config)?
                .as_image() // Renders this page to an image::DynamicImage...
                .into_rgb8() // ... then converts it to an image::Image...
                .save_with_format(
                    format!("test-page-{}.jpg", index), 
                    image::ImageFormat::Jpeg
                ) // ... and saves it to a file.
                .map_err(|_| PdfiumError::ImageError)?;
        }

        Ok(())
    }
```

`pdfium-render` binds to a Pdfium library at run-time, allowing for flexible selection of system-provided or bundled Pdfium libraries and providing idiomatic Rust error handling in situations where a Pdfium library is not available. A key advantage of binding to Pdfium at run-time rather than compile-time is that a Rust application using `pdfium-render` can be compiled to WASM for running in a browser alongside a WASM-packaged build of Pdfium.

## Examples

Short, commented examples that demonstrate all the major Pdfium document handling features are available at <https://github.com/ajrcarey/pdfium-render/tree/master/examples>. These examples demonstrate:

* Rendering pages, and portions of pages, to bitmaps.
* Text and image extraction.
* Form field introspection and filling of form fields.
* Document signature introspection.
* Document attachment creation and introspection.
* Document concatenation.
* Page object introspection.
* Page annotation introspection.
* Page link introspection.
* Creation of new documents and new pages.
* Creation of page objects for text, paths, and bitmaps.
* Page object transformation.
* Multi-page tiled rendering.
* Watermarking.
* Thread safety.
* Compiling to WASM.

## What's new

_Note: upcoming release 0.9.0 will remove all deprecated items. For a complete list of deprecated
items, see <https://github.com/ajrcarey/pdfium-render/issues/36>._

Release 0.8.27 adds a new `axum_once_cell` example demonstrating how to use Pdfium safely across asynchronous tasks with Axum, thanks to an excellent contribution from <https://github.com/danwritecode>, and fixes two bugs related to memory safety in the WASM bindings implementation, thanks to an excellent contribution from <https://github.com/samsieber>. The first bug affected certain UTF-16 string handling operations; the second bug could result in data corruption when working with raw bitmap pixel buffers.

Release 0.8.26 relaxes the minimum supported Rust version to 1.61 based on user feedback, increments the `pdfium_latest` feature to `pdfium_6721` to match new Pdfium release 6721 at <https://github.com/bblanchon/pdfium-binaries>, adds new crate features `image_025`, `image_024`, and `image_023` to allow explicitly pinning the version of `image` that should be used by `pdfium-render`, sets `image` to `image_025`, and adjusts bookmark traversal so that bookmarks are returned in a more natural order, thanks to an excellent contribution from <https://github.com/mlaiosa>.

Release 0.8.25 establishes a minimum supported Rust version of 1.60 for `pdfium-render`, increments the `pdfium_latest` feature to `pdfium_6666` to match new Pdfium release 6666 at <https://github.com/bblanchon/pdfium-binaries>, adds new crate features `pdfium_use_skia`, `pdfium_use_win32`, `pdfium_enable_xfa`, and `pdfium_enable_v8` to make available certain Pdfium functions that require Pdfium to be built with specific compile-time flags, and adds bindings for all remaining `FPDF_*` functions in the Pdfium API to `PdfiumLibraryBindings`, an important milestone leading up to release 0.9.0.

Release 0.8.24 fixes a bug in certain string handling operations in the WASM bindings implementation, and introduces the ability to control the version of the Pdfium API used by `pdfium-render`. By default `pdfium-render` uses the latest released version of the Pdfium API, potentially requiring you to upgrade your Pdfium library if the latest release contains breaking changes. This can be inconvenient! To explicitly use an older API version, select one of the crate's Pdfium version feature flags when taking `pdfium-render` as a dependency in your project's `Cargo.toml`. See the "Crate features" section below for more information.

## Binding to Pdfium

`pdfium-render` does not include Pdfium itself. You have several options:

* Bind to a dynamically-built Pdfium library provided by the operating system.
* Bind to a dynamically-built Pdfium library packaged alongside your Rust executable.
* Bind to a statically-built Pdfium library linked to your executable at compile time.

When compiling to WASM, packaging an external build of Pdfium as a separate WASM module is essential.

## Dynamic linking

Binding to a pre-built Pdfium dynamic library at runtime is the simplest option. On Android, a pre-built `libpdfium.so` is packaged as part of the operating system (although recent versions of Android no longer permit user applications to access it); alternatively, you can package a dynamic library appropriate for your operating system alongside your Rust executable.

Pre-built Pdfium dynamic libraries suitable for runtime binding are available from several sources:

* Native (i.e. non-WASM) builds of Pdfium for all major platforms: <https://github.com/bblanchon/pdfium-binaries/releases>
* Android, iOS, macOS, and WASM builds of Pdfium: <https://github.com/paulocoutinhox/pdfium-lib/releases>

If you are compiling a native (i.e. non-WASM) build, and you place an appropriate Pdfium library in the same folder as your compiled application, then binding to it at runtime is as simple as:

```rust
    use pdfium_render::prelude::*;

    let pdfium = Pdfium::new(
        Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./")).unwrap()
    );
```

A common pattern used in the examples at <https://github.com/ajrcarey/pdfium-render/tree/master/examples> is to first attempt to bind to a Pdfium library in the same folder as the compiled example, and attempt to fall back to a system-provided library if that fails:

```rust
    use pdfium_render::prelude::*;

    let pdfium = Pdfium::new(
        Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
            .or_else(|_| Pdfium::bind_to_system_library())
            .unwrap() // Or use the ? unwrapping operator to pass any error up to the caller
    );
```

This pattern is used to provide an implementation of the `Default` trait, so the above can be written more simply as:

```rust
    use pdfium_render::prelude::*;

    let pdfium = Pdfium::default();
```

## Static linking

The `static` crate feature offers an alternative to dynamic linking if you prefer to link Pdfium directly into your executable at compile time. This enables the `Pdfium::bind_to_statically_linked_library()` function which binds directly to the Pdfium functions compiled into your executable:

```rust
    use pdfium_render::prelude::*;

    let pdfium = Pdfium::new(Pdfium::bind_to_statically_linked_library().unwrap());
```

As a convenience, `pdfium-render` can instruct `cargo` to link to either a dynamically-built or a statically-built Pdfium library for you. To link to a dynamically-built library, set the `PDFIUM_DYNAMIC_LIB_PATH` environment variable when you run `cargo build`, like so:

```rust
    PDFIUM_DYNAMIC_LIB_PATH="/path/containing/your/dynamic/pdfium/library" cargo build
```

`pdfium-render` will pass the following flags to `cargo`:

```rust
    cargo:rustc-link-lib=dylib=pdfium
    cargo:rustc-link-search=native=$PDFIUM_DYNAMIC_LIB_PATH
```

To link to a statically-built library, set the path to the directory containing your library using the `PDFIUM_STATIC_LIB_PATH` environment variable when you run `cargo build`, like so:

```rust
    PDFIUM_STATIC_LIB_PATH="/path/containing/your/static/pdfium/library" cargo build
```

`pdfium-render` will pass the following flags to `cargo`:

```rust
    cargo:rustc-link-lib=static=pdfium
    cargo:rustc-link-search=native=$PDFIUM_STATIC_LIB_PATH
```

These two environment variables save you writing a custom `build.rs` yourself. If you have your own build pipeline that links Pdfium statically into your executable, simply leave these environment variables unset.

Note that the path you set in either `PDFIUM_DYNAMIC_LIB_PATH` or `PDFIUM_STATIC_LIB_PATH` should not include the filename of the library itself; it should just be the path of the containing directory. You must make sure your library is named in the appropriate way for your target platform (`libpdfium.so` or `libpdfium.a` on Linux and macOS, for example) in order for the Rust compiler to locate it.

Depending on how your Pdfium library was built, you may need to also link against a C++ standard library. To link against the GNU C++ standard library (`libstdc++`), use the optional `libstdc++` feature. `pdfium-render` will pass the following additional flag to `cargo`:

```rust
    cargo:rustc-link-lib=dylib=stdc++
```

To link against the LLVM C++ standard library (`libc++`), use the optional `libc++` feature. `pdfium-render` will pass the following additional flag to `cargo`:

```rust
    cargo:rustc-link-lib=dylib=c++
```

Alternatively, use the `link-cplusplus` crate to link against a C++ standard library. `link-cplusplus` offers more options for deciding which standard library should be selected, including automatically selecting the build platform's installed default.

`pdfium-render` will not build Pdfium for you; you must build Pdfium yourself, source a pre-built static archive from elsewhere, or use a dynamically built library downloaded from one of the sources listed above in the "Dynamic linking" section. If you wish to build a static library yourself, an overview of the build process - including a sample build script - is available at <https://github.com/ajrcarey/pdfium-render/issues/53>.

## Compiling to WASM

See <https://github.com/ajrcarey/pdfium-render/tree/master/examples> for a full example that shows
how to bundle a Rust application using `pdfium-render` alongside a pre-built Pdfium WASM module for
inspection and rendering of PDF files in a web browser.

Certain functions that access the file system are not available when compiling to WASM. In all cases,
browser-specific alternatives are provided, as detailed at the link above.

At the time of writing, the WASM builds of Pdfium at <https://github.com/bblanchon/pdfium-binaries/releases> are compiled with a non-growable WASM heap memory allocator. This means that attempting to open a PDF document longer than just a few pages will result in an unrecoverable out of memory error. The WASM builds of Pdfium at <https://github.com/paulocoutinhox/pdfium-lib/releases> are recommended as they do not have this problem.

## Multi-threading

Pdfium makes no guarantees about thread safety and should be assumed _not_ to be thread safe. The Pdfium authors specifically recommend that parallel processing, not multi-threading, be used to process multiple documents simultaneously.

`pdfium-render` achieves thread safety by locking access to Pdfium behind a mutex; each thread must acquire exclusive access to this mutex in order to make any call to Pdfium. This has the effect of sequencing all calls to Pdfium as if they were single-threaded, even when using `pdfium-render` from multiple threads. This approach offers no performance benefit, but it ensures that Pdfium will not crash when running as part of a multi-threaded application.

An example of safely using `pdfium-render` as part of a multi-threaded parallel iterator is available at <https://github.com/ajrcarey/pdfium-render/tree/master/examples>.

## Crate features

This crate provides the following optional features:

* `bindings`: uses `cbindgen` to generate Rust bindings to the Pdfium functions defined in the `include/*.h` files each time `cargo build` is run. If `cbindgen` or any of its dependencies are not available then the build will fail.
* `image`: controls whether the `image` crate should be used by `pdfium-render` to provide page and page object rendering functionality. Projects that do not require page or page object rendering can disable this feature to avoid compiling the `image` crate into their binaries. It is possible to control the specific version of `image` that will be used by `pdfium-render`; see the "Crate features for selecting `image` versions" section below.
* `libstdc++`: links against the GNU C++ standard library when compiling. Requires the `static` feature. See the "Static linking" section above.
* `libc++`: links against the LLVM C++ standard library when compiling. Requires the `static` feature. See the "Static linking" section above.
* `static`: enables binding to a statically-linked build of Pdfium. See the "Static linking" section above.
* `sync`: provides implementations of the `Send` and `Sync` traits for the `Pdfium` and `PdfDocument` structs. This is useful for creating static instances that can be used with `lazy_static` or `once_cell`, although those instances are not guaranteed to be thread-safe. Use entirely at your own risk. Requires the `thread_safe` feature.
* `thread_safe`: wraps access to Pdfium behind a mutex to ensure thread-safe access to Pdfium. See the "Multithreading" section above.

#### Crate features for selecting `image` versions

Release 0.8.26 introduced new features to explicitly control the version of the `image` crate used by `pdfium-render`:

* `image`: uses the latest version of the `image` crate. This is currently `image_025`.
* `image_025`: uses `image` crate version `0.25`.
* `image_024`: uses `image` crate version `0.24`.
* `image_023`: uses `image` crate version `0.23`.

#### Crate features for selecting Pdfium API versions

Release 0.8.24 introduced new features to explicitly control the version of the Pdfium API used by `pdfium-render`:

* `pdfium_future`: binds `PdfiumLibraryBindings` to the latest published Pdfium API at <https://pdfium.googlesource.com/pdfium/+/refs/heads/main/public>, irrespective of whether those changes have been built into a release at <https://github.com/bblanchon/pdfium-binaries/releases>. Useful for testing unreleased changes.
* `pdfium_latest`: binds `PdfiumLibraryBindings` to the latest released build of Pdfium at <https://github.com/bblanchon/pdfium-binaries/releases> supported by `pdfium-render`. This is currently `pdfium_6721`.
* `pdfium_6721`, `pdfium_6666`, `pdfium_6611`, `pdfium_6569`, `pdfium_6555`, `pdfium_6490`, `pdfium_6406`, `pdfium_6337`, `pdfium_6295`, `pdfium_6259`, `pdfium_6164`, `pdfium_6124`, `pdfium_6110`, `pdfium_6084`, `pdfium_6043`, `pdfium_6015`, `pdfium_5961`: binds `PdfiumLibraryBindings` to the specified version of the Pdfium API.

A small number of functions in the Pdfium API are gated behind compile-time flags when compiling Pdfium. `pdfium-render` release 0.8.25 introduced new crate features to control whether these functions are included in the `PdfiumLibraryBindings` trait:

* `pdfium_use_skia`: includes SKIA-specific functions in `PdfiumLibraryBindings`. You must bind to a build of Pdfium that was compiled with the `PDF_USE_SKIA` flag, or using these functions will fail.
* `pdfium_use_win32`: includes Windows-specific functions in `PdfiumLibraryBindings`. You must bind to a build of Pdfium that was compiled with the `_WIN32` flag, or using these functions will fail. This feature requires the `windows` crate. This feature is not supported when compiling to WASM.
* `pdfium_enable_xfa`: includes XFA-specific functions in `PdfiumLibraryBindings`. You must bind to a build of Pdfium that was compiled with the `PDF_ENABLE_XFA` flag, or using these functions will fail.
* `pdfium_enable_v8`: includes V8-specific functions in `PdfiumLibraryBindings`. You must bind to a build of Pdfium that was compiled with the `PDF_ENABLE_V8` flag, or using these functions will fail. This feature is not supported when compiling to WASM.

#### Default features

The `image`, `thread_safe`, and `pdfium_latest` features are enabled by default. All other features are disabled by default.

## Minimum supported Rust version

With the `image` feature enabled, the minimum supported Rust version of `pdfium-render` will align with the minimum supported Rust version of `image` (at the time of writing, Rust 1.79 for `image` version 0.25). With the `image` feature disabled, the minimum supported Rust version of `pdfium-render` is 1.61.

## Porting existing Pdfium code from other languages

The high-level idiomatic Rust interface provided by `pdfium-render` is built on top of raw FFI bindings to the Pdfium API defined in the `PdfiumLibraryBindings` trait. It is completely feasible to use these raw FFI bindings directly if you wish, making porting existing code that uses the Pdfium API trivial while still gaining the benefits of late binding and WASM compatibility. For instance, the following code snippet (taken from a C++ sample):

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
    let pdfium = Pdfium::default();
    let bindings = pdfium.bindings();
    let test_doc = "test.pdf";

    bindings.FPDF_InitLibrary();
    let doc = bindings.FPDF_LoadDocument(test_doc, None);
    // ... do something with doc
    bindings.FPDF_CloseDocument(doc);
    bindings.FPDF_DestroyLibrary();
```

Pdfium's API uses three different string types: classic C-style null-terminated char arrays,
UTF-8 byte arrays, and a UTF-16LE byte array type named `FPDF_WIDESTRING`. For functions that take a C-style string or a UTF-8 byte array, `pdfium-render`'s binding will take the standard Rust `&str` type. For functions that take an `FPDF_WIDESTRING`, `pdfium-render` exposes two functions: the vanilla `FPDF_*()` function that takes an `FPDF_WIDESTRING`, and an additional `FPDF_*_str()` helper function that takes a standard Rust `&str` and converts it internally to an `FPDF_WIDESTRING` before calling Pdfium. Examples of functions with additional `_str()` helpers include `FPDFBookmark_Find()`, `FPDFText_SetText()`, `FPDFText_FindStart()`, `FPDFDoc_AddAttachment()`, `FPDFAnnot_SetStringValue()`, and `FPDFAttachment_SetStringValue()`.

The `PdfiumLibraryBindings::get_pdfium_utf16le_bytes_from_str()` and `PdfiumLibraryBindings::get_string_from_pdfium_utf16le_bytes()` utility functions are provided for converting to and from `FPDF_WIDESTRING` in your own code.

Some Pdfium functions return classic C-style integer boolean values, aliased as `FPDF_BOOL`. The `PdfiumLibraryBindings::TRUE()`, `PdfiumLibraryBindings::FALSE()`, `PdfiumLibraryBindings::is_true()`, `PdfiumLibraryBindings::to_result()`, and `PdfiumLibraryBindings::bool_to_pdfium()` utility functions are provided for converting to and from `FPDF_BOOL` in your own code.

Image pixel data in Pdfium is encoded in either three-channel BGR or four-channel BGRA. The `PdfiumLibraryBindings::bgr_to_rgba()`, `PdfiumLibraryBindings::bgra_to_rgba()`, `PdfiumLibraryBindings::rgb_to_bgra()`, and `PdfiumLibraryBindings::rgba_to_bgra()` utility functions are provided for converting between RGB and BGR image data in your own code. 

## Development status

As at Pdfium release `pdfium_6721` there are 426 `FPDF_*` functions in the Pdfium API. Bindings to these functions are available in the `PdfiumLibraryBindings` trait.

The initial focus of this crate was on rendering pages in a PDF file; consequently, high-level implementations of `FPDF_*` functions related to page rendering were prioritised. By 1.0, the functionality of all `FPDF_*` functions exported by all Pdfium modules will be available, with the exception of certain functions specific to interactive scripting, user interaction, and printing.

* Releases numbered 0.4.x added support for basic page rendering Pdfium functions to `pdfium-render`.
* Releases numbered 0.5.x-0.6.x added support for most read-only Pdfium functions to `pdfium-render`.
* Releases numbered 0.7.x added support for most Pdfium page object creation and editing functions to `pdfium-render`. 
* Releases numbered 0.8.x aim to progressively add support for all remaining Pdfium editing functions to `pdfium-render`.
* Releases numbered 0.9.x aim to fill any remaining gaps in the high-level interface prior to 1.0.

Some functions and type definitions have been renamed or revised since their initial implementations.
The initial implementations are still available but are marked as deprecated. These deprecated items
will be removed in release 0.9.0.

## Version history

* 0.8.27: adds new `axum_once_cell` example; fixes a WASM-specific bug in string termination detection when copying `FPDF_WIDESTRING` buffers to Pdfium's WASM address space; deprecates the memory-unsafe WASM implementation of `FPDFBitmap_GetBuffer` in favour of `FPDFBitmap_GetBuffer_as_vec` and `FPDFBitmap_GetBuffer_as_array` (renamed from `FPDFBitmap_GetArray`); establishes upper bound on `wasm-bindgen` dependency to avoid a build failure during Github workflow automated build that was introduced in `wasm-bindgen-macros` versions 0.2.96 and later, as described at <https://github.com/ajrcarey/pdfium-render/issues/177>; completes expansion of all `FPDF_*` doc comments in `PdfiumLibraryBindings` trait. Deprecated items will be removed in release 0.9.0.
* 0.8.26: relaxes version constraint on `chrono` dependency based on user feedback, relaxing the minimum supported Rust version to 1.61; increments the `pdfium_latest` feature to `pdfium_6721` to match new Pdfium release 6721 at <https://github.com/bblanchon/pdfium-binaries>; synchronises `pdfium_future` feature against current Google mainline; adds new crate features `image_025`, `image_024`, and `image_023` to allow explicitly pinning the version of `image` that should be used by `pdfium-render`, sets `image` to `image_025`; adjusts bookmark traversal so that bookmarks are returned in a more natural order, thanks to an excellent contribution from <https://github.com/mlaiosa>.
* 0.8.25: adds bindings for all outstanding Pdfium API functions, including `FPDFAnnot_GetFormAdditionalActionJavaScript()`, `FPDFBitmap_Create()`, `FPDFAnnot_GetFormFieldAlternateName()`, `FPDFAnnot_GetFileAttachment()`, `FPDFAnnot_AddFileAttachment()`, `FPDFAvail_Create()`, `FPDFAvail_Destroy()`, `FPDFAvail_IsDocAvail()`, `FPDFAvail_GetDocument()`, `FPDFAvail_GetFirstPageNum()`, `FPDFAvail_IsPageAvail()`, `FPDFAvail_IsFormAvail()`, `FPDFAvail_IsLinearized()`, `FPDFAnnot_GetFontColor()`, `FPDFPageObj_TransformF()`, `FPDFPageObj_GetMarkedContentID()`, `FPDF_StructElement_GetActualText()`, `FPDF_StructElement_GetObjType()`, `FPDF_StructElement_GetChildMarkedContentID()`, `FPDF_StructElement_GetAttributeAtIndex()`, `FPDF_StructElement_Attr_GetName()`, `FPDF_StructElement_Attr_GetValue()`, `FPDF_StructElement_Attr_GetType()`, `FPDF_StructElement_Attr_GetBooleanValue()`, `FPDF_StructElement_Attr_GetNumberValue()`, `FPDF_StructElement_Attr_GetStringValue()`, `FPDF_StructElement_Attr_GetBlobValue()`, `FPDF_StructElement_Attr_GetChildAtIndex()`, `FPDF_StructElement_GetMarkedContentIdCount()`, `FPDF_StructElement_GetMarkedContentIdAtIndex()`, `FPDF_ARGB()`, `FPDF_GetBValue()`, `FPDF_GetGValue()`, `FPDF_GetRValue()`, `FPDF_MovePages()`, `FPDFImageObj_GetImagePixelSize()`, `FPDFPageObj_GetRotatedBounds()`, `FPDFText_LoadCidType2Font()`, `FPDFTextObj_GetRenderedBitmap()`, `FPDF_FFLDrawSkia()`, `FPDF_NewXObjectFromPage()`, `FPDF_CloseXObject()`, `FPDF_NewFormObjectFromXObject()`, `FPDF_CopyViewerPreferences()`, `FPDF_RenderPageBitmapWithColorScheme_Start()`, `FPDF_RenderPageBitmap_Start()`, `FPDF_RenderPage_Continue()`, `FPDF_RenderPage_Close()`, `FPDF_GetDefaultTTFMap()`, `FPDF_GetDefaultTTFMapCount()`, `FPDF_GetDefaultTTFMapEntry()`, `FPDF_AddInstalledFont()`, `FPDFText_IsGenerated()`, `FPDFText_IsHyphen()`, `FPDFText_HasUnicodeMapError()`, `FPDF_InitLibraryWithConfig()`, `FPDF_SetSandBoxPolicy()`, `FPDF_SetPrintMode()`, `FPDF_DocumentHasValidCrossReferenceTable()`, `FPDF_GetTrailerEnds()`, `FPDF_GetDocUserPermissions()`, `FPDF_GetPageWidth()` `FPDF_GetPageHeight()` `FPDF_GetPageSizeByIndex()` `FPDF_RenderPageSkia()`, `FPDF_CountNamedDests()`, `FPDF_GetNamedDestByName()`, `FPDF_GetNamedDest()`, `FPDF_GetXFAPacketCount()`, `FPDF_GetXFAPacketName()`, `FPDF_GetXFAPacketContent()`, `FPDF_GetRecommendedV8Flags()`, `FPDF_GetArrayBufferAllocatorSharedInstance()`, `FPDF_BStr_Init()`, `FPDF_BStr_Set()`, `FPDF_BStr_Clear()`, `FPDF_SetPrintMode()`, and `FPDF_RenderPage()`; adds `pfdium_use_skia`,`pdfium_use_win32`, `pdfium_enable_xfa`, and `pdfium_enable_v8` crate feature flags; adjusts dependency versions in `Cargo.toml` to meet a minimum supported Rust version (MSRV) of Rust 1.60; establishes upper bound on `bindgen` dependency to avoid a build failure when compiling to WASM that was introduced in `bindgen` versions 0.70.0 and later, as described at <https://github.com/ajrcarey/pdfium-render/issues/156>.
* 0.8.24: introduces crate feature flags for selecting Pdfium API versions to use in `PdfiumLibraryBindings`; reworked `build.rs` to output bindings for multiple sets of Pdfium header files; reworks bindings implementations to differentiate between API versions that include the `FPDFFont_*` and `FPDFText_GetTextObject()` functions added in 0.8.23, and API versions that do not; reinstates `FPDFText_GetTextRenderMode()` function removed in 0.8.23 behind feature flags; adds WASM bindings utility function `copy_string_to_pdfium()` to correctly copy the string data of an `FPDF_WIDESTRING` to Pdfium's WASM memory module, instead of just the pointer location; adds `PdfiumLibraryBindings::version()` function for reporting the currently configured API version; internally reorganizes source code layout to make the code structure clearer. 
* 0.8.23: synchronizes Pdfium API header files against mainline; removes binding for function `FPDFText_GetTextRenderMode()` in response to upstream change described at <https://github.com/ajrcarey/pdfium-render/issues/151>; adds bindings for `FPDFText_GetTextObject()`, `FPDFFont_GetFamilyName()`, `FPDFFont_GetIsEmbedded()`, and `FPDFFont_GetFontData()` functions; deprecates `PdfFont::name()` function in favour of `PdfFont::family()` to match upstream naming changes; adds new functions `PdfFont::is_embedded()` and `PdfFont::data()` for retrieving embedded font data; updates `examples/fonts.rs` example; adds new function `PdfPageTextChar::text_object()` for retrieving the page object containing a specific character. Deprecated items will be removed in release 0.9.0.
* 0.8.22: adds bindings for `FPDFPage_TransformAnnots()`, thanks to an excellent contribution from <https://github.com/liammcdermott>; adds bindings for `FPDF_GetPageSizeByIndexF()`, thanks to an excellent contribution from <https://github.com/DorianRudolph>; updates all examples and tests that reference functionality from the `image` crate to use calls compatible with both `image` 0.25.x and 0.24.x, thanks to an excellent contribution from <https://github.com/DorianRudolph>; adds support for both static linking to a dynamically-built Pdfium library at compile time and dynamic binding caching for improved runtime performance, thanks to an excellent contribution from <https://github.com/DorianRudolph>; adds the `PdfPages::get_page_size()` and `PdfPages::get_page_sizes()` functions, thanks to an excellent contribution from <https://github.com/DorianRudolph>; removes an unneeded internal dependency on the `iter_tools` crate, thanks to an excellent contribution from <https://github.com/aruediger>.
* 0.8.21: adds the `PdfFormFieldText::set_value()` function for setting the values of text form fields, thanks to an excellent contribution from <https://github.com/liammcdermott>; adds new `examples/fill_form_field.rs` example.
* 0.8.20: adds `PdfPageAnnotationAttachmentPoints` struct and matching iterator; adds new annotation functions to `PdfPageAnnotationCommon` along with their matching implementations in `PdfPageAnnotationPrivate`, including `PdfPageAnnotationCommon::set_bounds()`, `PdfPageAnnotationCommon::set_position()`, `PdfPageAnnotationCommon::set_width()`, `PdfPageAnnotationCommon::set_height()`, `PdfPageAnnotationCommon::set_creation_date()`, `PdfPageAnnotationCommon::set_modification_date()`; `PdfPageAnnotationCommon::stroke_color()`, `PdfPageAnnotationCommon::set_stroke_color()`, `PdfPageAnnotationCommon::fill_color()`, `PdfPageAnnotationCommon::set_fill_color()` functions; adds `PdfPageAnnotationCommon::attachment_points()` accessor function; adds conversion from `chrono::DateTime` types to PDF date strings in `utils::dates`; adds mutability and annotation creation functions to `PdfPageAnnotations` collection; adds new `create_annotations.rs` example; adds `PdfPageTextSegment::chars()` convenience function.
* 0.8.19: adds bindings for `FORM_OnAfterLoadPage()`, `FORM_OnBeforeClosePage()`, `FPDFCatalog_IsTagged()`, `FPDFBookmark_GetCount()`, and `FPDF_GetPageAAction()` functions; adds the `PdfBookmark::children_len()` function; adjusts the behaviour of `PdfPage::flatten()` so that the page is reloaded after the call to `FPDFPage_Flatten()`. This ensures that the effect of the flatten operation is immediately visible to the caller; previously, it was necessary for the caller to explicitly drop and reload the page. For more details, see <https://github.com/ajrcarey/pdfium-render/issues/140>.
* 0.8.18: adjusts `PdfiumRenderWasmState::bind_to_pdfium()` to fall back to `Module["wasmExports"]["__indirect_function_table"]` if `Window.wasmTable` global variable is not available, in response to upstream packaging changes at <https://github.com/paulocoutinhox/pdfium-lib/releases>. For more details, see <https://github.com/ajrcarey/pdfium-render/issues/134>.
* 0.8.17: updates all examples (except for `export.rs`) to use extended `Pdfium::default()` implementation introduced in 0.8.12; fixes a segmentation fault in `PdfDocument::drop()` that can occur when using a V8/XFA-enabled build of Pdfium; adjusts `PdfiumRenderWasmState::bind_to_pdfium()` to fall back to `Module["wasmExports"]["malloc"]` and `Module["wasmExports"]["free"]` if `Module["_malloc"]` and `Module["_free"]` are not available, in response to upstream packaging changes at <https://github.com/paulocoutinhox/pdfium-lib/releases>. For more details, see <https://github.com/ajrcarey/pdfium-render/issues/128>.
* 0.8.16: deprecates `PdfBitmap::as_bytes()` function in favour of `PdfBitmap::as_raw_bytes()`; adds new `PdfBitmap::as_rgba_bytes()` for returning pixel byte data with normalized color channels, irrespective of the original bitmap pixel format; updates the WASM-specific `PdfBitmap::as_image_data()` function to use `PdfBitmap::as_rgba_bytes()` instead of `PdfBitmap::as_raw_bytes()`, ensuring the  color normalization behaviour of both WASM and non-WASM builds is identical; refactors `PdfBookmarksIterator` to use a standard depth-first graph traversal algorithm in response to <https://github.com/ajrcarey/pdfium-render/issues/120>; adds `PdfBookmark::destination()` function for retrieving the target destination of the action assigned to a bookmark, thanks to an excellent contribution from <https://github.com/xVanTuring>. Deprecated items will be removed in release 0.9.0.
* 0.8.15: adds new `reset_matrix()` and `reset_matrix_to_identity()` functions to consumers of the `create_transform_setters!()` macro; deprecates `set_matrix()` in favour of `apply_matrix()` and `PdfPage::set_matrix_with_clip()` in favour of `PdfPage::apply_matrix_with_clip()`; adds a matching corrects a byte alignment bug that could occur when converting three-bytes-per-pixel bitmaps to four-bytes-per-pixel bitmaps, thanks to an excellent contribution from <https://github.com/vladmovchan>. Deprecated items will be removed in release 0.9.0.
* 0.8.14: adjusts the `PdfSearchOptions::as_pdfium()` function introduced in 0.8.13 to return a `c_ulong` in order to fix a build-time error specific to Windows.
* 0.8.13: addresses incorrect results returned by `PdfPageTextObject::chars()` as described in <https://github.com/ajrcarey/pdfium-render/issues/98>; adds new `PdfPageTextSearch` and `PdfSearchOptions` objects and new `PdfPageText::search()` function for running text searches across the text of a single page, thanks to an excellent contribution from <https://github.com/zhonghua-wang>; adds new `examples/text_search.rs` example.
* 0.8.12: improves backwards compatibility with Rust versions prior to 1.62.0 for the `PdfAppearanceMode` enum added in 0.8.11 and the `Ord` trait implementation for `PdfPoints` added in 0.8.10; adds bindings for `FPDF_PageToDevice()` and `FPDF_DeviceToPage()` coordinate system conversion functions; exposes equivalent functionality in the high-level interface via new `PdfPage::points_to_pixels()` and `PdfPage::pixels_to_points()` functions; adds new `examples/export_clip_crop.rs` example; extends implementation of `Pdfium::default()` to try to load a Pdfium library located in the current working directory as well as a system library.
* 0.8.11: adds the `PdfAppearanceMode` enum, the `PdfFormFieldCommon::appearance_stream()` and `PdfFormFieldCommon::appearance_mode_value()` functions, supporting internal implementation of those functions in `PdfFormFieldPrivate`; improves implementation of `PdfFormRadioButtonField::is_checked()` to take appearance streams into account; improves implementation of `PdfForm::field_values()` to take control groups into account.
* 0.8.10: adds matrix math operations to `PdfMatrix`; adds `PdfRect::transform()` and `PdfMatrix::apply_to_points()` functions for transforming rectangles and points; uses matrix math operations in `PdfMatrix` to simplify implementation of `PdfRenderConfig`; adds `PdfPagePathObjectSegments::raw()` and `PdfPagePathObjectSegments::transform()` functions to allow iteration over raw or transformed path segment coordinates respectively; adds `PdfDestinationViewSettings` enum and `PdfDestination::view()` function for retrieving the view settings for an internal document destination.
* 0.8.9: changes `Pdfium::bind_to_library()` and `Pdfium::pdfium_platform_library_name_at_path()` to take and return `AsRef<Path>` and `PathBuf` types rather than strings, thanks to an excellent contribution from <https://github.com/heimmat>.
* 0.8.8: adjusts `PdfiumRenderWasmState::bind_to_pdfium()` to fall back to `Module["asm"]["malloc"]` and `Module["asm"]["free"]` if `Module["_malloc"]` and `Module["_free"]` are not available, in response to upstream packaging changes at <https://github.com/paulocoutinhox/pdfium-lib/releases>. For more details, see <https://github.com/ajrcarey/pdfium-render/issues/95>.
* 0.8.7: renames `PdfBitmapFormat::BRGx` to `PdfBitmapFormat::BGRx`, deprecating the misspelled variant; adds `Send` and `Sync` implementations for `PdfDocument` struct when using the `sync` crate feature; adds `Debug` trait implementation to `Pdfium` for better `once_cell` compatibility; adds new constants `PdfPoints::MAX`, `PdfPoints::MIN`, and `PdfRect::MAX`; corrects a clipping bug in `PdfPage::transform()` and `PdfPage::set_matrix()` by setting the default clipping area to `PdfRect::MAX` rather than `PdfPage::size()`. Deprecated items will be removed in release 0.9.0.
* 0.8.6: fixes a bug in `PdfColor::as_pdfium_color()` that resulted in the alpha value being ignored when composing the `FPDF_DWORD` representation of the color value; renames `PdfBitmapRotation` enum to `PdfPageRenderRotation`, deprecating the old enum; adds convenience functions `PdfColor::mix()`, `PdfColor::mix_with()`, `PdfColor::from_hex()`, `PdfColor::to_hex()`, and `PdfColor::to_hex_with_alpha()`; adds a wide variety of new color constants to `PdfColor`, deprecating all existing `PdfColor::SOLID_*` consts in favour of renamed consts with the `SOLID_` prefix removed; moves `PdfPoints` and `PdfRect` structs out into new files; adds `PdfQuadPoints` struct; adds implementations of `Display` to `PdfPoints`, `PdfRect`, and `PdfQuadPoints`; fixes a double-free bug in `PdfPageImageObject::get_image_from_bitmap_handle()`. Deprecated items will be removed in release 0.9.0.
* 0.8.5: adds `PdfDestination::page_index()` function; adds `PdfPageObjectCommon::dash_phase()` `PdfPageObjectCommon::set_dash_phase()`, `PdfPageObjectCommon::dash_array()`, and `PdfPageObjectCommon::set_dash_array()` functions thanks to an excellent contribution from <https://github.com/DorianRudolph>.
* 0.8.4: fixes conditional import of `PdfPoints` struct into `PdfPageImageObject` so it is no longer dependent on the `image` crate feature being enabled; corrects a bug in the calculation of rendered bitmap pixel dimensions, thanks to an excellent contribution from <https://github.com/slawekkolodziej>.
* 0.8.3: adds `PdfFonts` collection, `PdfDocument::fonts()` and `PdfDocument::fonts_mut()` accessor functions, and `PdfFontToken` struct; moves font constructors from `PdfFont` into `PdfFonts`, deprecating constructors in `PdfFont`; adds `ToPdfFontToken` trait, along with implementations of the trait for `PdfFont`, `&PdfFont`, and `PdfFontToken`; adjusts all functions that previously took a `PdfFont` or `&PdfFont` reference so that they now take a `impl ToPdfFontToken`. Deprecated items in `PdfFont` will be removed in release 0.9.0.
* 0.8.2: adds `PdfBitmap::from_bytes()` function in response to <https://github.com/ajrcarey/pdfium-render/issues/83>; relaxes lifetime requirements on `Pdfium::load_pdf_from_reader()` and related functions thanks to an excellent contribution from <https://github.com/bavardage>.
* 0.8.1: changes the data type of `PdfBitmap::Pixels` from `u16` to `c_int` and adds the `PdfBitmap::bytes_required_for_size()` helper function in response to <https://github.com/ajrcarey/pdfium-render/issues/80>.
* 0.8.0: removes the ability to acquire an owned `PdfPages` instance from `PdfDocument::pages()` as per <https://github.com/ajrcarey/pdfium-render/issues/47>; adds new `PdfDocument::pages_mut()` function to match reworked `PdfDocument::pages()` function; fixes a bug in the WASM implementation of `FPDFText_GetBoundedText()` as detailed in <https://github.com/ajrcarey/pdfium-render/issues/77>; reworks handling of `FPDF_GetLastError()` as detailed in <https://github.com/ajrcarey/pdfium-render/issues/78>.
* 0.7.34: replaces functions in `PdfPageLinks` using linear traversal with binary search traversal; adds new `PdfFormField` enum; renames `PdfPageObjectFormFragment` to `PdfPageXObjectFormObject` to disambiguate it from `PdfForm` and `PdfFormField`; adds `PdfPageAnnotationCommon::as_form_field()` accessor function; adds form field structs `PdfFormPushButtonField`, `PdfFormCheckboxField`, `PdfFormComboBoxField`, `PdfFormListBoxField`, `PdfFormRadioButtonField`, `PdfFormSignatureField`, `PdfFormTextField`, and `PdfFormUnknownField`; adds `PdfFormFieldOption` struct and `PdfFormFieldOptions` collection for retrieving the options displayed within a list box or combo box field; adds `PdfFormFieldCommon` and `PdfFormFieldPrivate` traits and associated implementations for all `PdfFormField` field types; adds the `PdfForm::field_values()` convenience function; adds `examples/form_fields.rs` example.
* 0.7.33: adds the `create_transform_setters!()` and `create_transform_getters!()` private macros, ensuring API consistency and maximising code reuse across all transformable objects; adds `PdfPage::transform()`, `PdfPage::transform_with_clip()`, and `PdfPage::set_matrix_with_clip()` functions; adds `examples/matrix.rs` example; adds bindings for remaining `FPDF_*ClipPath*()` functions.
* 0.7.32: fixes off-by-one errors in `PdfPageText::chars_inside_rect()` and `examples/chars.rs` thanks to an excellent contribution from <https://github.com/luketpeterson>, adds support for grayscale image processing to `PdfPageImageObject::get_image_from_bitmap_handle()` thanks to an excellent contribution from <https://github.com/stephenjudkins>, and corrects a missing dependency when using `pdfium-render` without the default `image` crate feature.
* 0.7.31: adds the `PdfPageLinks` collection, the `PdfPage::links()` and `PdfPage::links_mut()` functions, the `PdfLink` and `PdfDestination` structs, the `PdfActionCommon` and `PdfActionPrivate` traits, structs for the action types supported by Pdfium, the `PdfActionUri::uri()` function to address <https://github.com/ajrcarey/pdfium-render/issues/68>, and the new `examples/links.rs` example.
* 0.7.30: deprecates the `PdfPages::delete_page_at_index()` and `PdfPages::delete_page_range()` functions; adds `PdfPage::delete()` function in response to <https://github.com/ajrcarey/pdfium-render/issues/67>. Deprecated items will be removed in release 0.9.0, although it may be possible to restore these functions if safer reference handling in `PdfDocument` and `PdfPages` is introduced as part of <https://github.com/ajrcarey/pdfium-render/issues/47>.
* 0.7.29: removes the `sync` crate feature from the list of default crate features in response to <https://github.com/ajrcarey/pdfium-render/issues/66>.
* 0.7.28: removes the `PdfPageObjects::take_*()` functions; adds `PdfPageObject::is_copyable()` `PdfPageObject::try_copy()`, `PdfPageObjectGroup::retain()`, `PdfPageObjectGroup::retain_if_copyable()`, `PdfPageObjectGroup::is_copyable()`, `PdfPageObjectGroup::try_copy_onto_existing_page()`, `PdfPageObjectGroup::copy_onto_new_page_at_start()`, `PdfPageObjectGroup::copy_onto_new_page_at_end()`, and `PdfPageObjectGroup::copy_onto_new_page_at_index()` functions; adds `examples/copy_objects.rs` example; fixes a bug in the propagation of a page's content regeneration strategy; removes all use of `lazy_static!` macro in favour of `once_cell::sync::Lazy`.
* 0.7.27: adjusts `examples/index.html` to take into account upstream packaging changes in the WASM builds of Pdfium published at <https://github.com/paulocoutinhox/pdfium-lib/releases>; adds the `image` crate feature.
* 0.7.26: adds `sync` default crate feature providing `Send` and `Sync` implementations for `Pdfium` struct; adds `Display` and `Error` trait implementations to `PdfiumError` for `anyhow` compatibility; adjusts WASM example to account for upstream changes in Emscripten packaging of Pdfium WASM builds; corrects a lifetime problem in `Pdfium::load_pdf_from_bytes()` and deprecates `Pdfium::load_pdf_from_bytes()` in favour of `Pdfium::load_pdf_from_byte_slice()` and `Pdfium::load_pdf_from_byte_vec()`. Deprecated items will be removed in release 0.9.0.
* 0.7.25: adds the `PdfPageAnnotationObjects` collection and the `PdfPageAnnotation::objects()`, `PdfPageInkAnnotation::objects_mut()`, and `PdfPageStampAnnotation::objects_mut()` functions to the high-level interface.
* 0.7.24: adds bindings for `FPDFClipPath_CountPathSegments()`, `FPDFClipPath_GetPathSegment()`, `FPDFPath_CountSegments()`, `FPDFPath_GetPathSegment()`, and `FPDFPathSegment_*()` functions; adds `PdfFontGlyphs` and `PdfPagePathObjectSegments` collections to the high-level interface, along with accessor functions in `PdfFont` and `PdfPagePathObject`; adds the `PdfPathSegments` trait; introduces some infrastructure necessary for the future implementation of a `PdfClipPath` object; adds `PdfPages::first()`, `PdfPages::last()`, and `PdfPage::fonts()` convenience functions.
* 0.7.23: removes some unnecessary mutable bindings in `PdfBitmap`; uses `#[cfg(doc)]` declarations to ensure `cargo doc` generates documentation for all functionality, irrespective of the platform.
* 0.7.22: attempts to work around two problems in Pdfium's bitmap generation when retrieving processed renderings of page image objects. See <https://github.com/ajrcarey/pdfium-render/issues/52> for more information.
* 0.7.21: adds bindings for `FPDF_GetPageAAction()`, `FPDF_GetFileIdentifier()`, and all remaining `FPDFDest_*()` and `FPDFLink_*()` functions; adds `PdfAttachment::len()` and `PdfAttachment::is_empty()` convenience functions; adds `libstdc++` and `libc++` crate features; adds color conversion functions to `PdfiumLibraryBindings`; corrects bugs in color conversion when working with `PdfPageImageObject`, as detailed in <https://github.com/ajrcarey/pdfium-render/issues/50>; fixes a bug in the WASM implementation of `FPDFAnnot_GetAttachmentPoints()`; corrects some small typos in examples.
* 0.7.20: adds bindings for `FPDFPage_*Thumbnail*()`, `FPDFLink_*()`, and `FPDFText_Find*()` functions; adds `PdfAttachments::create_attachment_from_bytes()`, `PdfAttachments::create_attachment_from_file()`, `PdfAttachments::create_attachment_from_reader()`, `PdfAttachments::create_attachment_from_fetch()`, `PdfAttachments::create_attachment_from_blob()`, `PdfAttachments::delete_at_index()`, `PdfAttachment::save_to_writer()`, `PdfAttachment::save_to_file()`, `PdfAttachment::save_to_blob()`, `PdfPage::has_embedded_thumbnail()`, `PdfPage::embedded_thumbnail()`, and `PdfPage::boundaries_mut()` functions to the high-level interface; renames `PdfAttachment::bytes()` function introduced in 0.7.19 to `PdfAttachment::save_to_bytes()`.
* 0.7.19: adds bindings for `FPDFDoc_*Attachment*()` functions; adds `PdfAttachments` and `PdfSignatures` collections to the high-level interface.
* 0.7.18: adds convenience `bindings()` accessor functions to `PdfDocument`, `PdfPage`, `PdfBitmap`, `PdfFont`, and various other interfaces, thanks to an excellent contribution from <https://github.com/LU15W1R7H>; deprecates `Pdfium::get_bindings()` in favour of `Pdfium::bindings()` for consistency. Deprecated items will be removed in release 0.9.0.
* 0.7.17: relaxes some unnecessarily restrictive lifetime bounds in `PdfPageObjectPath`. 
* 0.7.16: adds `PdfPageObjects::create_path_object_bezier()` and `PdfPageObjectPath::new_bezier()` convenience functions; corrects some typos in documentation.
* 0.7.15: adds `PdfPageAnnotationCommon::name()`, `PdfPageAnnotationCommon::contents()`, `PdfPageAnnotationCommon::author()`, `PdfPageAnnotationCommon::creation_date()`, and `PdfPageAnnotationCommon::modification_date()` functions for working with annotations; adds `PdfPageText::for_annotation()` and `PdfPageText::chars_for_annotation()` for more easily extracting text and characters associated with annotations; adds `examples/annotations.rs` and `examples/image_extract.rs`; renames `examples/text.rs` to `examples/text_extract.rs`.
* 0.7.14: fixes a bug in the WASM implementation of `FPDF_StructElement_GetStringAttribute()`; pins required version of `image` crate to at least 0.24.0 or later to avoid incompatibility between the `image::DynamicImage` trait definitions in 0.23.x and 0.24.x; adds compatibility with web workers to the WASM implementation, thanks to an excellent contribution from <https://github.com/NyxCode>.
* 0.7.13: adds transformation and clipping functions to `PdfRenderConfig`; adds bindings for `FPDF_RenderPageBitmapWithMatrix()`; deprecates `PdfRenderConfig::rotate_if_portait()` in favour of the correctly-spelled `PdfRenderConfig::rotate_if_portrait()`. Deprecated items will be removed in release 0.9.0.
* 0.7.12: adds `PdfPage::render_into_bitmap()` and `PdfPage::render_into_bitmap_with_config()` functions for higher performance; deprecates `PdfPage::get_bitmap()` in favour of `PdfPage::render()`; deprecates `PdfPage::get_bitmap_with_config()` in favour of `PdfPage::render_with_config()`; deprecates `PdfBitmapConfig` in favour of `PdfRenderConfig`; deprecates `PdfBitmap::render()` as the function is no longer necessary. Deprecated items will be removed in release 0.9.0.
* 0.7.11: adds the new WASM-specific `PdfBitmap::as_array()` function as a higher performance alternative to the cross-platform `PdfBitmap::as_bytes()` function, thanks to an excellent contribution from <https://github.com/NyxCode>.
* 0.7.10: corrects some typos in documentation; adds additional constructors to `PdfPageImageObject` that apply a specified width and/or height at object creation time.
* 0.7.9: adds retrieval of the list of image filters applied to a `PdfPageImageObject`; adds the `PdfColorSpace` enum; adds bindings for the `FPDF_*Signature*()`, `FPDFSignatureObj_*()`, and `FPDF_StructTree_*()` functions.
* 0.7.8: adds image support to the `PdfPageImageObject` struct, the `PdfPageObjects::add_image_object()` and `PdfPageObjects::create_image_object()` functions, additional convenience functions for loading fonts from files and readers to `PdfFont`, and bindings for `FPDF_VIEWERREF_Get*()` functions.
* 0.7.7: adds the `thread_safe` crate feature and the accompanying example in `examples/thread_safe.rs`.
* 0.7.6: adds retrieval of text settings on a character-by-character basis to the `PdfPageText` and `PdfPageTextObject` objects; adds `PdfPageTextSegment` and `PdfPageTextChar` structs to the  high-level interface; adds retrieval of current transformation settings to all page objects; adds the `PdfPageTextObject::scaled_font_size()` function and renames `PdfPageTextObject::font_size()` to `PdfPageTextObject::unscaled_font_size()` as these names make clearer the differences between scaled and unscaled font sizes in text objects; adds bindings for all remaining `FPDFText_*()` functions.
* 0.7.5: corrects a bug in error handling on Windows. See <https://github.com/ajrcarey/pdfium-render/issues/24> for more information.
* 0.7.4: adds the `PdfPageGroupObject::remove_objects_from_page()` function; renamed `PdfPageObjects::delete_object()` and `PdfPageObjects::delete_object_at_index()` functions to `PdfPageObjects::remove_object()` and `PdfPageObjects::remove_object_at_index()` as these names better reflect the underlying operation that occurs.
* 0.7.3: corrects a bug in the implementation of `PdfPages::append()` introduced in 0.7.2. 
* 0.7.2: adds object groups for manipulating and transforming groups of page objects as if they were a single object, and the `PdfPages::watermark()` function for applying individualized watermarks to any or all pages in a document. Fixes a potential double-free bug in `PdfFont::drop()`.
* 0.7.1: adds path segment creation to the `PdfPagePathObject` object, convenience functions for quickly creating rectangles, ellipses, and circles, and the `PdfPageObjects::add_path_object()` function.
* 0.7.0: adds `PdfPermissions` collection, adds document loading and saving support, adds initial creation and editing support for documents, pages, and text objects, and improves WASM document file handling.
* 0.6.0: fixes some typos in documentation, updates upstream Pdfium WASM package source repository name.
* 0.5.9: corrects a bug in the statically linked bindings implementation. Adjusted tests to cover both dynamic and statically linked bindings implementations.
* 0.5.8: corrects a bug in the WASM implementation of certain `FPDFAnnot_*()` functions. Resolves a potential memory leak affecting the WASM implementation of various `FPDF_*()` functions.
* 0.5.7: adds support for binding to a statically-linked build of Pdfium, adds `bindgen` and `static` crate features.
* 0.5.6: adds `pdfium_render::prelude`, adds bindings for `FPDFAnnot_*()` and `FPDFPage_*Annot*()` functions, adds `PdfPageAnnotations` collection and `PdfPageAnnotation` struct to the high-level interface.
* 0.5.5: fixes two bugs in the WASM implementation, one to do with colors, one to do with text extraction. See <https://github.com/ajrcarey/pdfium-render/issues/9> and <https://github.com/ajrcarey/pdfium-render/issues/11> for more information.
* 0.5.4: changes default setting of `PdfBitmapConfig::set_reverse_byte_order()` to `true` to switch from Pdfium's default BGRA8 pixel format to RGBA8. This is necessary since the `image` crate dropped support for BGRA8 in version 0.24. See <https://github.com/ajrcarey/pdfium-render/issues/9> for more information.
* 0.5.3: adds bindings for `FPDFBookmark_*()`, `FPDFPageObj_*()`, `FPDFText_*()`, and `FPDFFont_*()` functions, adds `PdfPageObjects`, `PdfPageText`, and `PdfBookmarks` collections to the high-level interface.
* 0.5.2: adds bindings for `FPDF_GetPageBoundingBox()`, `FPDFDoc_GetPageMode()`, `FPDFPage_Get*Box()`, and `FPDFPage_Set*Box()` functions, adds `PdfPageBoundaries` collection to the high-level interface.
* 0.5.1: adds bindings for `FPDFPage_GetRotation()` and `FPDFPage_SetRotation()` functions, adds `PdfMetadata` collection to the high-level interface.
* 0.5.0: adds rendering of annotations and form field elements, thanks to an excellent contribution from <https://github.com/inzanez>.
* 0.4.2: bug fixes in `PdfBitmapConfig` implementation.
* 0.4.1: improvements to documentation and READMEs.
* 0.4.0: initial release of minimal page rendering functionality.
