#!/bin/sh

# strict mode
set -euo pipefail

# - KEEP this in sync with .github/workflows/main.yml
# - BUT, this needs to undo any directory change (`cd`) done in any of the GitHub Actions step

echo DEFAULT Rust TOOLCHAIN

echo
echo "DOC"
cargo doc --no-deps --quiet

echo
echo "CARGO TEST"
cargo test

echo
echo "CARGO BUILD (release) IN positive_tests/"
cd positive_tests
cargo build --release
cd - >/dev/null

echo
echo "CARGO TEST (debug) IN negative_tests_runner/"
cd negative_tests_runner
cargo test
cd - >/dev/null
