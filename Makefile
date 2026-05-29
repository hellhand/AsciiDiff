.PHONY: dev build test test-rust test-e2e install clean test-repo

# Install all dependencies
install:
	cd frontend && npm install
	cd frontend/src-tauri && cargo fetch

# Run in development mode (Tauri window + hot reload)
dev:
	cd frontend && npm run tauri dev

# Build production binary
build:
	cd frontend && npm run tauri build

# Run all tests
test: test-rust test-e2e

# Rust unit tests
test-rust:
	cd frontend/src-tauri && cargo test

# Playwright e2e tests
test-e2e:
	cd frontend && npx playwright test

# Frontend-only dev server (no Tauri backend)
dev-web:
	cd frontend && npm run dev

# Create test repository for manual testing
test-repo:
	./scripts/create-test-repo.sh

# Clean build artifacts
clean:
	cd frontend/src-tauri && cargo clean
	rm -rf frontend/dist frontend/node_modules/.vite
