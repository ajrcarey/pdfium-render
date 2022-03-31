#!/usr/bin/env bash

# Serve the application locally using basic-http-server.

cargo install basic-http-server
RUST_LOG=basic_http_server=trace basic-http-server -a 127.0.0.1:4000
