#!/usr/bin/env bash
set -euo pipefail

echo ">>> cargo fmt --all -- --check"
cargo fmt --all -- --check

echo ">>> cargo lint"
cargo lint

echo ">>> cargo test-all"
cargo test-all

echo "✅ Local quality gate passed"
