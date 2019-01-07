#!/usr/bin/env bash

ASSET_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cargo run --release --bin reser-generate-assets -- --asset-dir "$ASSET_DIR"
