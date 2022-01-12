# Idiomatic Rust bindings for Pdfium

`pdfium-render` provides an idiomatic high-level Rust interface around the low-level bindings to
Pdfium exposed by the excellent `pdfium-sys` crate.

```
    // Exports each page in a given test file to separate JPEG images
    // in the current working directory.

    Pdfium::new(Pdfium::bind_to_system_library().unwrap())
        .load_pdf_from_file("./test/test.pdf", None)
        .unwrap()
        .pages()
        .for_each(|page| {
            page
                .get_bitmap_with_config(&PdfBitmapConfig::new()
                    .set_target_width(2000)
                    .set_maximum_height(2000)
                    .rotate_if_landscape(PdfBitmapRotation::Degrees90, true))
                .unwrap()
                .as_image() // Renders this page to an Image::DynamicImage
                .as_bgra8()
                .unwrap()
                .save_with_format(format!("test-page-{}.jpg", page.index()), ImageFormat::Jpeg)
                .unwrap();
        });
```

In addition to providing a more natural interface to Pdfium, `pdfium-render` differs from
`pdfium-sys` in several other useful ways:

* `pdfium-render` uses `libloading` to _late-bind_ to a Pdfium library, whereas `pdfium-sys` binds to a library at compile time (and is a bit fiddly to configure). This makes `pdfium-render` considerably more flexible to work with, and enables dynamic switching between bundled libraries and system libraries, as well as idiomatic Rust error handling of situations where a Pdfium library is not available at runtime.
* Late-binding to Pdfium means that `pdfium-render` can be used to target Pdfium-compiled-to-WASM for running in a browser, which is not possible with `pdfium-sys`.
* Pages rendered by Pdfium can be exported as instances of `Image::DynamicImage` for easy, idiomatic post-processing. 

# Development status

The initial focus of this crate has been on rendering pages in a PDF file; consequently, FPDF_*
functions related to bitmaps and rendering have been prioritised. By 1.0, the functionality of all
FPDF_* functions exported by Pdfium will be available.

If you need a function that is not currently exposed, just raise an issue.

# Compiling to WASM

See `examples/README.md` for a fully worked and commented example that shows how to bundle a Rust
application using `pdfium-render` alongside a pre-built Pdfium WASM module for in-browser
introspection and rendering of PDF files.

# Porting existing Pdfium code from other languages

The high-level idiomatic Rust interface provided by the `Pdfium` struct is entirely optional;
the `Pdfium` struct wraps around raw FFI bindings provided by the `PdfiumLibraryBindings`
trait, and it is completely feasible to simply use those raw FFI bindings directly
rather than the high level interface. This makes porting existing code that calls FPDF_* functions
very straight-forward, while still gaining the benefits of runtime library binding and
WASM compatibility provided by `pdfium-render`. For instance, the following C++ code snippet:

```
    string test_doc = "myTest.pdf";

    FPDF_InitLibrary();
    FPDF_DOCUMENT doc = FPDF_LoadDocument(test_doc, NULL);
    // ... do something with doc
    FPDF_CloseDocument(doc);
    FPDF_DestroyLibrary();
```

would translate to the following Rust code:

```
    let bindings = Pdfium::bind_to_system_library()?;
    
    let test_doc = "myTest.pdf";

    bindings.FPDF_InitLibrary();
    let doc = bindings.FPDF_LoadDocument(test_doc, None);
    // ... do something with doc
    bindings.FPDF_CloseDocument(doc);
    bindings.FPDF_DestroyLibrary();
```

# External Pdfium builds

`pdfium-render` does not bundle Pdfium at all. You can either bind to a system-provided library,
or package an external build of Pdfium alongside your Rust application. For WASM, packaging an
external build of Pdfium as a WASM module is essential.

* Native builds of Pdfium for all major platforms: https://github.com/bblanchon/pdfium-binaries/releases
* WASM builds of Pdfium, suitable for deploying alongside `pdfium-render`: https://github.com/paulo-coutinho/pdfium-lib/releases
