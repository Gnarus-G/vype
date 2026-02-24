#!/bin/bash
set -e

REPO="gnarus-g/vype"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"

check_runtime_deps() {
    local missing=()
    
    if ! ldconfig -p | grep -q "libxdo.so"; then
        missing+=("libxdo-dev (or libxdo)")
    fi
    
    if ! ldconfig -p | grep -q "libasound.so"; then
        missing+=("libasound2-dev (or libasound2)")
    fi
    
    if [ ${#missing[@]} -gt 0 ]; then
        echo "Error: Missing runtime dependencies:" >&2
        for dep in "${missing[@]}"; do
            echo "  - $dep" >&2
        done
        echo "" >&2
        echo "Install with:" >&2
        echo "  sudo apt install libxdo-dev libasound2-dev" >&2
        exit 1
    fi
}

get_latest_release() {
    curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/'
}

detect_gpu() {
    if command -v nvidia-smi &>/dev/null; then
        echo "cuda"
    elif command -v vulkaninfo &>/dev/null || [ -d /usr/share/vulkan ]; then
        echo "vulkan"
    else
        echo "vulkan"
    fi
}

VERSION="${1:-$(get_latest_release)}"
GPU_BACKEND="${VYPE_GPU:-$(detect_gpu)}"
ARCH="x86_64-linux"

check_runtime_deps

echo "Installing vype $VERSION ($GPU_BACKEND backend)..."

TEMP_DIR=$(mktemp -d)
trap "rm -rf $TEMP_DIR" EXIT

TARBALL="vype-$VERSION-$ARCH-$GPU_BACKEND.tar.gz"
URL="https://github.com/$REPO/releases/download/$VERSION/$TARBALL"

echo "Downloading $URL..."
curl -fsSL "$URL" -o "$TEMP_DIR/$TARBALL"

mkdir -p "$INSTALL_DIR"
tar -xzf "$TEMP_DIR/$TARBALL" -C "$INSTALL_DIR"
chmod +x "$INSTALL_DIR/vype"

echo ""
echo "Installed vype to $INSTALL_DIR/vype"
echo ""
echo "Make sure $INSTALL_DIR is in your PATH."
echo "Add to PATH: export PATH=\"\$PATH:$INSTALL_DIR\""