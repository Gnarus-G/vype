#!/bin/bash
set -e

REPO="gnarus-g/vype"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"

check_runtime_deps() {
    local missing=()
    
    if ! ldconfig -p 2>/dev/null | grep -q "libxdo.so" && ! pacman -Qi libxdo &>/dev/null; then
        missing+=("libxdo")
    fi
    
    if ! ldconfig -p 2>/dev/null | grep -q "libasound.so" && ! pacman -Qi alsa-lib &>/dev/null; then
        missing+=("alsa-lib")
    fi
    
    if [ ${#missing[@]} -gt 0 ]; then
        echo "Error: Missing runtime dependencies:" >&2
        for dep in "${missing[@]}"; do
            echo "  - $dep" >&2
        done
        echo "" >&2
        echo "Install with:" >&2
        if [ -f /etc/arch-release ]; then
            echo "  sudo pacman -S xdotool alsa-lib" >&2
        else
            echo "  sudo apt install libxdo-dev libasound2-dev" >&2
        fi
        exit 1
    fi
}

detect_distro() {
    if [ -f /etc/arch-release ]; then
        echo "arch"
    elif [ -f /etc/debian_version ] || [ -f /etc/ubuntu-version ]; then
        echo "ubuntu"
    else
        echo "ubuntu"
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
DISTRO="${VYPE_DISTRO:-$(detect_distro)}"
ARCH="x86_64"

check_runtime_deps

echo "Installing vype $VERSION ($GPU_BACKEND backend, $DISTRO)..."

TEMP_DIR=$(mktemp -d)
trap "rm -rf $TEMP_DIR" EXIT

TARBALL="vype-$VERSION-$ARCH-$DISTRO-$GPU_BACKEND.tar.gz"
URL="https://github.com/$REPO/releases/download/$VERSION/$TARBALL"

echo "Downloading $URL..."
curl -fsSL "$URL" -o "$TEMP_DIR/$TARBALL"

mkdir -p "$INSTALL_DIR"
tar -xzf "$TEMP_DIR/$TARBALL" -C "$INSTALL_DIR"
chmod +x "$INSTALL_DIR/vype"

SYSTEMD_DIR="$HOME/.config/systemd/user"
mkdir -p "$SYSTEMD_DIR"

cat > "$SYSTEMD_DIR/vype.service" << EOF
[Unit]
Description=Vype speech-to-text keyboard

[Service]
ExecStart=%h/.local/bin/vype
Restart=on-failure
Environment=VYPE_GPU=$GPU_BACKEND

[Install]
WantedBy=default.target
EOF

echo ""
echo "Installed vype to $INSTALL_DIR/vype"
echo "Systemd service installed to $SYSTEMD_DIR/vype.service"
echo ""
echo "To enable and start the service:"
echo "  systemctl --user enable vype"
echo "  systemctl --user start vype"
echo ""
echo "Make sure $INSTALL_DIR is in your PATH."
echo "Add to PATH: export PATH=\"\$PATH:$INSTALL_DIR\""