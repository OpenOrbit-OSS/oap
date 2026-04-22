#!/bin/bash
# OAP Diagnostic Suite - OpenOrbit Initiative

echo "🔍 RUNNING SYSTEM DIAGNOSTICS..."

# 1. Check Rust Compiler
echo "[1/3] Checking Toolchain..."
cargo --version || { echo "❌ Rust not found"; exit 1; }

# 2. Run Unit Tests (Penting untuk orbit mechanics)
echo "[2/3] Running Unit Tests..."
cargo test --lib || { echo "❌ Tests Failed!"; exit 1; }

# 3. Code Linting (Check for messy code)
echo "[3/3] Checking Code Quality (Clippy)..."
cargo clippy -- -D warnings || { echo "❌ Code quality check failed"; exit 1; }

echo "✅ ALL SYSTEMS GO. OAP is flight-ready!"