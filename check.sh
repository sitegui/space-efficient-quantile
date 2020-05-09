#!/bin/bash -e

echo "=== cargo clippy ==="
# This is required because clippy does not rebuild by default
# See https://github.com/rust-lang/rust-clippy/issues/2604
touch src/lib.rs
cargo clippy --all-targets --all-features -- -D clippy::all