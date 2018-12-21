#!/bin/sh -e

echo "compile wasm"
cargo build --target wasm32-unknown-unknown --release

echo "gc wasm"
RUST_BACKTRACE=1 wasm-gc target/wasm32-unknown-unknown/release/wasm_audioworklet_synth.wasm -o wasm/wasm_audioworklet_synth.wasm

echo "finish!"
