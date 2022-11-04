#!/bin/bash
TARGET="${CARGO_TARGET_DIR:-target}"
set -e

cargo fmt

cargo build --all --target wasm32-unknown-unknown --release
mkdir -p res
cp "$TARGET"/wasm32-unknown-unknown/release/*.wasm ./res/
