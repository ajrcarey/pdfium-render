# Examples

Simple examples demonstrating how to use `pdfium-render` on both native and WASM compilation targets.

* `export.rs`: exports the individual pages in test/test.pdf to JPGs in the working directory. Run this example via `cargo run --example export`. The example will attempt to bind to a copy of `libpdfium.so` in the working directory, falling back to the system-bundled library if local loading fails. See comments in the source file for more details.
* `wasm.rs`: demonstrates pdfium running in the browser. This requires some manual bundling of the correct resources; read on.

## Bundling for WASM

Since `pdfium-render` does not include `pdfium` itself, an external pre-packaged WASM build of `pdfium` is required. A suitable build is available from https://github.com/paulo-coutinho/pdfium-lib/releases.

* Build the WASM module for the sample: `cargo install wasm-pack && wasm-pack build examples/ --target no-modules`. This creates a WASM module and supporting Javascript files in `examples/pkg`.  
* Copy the `pdfium_render_wasm_example.js` and `pdfium_render_wasm_example_bg.wasm` files from `examples/pkg/` into a release folder.
* Download the pre-packaged WASM build from https://github.com/paulo-coutinho/pdfium-lib/releases and extract the `release/node/pdfium.js` and `release/node/pdfium.wasm` files into your release folder.
* Copy the `index.html` and `pdfium_render.js` files from `examples` into your release folder.
* Optionally copy the `serve.sh` file from `examples` into your release folder; this is a tiny script that will spin up a Python webserver for you. You can ignore this if you have another way of serving the files.
* Serve the content from your release folder using a webserver or by running `serve.sh`. If you use `serve.sh`, then the content will be available at http://0.0.0.0:4000.

You should see the sizes of each individual page in test/test.pdf logged to the Javascript console, and a single selected page from the test document will be rendered into an HTML canvas element.
