#!/bin/bash
set -e

echo "🚀 Deploying Linera Security Bounty Platform"
echo ""

# Check if Linera network is running
if ! pgrep -f "linera-proxy" > /dev/null; then
    echo "❌ Linera network not running. Start with: linera net up"
    exit 1
fi

# Check if environment variables are set
if [ -z "$LINERA_WALLET" ]; then
    echo "❌ LINERA_WALLET not set!"
    echo ""
    echo "Run 'linera net up' and export the environment variables it provides:"
    echo "  export LINERA_WALLET=..."
    echo "  export LINERA_KEYSTORE=..."
    echo "  export LINERA_STORAGE=..."
    exit 1
fi

# Build the application
echo "📦 Building application..."
cd security-bounty
cargo build --release --target wasm32-unknown-unknown

# Publish and create application
echo "🌐 Publishing to Linera network..."
linera project publish-and-create

cd ..

echo ""
echo "✅ Deployment complete!"
echo ""
echo "Next steps:"
echo "  1. Start service: make serve"
echo "  2. Start frontend: make frontend"
echo "  3. Open browser: http://localhost:3000"