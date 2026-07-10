#!/bin/bash

# Deployment script for MonoDelay-1 Knobs WASM
# This script builds the WASM module and prepares it for deployment

set -e

echo "=========================================="
echo "MonoDelay-1 Knobs WASM Deployment"
echo "=========================================="

cd "$(dirname "$0")"

# Step 1: Install dependencies
echo ""
echo "Step 1: Installing dependencies..."
rustup target add wasm32-unknown-unknown || true
cargo install wasm-bindgen-cli || true

# Step 2: Build the library as WASM
echo ""
echo "Step 2: Building WASM module..."
cargo build --target wasm32-unknown-unknown --release --lib

# Step 3: Generate JavaScript bindings
echo ""
echo "Step 3: Generating JavaScript bindings..."
wasm-bindgen --target web --out-dir ./web/pkg \
    --no-typescript \
    ./target/wasm32-unknown-unknown/release/monodelay_knobs.wasm

# Step 4: Copy WASM file
echo ""
echo "Step 4: Copying WASM file..."
cp ./target/wasm32-unknown-unknown/release/monodelay_knobs.wasm ./web/pkg/

# Step 5: Create version file
echo ""
echo "Step 5: Creating version file..."
echo "const VERSION = '$(git rev-parse --short HEAD)';" > ./web/pkg/version.js
echo "const BUILD_DATE = '$(date -u +"%Y-%m-%dT%H:%M:%SZ")';" >> ./web/pkg/version.js

# Step 6: Verify files
echo ""
echo "Step 6: Verifying deployment files..."
echo "Files in web/pkg/:"
ls -lh ./web/pkg/

echo ""
echo "=========================================="
echo "Deployment complete!"
echo "=========================================="
echo ""
echo "To test locally:"
echo "  1. python3 -m http.server 8080 --directory rust-ui/web"
echo "  2. Open http://localhost:8080 in your browser"
echo ""
echo "To deploy to GitHub Pages:"
echo "  1. Push to main branch"
echo "  2. GitHub Actions will automatically build and deploy"
echo "  3. Access at: https://<username>.github.io/MonoDelay-1/rust-ui/web/"
echo ""
