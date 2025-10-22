#!/bin/bash
set -e

echo "🧪 Running Linera Security Bounty Tests"
echo ""

cd security-bounty

echo "1️⃣ Running cargo test..."
cargo test

echo ""
echo "2️⃣ Running clippy..."
cargo clippy --all-targets --all-features -- -D warnings

echo ""
echo "3️⃣ Checking formatting..."
cargo fmt --all -- --check

echo ""
echo "✅ All tests passed!"
