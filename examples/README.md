# Examples

Simple examples demonstrating how to use `pdfium-render` on both native and WASM compilation targets.

For general comments about `pdfium-render` and binding to Pdfium, see `export.rs`.

Each example can run via `cargo run --example <example_name>`.

* `annotations.rs`: iterates over every annotation on every page in `test/annotations-test.pdf`, displaying information about each annotation.
* `attachments.rs`: generates a new document by embedding `test/annotations-test.pdf`, `test/create-test.pdf`, and `test/path-test.pdf` as attachments, saving the new document to `test/attachments.pdf`. 
* `chars.rs`: iterates over the individual characters in a text object to determine the bounding boxes of each word in the text object.
* `concat.rs`: generates a new document by concatenating pages from `test/export-test.pdf`, `test/form-test.pdf`, and `test/text-test.pdf` together, saving the new document to `test/concat-test.pdf`
* `copy_objects.rs`: moves a selection of page objects from one page to another using the object copying functions in `PdfPageGroupObject`, saving the new document to `test/copy-test.pdf`.
* `create.rs`: generates a new document by placing text objects onto a blank page, saving the new document to `test/create-test.pdf`.
* `descenders.rs`: iterates over the individual characters in a text object, measuring which have glyph shapes that descend beneath the text object's font baseline.
* `export.rs`: exports the individual pages in `test/export-test.pdf` to JPGs in the working directory. The example will attempt to bind to a copy of Pdfium in the working directory, falling back to the system-bundled library if local loading fails.
* `export_clip_crop.rs`: exports just a portion of the page in `test/export-clip-crop-test.pdf` to a JPG file, clipping and cropping the rendering output based on object properties in the file.
* `fonts.rs`: displays information about the 14 built-in PDF fonts retrieved from Pdfium.
* `form.rs`: exports the individual pages in `test/form-test.pdf` to JPGs in the working directory. The sample PDF includes pre-filled form fields, the values of which should also be rendered.
* `form_fields.rs`: iterates over the form fields in `test/form-test.pdf`, displaying information about each form field.
* `image.rs`: generates a new document by placing image objects onto a blank page, saving the new document to `test/image-test.pdf`.
* `image_extract.rs`: extracts and outputs the images on each page in `test/image-test.pdf` to files.
* `links.rs`: iterates over every link on every page in `test/links-test.pdf`, displaying information about each link.
* `matrix.rs`: uses a single `PdfMatrix` object to apply a consistent transformation to a variety of transformable PDF objects, saving its output to `test/matrix-test.pdf`.
* `objects.rs`: iterates over every page object on every page in `test/export-test.pdf`, displaying information about each page object.
* `path.rs`: generates a new document by placing path objects onto a blank page, saving the new document to `test/path-test.pdf`.
* `segments.rs`: iterates over every path object in `test/segments.pdf`, displaying information on each path segment in the path object.
* `signatures.rs`: iterates over every digital signature in `test/signatures.pdf`, displaying information about each signature.
* `text_extract.rs`: extracts and outputs the text on each page in `test/text-test.pdf` to the console.
* `text_search.rs`: finds and highlights a search term found on the first page of `test/text-test.pdf`, saving the result to a new document at `test/search-results.pdf`.
* `thread_safe.rs`: explains in comments `pdfium-render`'s approach to ensuring thread-safe access to Pdfium, and demonstrates using a parallel iterator to process multiple rendering tasks on separate threads.
* `tile.rs`: generates a new document by tiling pages from `test/export-test.pdf`, `test/form-test.pdf`, and `test/text-test.pdf`, saving the new document to `test/tile-test.pdf`.
* `wasm.rs`: demonstrates `pdfium-render` running in a browser. This requires some manual bundling of the correct resources; see below.
* `watermark.rs`: adds a watermark to each page in a previously-generated document, saving the watermarked document to `test/watermark-test.pdf`.

## Bundling for WASM

Since `pdfium-render` does not include Pdfium itself, an external pre-packaged WASM build of `pdfium` is required. Suitable builds are available from https://github.com/paulocoutinhox/pdfium-lib/releases.

* Build the WASM module for the sample: `cargo install wasm-pack && wasm-pack build examples/ --target no-modules`. This creates a WASM module and supporting Javascript files in `examples/pkg`.
* Copy the `pdfium_render_wasm_example.js` and `pdfium_render_wasm_example_bg.wasm` files from `examples/pkg` into a release folder.
* Download a pre-packaged WASM build from https://github.com/paulocoutinhox/pdfium-lib/releases and extract the `release/node/pdfium.js` and `release/node/pdfium.wasm` files into your release folder.
* Copy the `index.html` file from `examples` into your release folder.
* Copy a sample PDF file into your release folder and name it `test.pdf`. Any well-formed, non-secured PDF is a suitable sample, including the test files in https://github.com/ajrcarey/pdfium-render/tree/master/test.
* Optionally copy the `serve.sh` file from `examples` into your release folder; this is a tiny script that will spin up a development webserver for you using the `basic-http-server` crate. You can ignore this if you have another way of serving the files.
* Serve the content from your release folder using a webserver or by running `serve.sh`. If you use `serve.sh` then the content will be available at http://localhost:4000.

