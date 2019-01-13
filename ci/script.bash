#!/usr/bin/env bash

set -exo pipefail

main() {
    rustup target add "$TARGET"
    cargo build --target "$TARGET" --verbose
    cargo test --target "$TARGET" --verbose
}

main
