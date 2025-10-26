#!/bin/bash
set -e

echo "════════════════════════════════════════════════════════════"
echo " Deploying Linera Security Bounty Platform"
echo "════════════════════════════════════════════════════════════"
echo ""

# Check if Linera network is running
if ! pgrep -f "linera-proxy" > /dev/null; then
    echo "Network is not running."
    echo "Start it first: make network-up"
    exit 1
fi

# Check if environment variables are set
if [ -z "$LINERA_WALLET" ]; then
    echo "Environment variables are not set."
    echo "Run 'make network-up' and copy the export commands."
    exit 1
fi

echo "Network is running"
echo "Environment is configured"
echo ""

# Build the application
echo "Building the app..."
cd security-bounty
cargo build --release --target wasm32-unknown-unknown
echo "Build complete"
echo ""

# Publish and create application
echo "Publishing to network..."
APP_OUTPUT=$(linera project publish-and-create 2>&1)
APP_ID=$(echo "$APP_OUTPUT" | grep -E '^[a-f0-9]{64}$' | head -1)
echo "$APP_OUTPUT"
cd ..

# Update frontend automatically
echo ""
echo "Updating frontend endpoint..."
CHAIN_ID=$(linera wallet show | grep "Admin Chain" -A 1 | grep "Chain ID" | awk '{print $3}')
sed -i '' "s|chains/[^/]*/applications/[^']*|chains/${CHAIN_ID}/applications/${APP_ID}|g" frontend/index.html
echo "Frontend updated"
echo "Chain: ${CHAIN_ID}"
echo "App: ${APP_ID}"
echo ""

echo "════════════════════════════════════════════════════════════"
echo " Deployment complete"
echo "════════════════════════════════════════════════════════════"
echo ""
echo "Next steps:"
echo " Terminal 1: make serve"
echo " Terminal 2: make frontend"
echo " Browser: http://localhost:3000"