# AGENTS.md

## Development Commands

```bash
# Build all components (default CPU backend)
cargo build

# Build client only
cargo build -p vypec

# Build daemon only
cargo build -p vyped

# Build daemon with Vulkan backend
cargo build -p vyped --no-default-features --features vulkan

# Build daemon with CUDA backend
cargo build -p vyped --no-default-features --features cuda

# Test (fast, without whisper)
cargo test
```

## Two-Process Architecture

Vype now uses a two-process architecture with zero-copy IPC via iceoryx2:

- **vypec** (client): sends control commands (start/stop/toggle/partial) to vyped
- **vyped** (daemon): handles PTT key detection, audio capture, transcription, and typing

### Running

Both processes must run simultaneously:

```bash
# Terminal 1: Start daemon (with transcription + key handling)
cargo run -p vyped --no-default-features --features vulkan -- -k F9

# Terminal 2: Send commands on demand
cargo run -p vypec -- toggle
cargo run -p vypec -- start
cargo run -p vypec -- stop
```

### Communication

- **Service**: `vype/ptt_events`
- **Message**: `PttEvent` (StartRecording, StopRecording, PartialTranscribe, ToggleRecording)
- **Direction**: vypec → vyped

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

Service builds and runs `target/release/vyped -s large -k F9` with Vulkan.

### Production Service (install.sh)

Installed by `install.sh` to `~/.config/systemd/user/vype.service`. Uses pre-built binary from GitHub releases.
