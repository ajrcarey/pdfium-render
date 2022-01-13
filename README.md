# Idiomatic Rust bindings for Pdfium

`pdfium-render` provides an idiomatic high-level Rust interface around the low-level bindings to
Pdfium exposed by the excellent `pdfium-sys` crate.

```
    // Iterates over each page in the given test PDF file and renders each page
    // as a separate JPEG image in the current working directory.

    Pdfium::new(Pdfium::bind_to_system_library()?)
        .load_pdf_from_file("test.pdf", None)
        .pages()
        .for_each(|page| {
            page
                .get_bitmap_with_config(&PdfBitmapConfig::new()
                    .set_target_width(2000)
                    .set_maximum_height(2000)
                    .rotate_if_landscape(PdfBitmapRotation::Degrees90, true))?
                .as_image() // Renders this page to an Image::DynamicImage
                .as_bgra8()?
                .save_with_format(format!("test-page-{}.jpg", page.index()), ImageFormat::Jpeg)?;
        });
```

In addition to providing a more natural interface to Pdfium, `pdfium-render` differs from
`pdfium-sys` in several other important ways:

* `pdfium-render` uses `libloading` to late bind to a Pdfium library at run-time, whereas
  `pdfium-sys` binds to a library at compile-time. Not only can compile-time binding be a
  bit fiddly to configure, but it precludes compiling `pdfium-sys` to WASM. By binding
  to Pdfium at run-time instead of compile-time, `pdfium-render` can dynamically switch between
  bundled libraries and system libraries and offer idiomatic Rust error handling in situations where
  a Pdfium library is not available.
* Late binding to Pdfium means that `pdfium-render` can be compiled to WASM for running in a
  browser; this is not possible with `pdfium-sys`.
* Pages rendered by Pdfium can be exported as instances of `Image::DynamicImage` for easy,
  idiomatic post-processing. 

## Porting existing Pdfium code from other languages

The high-level idiomatic Rust interface provided by the `Pdfium` struct is entirely optional;
the `Pdfium` struct wraps around raw FFI bindings defined in the `PdfiumLibraryBindings`
trait, and it is completely feasible to simply use those raw FFI bindings directly
rather than the high level interface. This makes porting existing code that calls FPDF_* functions
very straight-forward, while still gaining the benefits of late binding and
WASM compatibility. For instance, the following code snippet (taken from a C++ sample):

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
    let bindings = Pdfium::bind_to_system_library()?;
    
    let test_doc = "test.pdf";

    bindings.FPDF_InitLibrary();
    let doc = bindings.FPDF_LoadDocument(test_doc, None);
    // ... do something with doc
    bindings.FPDF_CloseDocument(doc);
    bindings.FPDF_DestroyLibrary();
```

## External Pdfium builds

`pdfium-render` does not bundle Pdfium at all. You can either bind to a system-provided library
or package an external build of Pdfium alongside your Rust application. When compiling to WASM,
packaging an external build of Pdfium as a WASM module is essential.

* Native builds of Pdfium for all major platforms: `<https://github.com/bblanchon/pdfium-binaries/releases>`
* WASM builds of Pdfium, suitable for deploying alongside `pdfium-render`: `<https://github.com/paulo-coutinho/pdfium-lib/releases>`

## Compiling to WASM

See `<https://github.com/ajrcarey/pdfium-render/tree/master/examples>` for a full example that shows
how to bundle a Rust application using `pdfium-render` alongside a pre-built Pdfium WASM module for
in-browser introspection and rendering of PDF files.

## Development status

The initial focus of this crate has been on rendering pages in a PDF file; consequently, FPDF_*
functions related to bitmaps and rendering have been prioritised. By 1.0, the functionality of all
FPDF_* functions exported by Pdfium will be available.

If you need a function that is not currently exposed, just raise an issue.
