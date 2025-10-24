#!/bin/bash
set -e

echo "════════════════════════════════════════════════════════════"
echo "  Deploying Linera Security Bounty Platform"
echo "════════════════════════════════════════════════════════════"
echo ""

# Check if Linera network is running
if ! pgrep -f "linera-proxy" > /dev/null; then
    echo "Oops! The network isn't running."
    echo ""
    echo "Start it first:"
    echo "  make network-up"
    echo ""
    echo "Then export the variables and try again."
    exit 1
fi

# Check if environment variables are set
if [ -z "$LINERA_WALLET" ]; then
    echo "════════════════════════════════════════════════════════════"
    echo "Wait! Environment variables aren't set."
    echo "════════════════════════════════════════════════════════════"
    echo ""
    echo "Did you run 'make network-up'?"
    echo ""
    echo "After that command, you should see 3 export commands like:"
    echo "  export LINERA_WALLET=/tmp/.tmpXXX/wallet.json"
    echo "  export LINERA_KEYSTORE=/tmp/.tmpXXX/keystore"
    echo "  export LINERA_STORAGE=rocksdb:/tmp/.tmpXXX/storage"
    echo ""
    echo "Copy those EXACT lines and paste them in this terminal."
    echo "Then run 'make deploy' again."
    echo ""
    exit 1
fi

echo "Network is running ✓"
echo "Environment is configured ✓"
echo ""

# Build the application
echo "Building the app..."
cd security-bounty
cargo build --release --target wasm32-unknown-unknown
echo "Build complete ✓"
echo ""

# Publish and create application
echo "Publishing to network..."
APP_ID=$(linera project publish-and-create)
cd ..

echo ""
echo "════════════════════════════════════════════════════════════"
echo "  Deployment complete!"
echo "════════════════════════════════════════════════════════════"
echo ""
echo "Your app ID: $APP_ID"
echo ""
echo "Next steps:"
echo "  1. Start GraphQL (in one terminal):"
echo "     make serve"
echo ""
echo "  2. Start frontend (in another terminal):"
echo "     make frontend"
echo ""
echo "  3. Open your browser:"
echo "     http://localhost:3000"