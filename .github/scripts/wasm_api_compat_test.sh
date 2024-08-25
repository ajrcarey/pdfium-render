#!/bin/bash

# $1 contains the pdfium_* API feature to check

cp examples/Cargo.toml examples/Cargo.bak

features='"image", "pdfium_use_skia", "pdfium_enable_xfa", "pdfium_enable_v8", "'$1'"'
crate_path='"../"'
sed "s|pdfium-render = .*|pdfium-render = { path = ${crate_path}, default-features = false, features = [${features}] }|g" examples/Cargo.bak > examples/Cargo.toml

wasm-pack build examples/ --target no-modules
result=$?

rm examples/Cargo.toml
mv examples/Cargo.bak examples/Cargo.toml

exit $result