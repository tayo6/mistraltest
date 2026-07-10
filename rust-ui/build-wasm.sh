#!/bin/bash

# Build script for WASM version of MonoDelay-1 Knobs
# Run this from the rust-ui directory

set -e

echo "Building MonoDelay-1 Knobs for WASM..."

# Ensure we're in the right directory
cd "$(dirname "$0")"

# Install wasm target if not already installed
rustup target add wasm32-unknown-unknown || true

# Install wasm-bindgen if not already installed
cargo install wasm-bindgen-cli || true

echo "Building library as WASM..."
cargo build --target wasm32-unknown-unknown --release --lib

echo "Generating JavaScript bindings..."
wasm-bindgen --target web --out-dir ./web/pkg \
    --no-typescript \
    ./target/wasm32-unknown-unknown/release/monodelay_knobs.wasm

echo "Copying WASM file..."
cp ./target/wasm32-unknown-unknown/release/monodelay_knobs.wasm ./web/pkg/

echo "Creating version file..."
echo "const VERSION = '$(git rev-parse --short HEAD)';" > ./web/pkg/version.js

echo ""
echo "WASM build complete!"
echo ""
echo "To test locally:"
echo "  1. Install a local HTTP server: npm install -g http-server"
echo "  2. Run: http-server rust-ui/web -p 8080"
echo "  3. Open: http://localhost:8080"
echo ""
echo "Or use Python's built-in server:"
echo "  python3 -m http.server 8080 --directory rust-ui/web"
