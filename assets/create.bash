#!/usr/bin/env bash

ASSETS_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cargo run --release --bin reser-generate-assets -- --assets-dir "$ASSETS_DIR"
