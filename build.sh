#!/bin/bash
set -e

package_name=$(cat Cargo.toml | sed -ne 's/^name\s\+=\s\+"\(.*\)"$/\1/p' | sed -e 's/-/_/g')
RUSTFLAGS="-C link-arg=-s --remap-path-prefix ${CARGO_HOME:-$HOME/.cargo}=/usr/local/cargo" cargo build --target wasm32-unknown-unknown --release
mkdir -p ./res
cp target/wasm32-unknown-unknown/release/${package_name}.wasm ./res/${package_name}.wasm
touch -r target/wasm32-unknown-unknown/release/${package_name}.wasm ./res/${package_name}.wasm
