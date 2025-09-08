# Bad Variable Fixer Makefile

.PHONY: build release install clean test fmt clippy help

# Default target
help:
	@echo "🔥 Bad Variable Fixer Build System 🔥"
	@echo "Available targets:"
	@echo "  build    - Build debug version"
	@echo "  release  - Build optimized release version"
	@echo "  install  - Build and install to system PATH"
	@echo "  clean    - Clean build artifacts"
	@echo "  test     - Run tests"
	@echo "  fmt      - Format code"
	@echo "  clippy   - Run clippy lints"
	@echo "  package  - Create distribution packages"

# Build debug version
build:
	@echo "🔨 Building debug version..."
	cargo build

# Build release version
release:
	@echo "🚀 Building release version..."
	cargo build --release
	@echo "✅ Release binary available at: target/release/yourmom-fixer"

# Install to system
install: release
	@echo "📦 Installing yourmom-fixer..."
	@chmod +x install.sh
	@./install.sh

# Clean build artifacts
clean:
	@echo "🧹 Cleaning..."
	cargo clean

# Run tests
test:
	@echo "🧪 Running tests..."
	cargo test

# Format code
fmt:
	@echo "✨ Formatting code..."
	cargo fmt

# Run clippy
clippy:
	@echo "📎 Running clippy..."
	cargo clippy -- -D warnings

# Create distribution packages
package: release
	@echo "📦 Creating distribution packages..."
	@mkdir -p dist
	@cp target/release/bad_variable_changer dist/
	@cp README.md dist/
	@cp install.sh dist/
	@cd dist && tar -czf bad_variable_changer-linux.tar.gz bad_variable_changer README.md install.sh
	@echo "✅ Distribution package created: dist/bad_variable_changer-linux.tar.gz"

# Development workflow
dev: fmt clippy test build
	@echo "🎯 Development checks complete!"

# CI workflow
ci: fmt clippy test release
	@echo "✅ CI checks passed!"
