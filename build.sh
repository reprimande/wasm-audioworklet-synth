#!/bin/sh -e

echo "compile wasm"
cargo build --target wasm32-unknown-unknown --release

echo "gc wasm"
RUST_BACKTRACE=1 wasm-gc target/wasm32-unknown-unknown/release/wasm_audioworklet.wasm -o wasm/wasm_audioworklet.wasm

echo "finish!"
