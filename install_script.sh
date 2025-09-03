#!/bin/bash

# YourMom Variable Fixer Installation Script
# This script builds and installs the yourmom-fixer CLI tool

set -e

echo "🔥 YourMom Variable Fixer Installation 🔥"
echo "========================================"

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust is not installed!"
    echo "Please install Rust from https://rustup.rs/ and try again."
    exit 1
fi

# Check Rust version
rust_version=$(rustc --version | cut -d' ' -f2)
echo "✅ Found Rust version: $rust_version"

# Build the project
echo "🔨 Building yourmom-fixer..."
cargo build --release

if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
else
    echo "❌ Build failed!"
    exit 1
fi

# Determine installation directory
if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS
    INSTALL_DIR="/usr/local/bin"
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    # Linux
    if [ -w "/usr/local/bin" ]; then
        INSTALL_DIR="/usr/local/bin"
    elif [ -w "$HOME/.local/bin" ]; then
        INSTALL_DIR="$HOME/.local/bin"
        mkdir -p "$INSTALL_DIR"
    else
        INSTALL_DIR="$HOME/bin"
        mkdir -p "$INSTALL_DIR"
    fi
else
    # Windows or other
    INSTALL_DIR="$HOME/bin"
    mkdir -p "$INSTALL_DIR"
fi

echo "📦 Installing to: $INSTALL_DIR"

# Copy the binary
cp target/release/yourmom-fixer "$INSTALL_DIR/"

if [ $? -eq 0 ]; then
    echo "✅ Installation successful!"
    
    # Check if the install directory is in PATH
    if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
        echo ""
        echo "⚠️  WARNING: $INSTALL_DIR is not in your PATH"
        echo "Add this line to your shell profile (~/.bashrc, ~/.zshrc, etc.):"
        echo "export PATH=\"$INSTALL_DIR:\$PATH\""
        echo ""
    fi
    
    echo "🎉 yourmom-fixer is now installed!"
    echo ""
    echo "Try it out:"
    echo "  yourmom-fixer --help"
    echo "  yourmom-fixer --dry-run *.py"
    echo ""
    echo "Happy variable fixing! 🚀"
else
    echo "❌ Installation failed!"
    exit 1
fi
