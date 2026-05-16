#!/usr/bin/env bash

set -euo pipefail

section() {
    echo
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "$1"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
}

section "Formatting Check"
cargo fmt --all -- --check

section "Clippy"
cargo clippy \
    --workspace \
    --all-targets \
    --all-features \
    -- -D warnings

section "Tests"
cargo test --workspace --all-features

section "Build"
cargo build --workspace --all-targets --all-features

section "Security Audit"
cargo audit --ignore RUSTSEC-2023-0071

section "Dependency Policy Check"
cargo deny check

echo
echo "✓ All verification checks passed"