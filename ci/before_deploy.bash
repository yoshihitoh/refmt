#!/usr/bin/env bash

set -exo pipefail

make_artifacts() {
    cargo build --target "$TARGET" --release
}

make_tarball() {
    local tmp_dir="$(mktemp -d)"
    local name="${PROJECT_NAME}-${TRAVIS_TAG}-${TARGET}"
    local staging="$tmp_dir/$name"
    mkdir -p "$staging/complete"

    local out_dir="$(pwd)/deployment"
    mkdir -p "$out_dir"

    cp "target/$TARGET/release/refmt" "$staging/refmt"
    strip "$staging/refmt"
    cp {README.md,LICENSE} "$staging"

    (cd "$tmp_dir" && tar czf "$out_dir/$name.tar.gz" "$name")
    rm -rf "$tmp_dir"
}

main() {
    make_artifacts
    make_tarball
}

main
