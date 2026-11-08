#!/usr/bin/env bash

# This support script can be used to run the example for WASM. See:
# https://github.com/ajrcarey/pdfium-render/tree/master/examples

# Serve the application locally using basic-http-server.

cargo install basic-http-server
RUST_LOG=basic_http_server=trace basic-http-server -a 127.0.0.1:4000
