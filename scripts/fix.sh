#!/usr/bin/env bash

set -euo pipefail

section() {
    echo
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "$1"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
}

section "Formatting"
cargo fmt --all

section "Applying Clippy Fixes"
cargo clippy \
    --fix \
    --workspace \
    --all-features \
    --allow-dirty \
    --allow-staged

section "Clippy Verification"
cargo clippy \
    --workspace \
    --all-targets \
    --all-features \
    -- -D warnings

section "Security Audit"
cargo audit

section "Dependency Policy Check"
cargo deny check

echo
echo "✓ All fixes applied successfully"