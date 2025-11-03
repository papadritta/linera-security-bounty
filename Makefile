.PHONY: help build test clean deploy serve frontend lint format format-fix check-all network-up network-down network-status check-env

# Default target
.DEFAULT_GOAL := help

help:
	@echo "════════════════════════════════════════════════════════════"
	@echo "  Linera Security Bounty Platform"
	@echo "════════════════════════════════════════════════════════════"
	@echo ""
	@echo "Hey! Here's what you can do:"
	@echo ""
	@echo "Build stuff:"
	@echo "  make build       → Compile the application"
	@echo "  make test        → Run tests"
	@echo "  make lint        → Check code quality"
	@echo "  make format      → Check code style"
	@echo "  make format-fix  → Fix code style automatically"
	@echo "  make check-all   → Run everything (tests + lint + format)"
	@echo "  make clean       → Clean up build files"
	@echo ""
	@echo "Network stuff:"
	@echo "  make network-up     → Start local network"
	@echo "  make network-down   → Stop local network"
	@echo "  make network-status → Check if network is running"
	@echo "  make check-env      → Check if environment variables are set"
	@echo ""
	@echo "Deploy:"
	@echo "  make deploy → Deploy your app to the network"
	@echo ""
	@echo "Run services:"
	@echo "  make serve    → Start GraphQL (port 8080)"
	@echo "  make frontend → Start web UI (port 3000)"
	@echo ""
	@echo "Quick start (you need 3 terminals):"
	@echo "  Terminal 1:"
	@echo "    make network-up (keep running, copy the export commands)"
	@echo "  Terminal 2:"
	@echo "    (paste the 3 export commands)"
	@echo "    make check-env"
	@echo "    make build"
	@echo "    make test"
	@echo "    make deploy"
	@echo "    make serve (keep running)"
	@echo "  Terminal 3:"
	@echo "    make frontend"
	@echo "  Then: http://localhost:3000"

build:
	@echo "Building your app..."
	@cd security-bounty && cargo build --release --target wasm32-unknown-unknown
	@echo "Done!"

test:
	@echo "Running tests..."
	@cd security-bounty && cargo test
	@echo "All tests passed!"

lint:
	@echo "Checking code quality..."
	@cd security-bounty && cargo clippy --all-targets --all-features -- -D warnings
	@echo "Looking good!"

format:
	@echo "Checking code style..."
	@cd security-bounty && cargo fmt --all -- --check
	@echo "Code looks great!"

format-fix:
	@echo "Fixing code style..."
	@cd security-bounty && cargo fmt --all
	@echo "Fixed!"

check-all: test lint format
	@echo ""
	@echo "════════════════════════════════════════════════════════════"
	@echo "  Everything looks good! You're ready to deploy."
	@echo "════════════════════════════════════════════════════════════"

clean:
	@echo "Cleaning up..."
	@cd security-bounty && cargo clean
	@rm -rf .linera/ || true
	@echo "All clean!"

network-up:
	@echo "Starting local network in BLOCKING mode..."
	@echo ""
	@echo "IMPORTANT: This command will block this terminal!"
	@echo "The network MUST keep running."
	@echo ""
	@echo "After starting, you'll see 3 export commands."
	@echo "Copy them, then:"
	@echo "  1. Open a NEW terminal"
	@echo "  2. Paste the export commands in the new terminal"
	@echo "  3. Continue all other commands in that new terminal"
	@echo ""
	@echo "Keep THIS terminal running (don't press Ctrl+C)."
	@echo ""
	@linera net up

network-down:
	@echo "Stopping network..."
	@linera net down || true
	@echo "Network stopped"

network-status:
	@echo "Checking network..."
	@if pgrep -f "linera-proxy" > /dev/null; then \
		echo "Network is running"; \
	else \
		echo "Network is NOT running"; \
		echo "Start it with: make network-up"; \
	fi

check-env:
	@echo "Checking environment variables..."
	@if [ -z "$$LINERA_WALLET" ]; then \
		echo ""; \
		echo "LINERA_WALLET - NOT SET"; \
		echo "LINERA_KEYSTORE - NOT SET"; \
		echo "LINERA_STORAGE - NOT SET"; \
		echo ""; \
		echo "You need to export these variables!"; \
		echo "Run 'make network-up' and copy the export commands."; \
	else \
		echo ""; \
		echo "LINERA_WALLET - $$LINERA_WALLET"; \
		echo "LINERA_KEYSTORE - $$LINERA_KEYSTORE"; \
		echo "LINERA_STORAGE - $$LINERA_STORAGE"; \
		echo ""; \
		echo "All set! You can run 'make deploy' now."; \
	fi

deploy:
	@echo "Deploying to network..."
	@./scripts/deploy.sh

serve:
	@echo "Starting GraphQL service on http://localhost:8080"
	@echo "Press Ctrl+C to stop"
	@linera service --port 8080

frontend:
	@echo "Starting frontend on http://localhost:3000"
	@echo "Press Ctrl+C to stop"
	@cd frontend && python3 -m http.server 3000