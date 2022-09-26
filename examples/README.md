# Examples

Simple examples demonstrating how to use `pdfium-render` on both native and WASM compilation targets.

* `annotations.rs`: iterates over every annotation on every page in `test/annotation-test.pdf`, displaying information about each annotation. Run this example via `cargo run --example annotations`.
* `attachments.rs`: generates a new document by creating file attachments, saving the new document to `test/attachments.pdf`. Run this example via `cargo run --example attachments`. 
* `chars.rs`: iterates over the individual characters in a text object to determine the bounding boxes of each word in the text object. Run this example via `cargo run --example chars`.
* `descenders.rs`: iterates over the individual characters in a text object, measuring which have glyph shapes that descend beneath the text object's font baseline. Run this example via `cargo run --example descenders`.
* `concat.rs`: generates a new document by concatenating pages from `test/export-test.pdf`, `test/form-test.pdf`, and `test/text-test.pdf` together, saving the new document to `test/concat-test.pdf`. Run this example via `cargo run --example concat`.
* `create.rs`: generates a new document by placing text objects onto a blank page, saving the new document to `test/create-test.pdf`. Run this example via `cargo run --example create`.
* `export.rs`: exports the individual pages in `test/export-test.pdf` to JPGs in the working directory. Run this example via `cargo run --example export`. The example will attempt to bind to a copy of Pdfium in the working directory, falling back to the system-bundled library if local loading fails. See comments in the source file for more details.
* `fonts.rs`: displays information about the 14 built-in PDF fonts retrieved from Pdfium. Run this example via `cargo run --example fonts`.
* `form.rs`: exports the individual pages in `test/form-test.pdf` to JPGs in the working directory. The sample PDF includes pre-filled form fields, the values of which should also be rendered. Run this example via `cargo run --example form`.
* `image.rs`: generates a new document by placing image objects onto a blank page, saving the new document to `test/image-test.pdf`. Run this example via `cargo run --example image`.
* `image_extract.rs`: extracts and outputs the images on each page in `test/image-test.pdf` to files. Run this example via `cargo run --example image_extract`.
* `objects.rs`: iterates over every page object on every page in `test/export-test.pdf`, displaying information about each page object. Run this example via `cargo run --example objects`.
* `path.rs`: generates a new document by placing path objects onto a blank page, saving the new document to `test/path-test.pdf`. Run this example via `cargo run --example path`.
* `signatures.rs`: iterates over every digital signature in `test/signatures.pdf`, displaying information on about each signature. Run this example via `cargo run --example signatures`.
* `text_extract.rs`: extracts and outputs the text on each page in `test/text-test.pdf` to the console. Run this example via `cargo run --example text_extract`.
* `thread_safe.rs`: explains in comments `pdfium-render`'s approach to ensuring thread-safe access to Pdfium, and demonstrates using a parallel iterator to process multiple rendering tasks on separate threads. Run this example via `cargo run --example thread_safe`.
* `tile.rs`: generates a new document by tiling pages from `test/export-test.pdf`, `test/form-test.pdf`, and `test/text-test.pdf`, saving the new document to `test/tile-test.pdf`. Run this example via `cargo run --example tile`.
* `wasm.rs`: demonstrates `pdfium-render` running in a browser. This requires some manual bundling of the correct resources; read on.
* `watermark.rs`: adds a watermark to each page in a previously-generated document, saving the watermarked document to `test/watermark-test.pdf`. Run this example via `cargo run --example watermark`.

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
