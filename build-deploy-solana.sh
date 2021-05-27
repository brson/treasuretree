#!/bin/bash

set -e

cargo build-bpf --manifest-path=src/geonft_solana/Cargo.toml
solana program deploy target/deploy/geonft_solana.so
