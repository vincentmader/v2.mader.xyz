#!/bin/sh

cd src/mxyz-client && cargo build --target wasm32-unknown-unknown --release
cd ../..

PATH_TO_OUT_PKG="src/mxyz-server/static/pkg"
PATH_TO_WASM="src/target/wasm32-unknown-unknown/release/mxyz_client.wasm"
mkdir -p "$PATH_TO_OUT_PKG"
wasm-bindgen --target web --out-dir "$PATH_TO_OUT_PKG" "$PATH_TO_WASM"

cd src && cargo run --release
