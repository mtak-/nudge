#!/bin/bash

set -ex

cd "$(dirname "$0")"/..

export RUSTFLAGS="-D warnings"

cargo build --no-default-features --all
cargo build --no-default-features --features std --all

if [[ "$TRAVIS_RUST_VERSION" == "nightly" ]]; then
    cargo build --no-default-features --features nightly --all
    cargo build --no-default-features --features std,nightly --all
fi
