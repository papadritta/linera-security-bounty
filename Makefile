.PHONY: help build test clean deploy serve lint format check-all

help:
	@echo "Linera Security Bounty Platform - Build Commands"
	@echo ""
	@echo "Usage: make [target]"
	@echo ""
	@echo "Targets:"
	@echo "  build       - Build the Linera application (WASM)"
	@echo "  test        - Run all tests"
	@echo "  lint        - Run clippy linter"
	@echo "  format      - Format code with rustfmt"
	@echo "  check-all   - Run tests + lint + format check"
	@echo "  clean       - Clean build artifacts"
	@echo "  deploy      - Deploy to local Linera network"
	@echo "  serve       - Start GraphQL service (port 8080)"
	@echo "  frontend    - Serve frontend (port 3000)"

build:
	@echo "Building Linera application..."
	cd security-bounty && cargo build --release --target wasm32-unknown-unknown

test:
	@echo "Running tests..."
	cd security-bounty && cargo test

lint:
	@echo "Running clippy..."
	cd security-bounty && cargo clippy --all-targets --all-features -- -D warnings

format:
	@echo "Checking code formatting..."
	cd security-bounty && cargo fmt --all -- --check

format-fix:
	@echo "Fixing code formatting..."
	cd security-bounty && cargo fmt --all

check-all: test lint format
	@echo "✅ All checks passed!"

clean:
	@echo "Cleaning build artifacts..."
	cd security-bounty && cargo clean
	rm -rf .linera/

deploy:
	@echo "Deploying to local Linera network..."
	@./scripts/deploy.sh

serve:
	@echo "Starting GraphQL service on port 8080..."
	linera service --port 8080

frontend:
	@echo "Starting frontend server on port 3000..."
	cd frontend && python3 -m http.server 3000
