#!/bin/bash

set -e

wasm-pack build --target=web src/geonft_wasm
cargo build-bpf --manifest-path=src/geonft_solana/Cargo.toml
cargo build -p geonft_web
cargo build -p geonft_sync
