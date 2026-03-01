# AGENTS.md

## Development Commands

```bash
# Build (no transcription feature, fast)
cargo build

# Build with Vulkan backend
cargo build -r --features vulkan

# Build with CUDA backend
cargo build -r --features cuda

# Test (fast, without whisper)
cargo test
```

## Systemd Services

### vype-dev.service

Development service that builds and runs release binary with Vulkan backend.

```bash
# Install/update service
cp vype-dev.service ~/.config/systemd/user/
systemctl --user daemon-reload

# Enable (autostart on login)
systemctl --user enable vype-dev

# Start now
systemctl --user start vype-dev

# Restart (rebuilds and runs)
systemctl --user restart vype-dev

# View logs
journalctl --user -u vype-dev -f
```

Service runs `target/release/vype -s large` with Vulkan after building.

### Production Service (install.sh)

Installed by `install.sh` to `~/.config/systemd/user/vype.service`. Uses pre-built binary from GitHub releases.