You should see the sizes of each individual page in your sample file logged to the Javascript console, and the first page in the file will be rendered into an HTML canvas element.

Comments in the `index.html` file explain how to instantiate both the compiled Pdfium and the example
WASM modules and bind them together dynamically at run time. The basic recipe is simple:

* Load and instantiate the Pdfium WASM module first.
* Once Pdfium is instantiated, load and instantiate the WASM module for your compiled Rust application.
* Once your WASM module is instantiated, call `pdfium-render`'s exported `initialize_pdfium_render()` function, passing it both instantiated WASM modules.
* You can now call any Pdfium-related functions exported by your compiled Rust application.

## Interface changes when compiling to WASM

Certain `pdfium-render` functions that access the filesystem are not available when compiling to WASM,
due to the security model present in modern web browsers. Alternative functions for accessing files
over the network are provided instead.

### Interface changes in the `Pdfium` struct

The `Pdfium::load_pdf_from_file()` and `Pdfium::load_pdf_from_reader()` functions are not available
when running in the browser. The `Pdfium::load_pdf_from_byte_slice()` and `Pdfium::load_pdf_from_byte_vec()`
functions are available, and the following additional functions are provided:

* The `Pdfium::load_pdf_from_fetch()` function uses the browser's built-in `fetch()` API
  to download a URL over the network and open it as a PDF document.
* The `Pdfium::load_pdf_from_blob()` function opens a PDF document from the byte data in a Javascript
  `Blob` or `File` object, including `File` objects returned from an `<input type="file">` element.

### Interface changes in the `PdfDocument` struct

The `PdfDocument::save_to_file()` function is not available when running in the browser.
The `PdfDocument::save_to_bytes()` and `PdfDocument::save_to_writer()` functions are
available, and the following additional function is provided:

* The `PdfDocument::save_to_blob()` function returns the byte data for the document as a
  Javascript `Blob` object.

### Interface changes in the `PdfBitmap` struct

The following additional functions are provided during rendering:

* The `PdfBitmap::as_image_data()` function renders directly to a Javascript `ImageData` object,
  ready to display in an HTML `<canvas>` element.
* The `PdfBitmap::as_array()` function renders directly to a Javascript `Uint8Array` object.
  This function avoids a memory allocation and copy required by both `PdfBitmap::as_bytes()`
  and `PdfBitmap::as_image_data()`, making it preferable for situations where performance is paramount.

### Interface changes in the `PdfFonts` struct

The `PdfFonts::load_type1_from_file()` and `PdfFonts::load_true_type_from_file()` functions are
not available when running in the browser. The following additional functions are provided:

* The `PdfFonts::load_type1_from_fetch()` function uses the browser's built-in `fetch()` API
  to download a URL over the network and load it as a Type 1 font.
* The `PdfFonts::load_true_type_from_fetch()` function uses the browser's built-in `fetch()` API
  to download a URL over the network and load it as a TrueType font.
* The `PdfFonts::load_type1_from_blob()` function loads a Type 1 font from the byte data in a
  Javascript `Blob` or `File` object, including `File` objects returned from an `<input type="file">`
  element.
* The `PdfFonts::load_true_type_from_blob()` function loads a TrueType font from the byte data in a
  Javascript `Blob` or `File` object, including `File` objects returned from an `<input type="file">`
  element.

### Interface changes in the `PdfAttachments` struct

The `PdfAttachments::create_attachment_from_file()` function is not available when running in
the browser. The `PdfAttachments::create_attachment_from_bytes()` and
`PdfAttachments::create_attachment_from_reader()` functions are available, and
the following additional functions are provided:

* The `PdfAttachments::create_attachment_from_fetch()` function uses the brower's built-in `fetch()` API
  to download a URL over the network and use it as an embedded attachment.
* The `PdfAttachments::create_attachment_from_blob()` function creates an embedded attachment
  from the byte data in a Javascript `Blob` or `File` object, including `File` objects returned from
  an `<input type="file">` element.

### Interface changes in the `PdfAttachment` struct

The `PdfAttachment::save_to_file()` function is not available when running in the browser.
The `PdfAttachment::save_to_bytes()` and `PdfAttachment::save_to_writer()` functions are
available, and the following additional function is provided:

* The `PdfAttachment::save_to_blob()` function returns the byte data for the attachment as a
  Javascript `Blob` object.
