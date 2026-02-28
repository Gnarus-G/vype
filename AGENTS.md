# Development Agents

## Systemd User Service Setup

### vype-dev.service

A systemd user service for development, located at `vype-dev.service` in the project root.

**Features:**
- Auto-builds with `cargo build --features vulkan` before starting
- Uses medium model by default (`-s medium`)
- Uses vulkan GPU backend
- Auto-restarts on failure

### Installation

```bash
# Symlink to systemd user directory
mkdir -p ~/.config/systemd/user
ln -sf /path/to/vype/vype-dev.service ~/.config/systemd/user/vype-dev.service

# Enable and start
systemctl --user daemon-reload
systemctl --user enable vype-dev
systemctl --user start vype-dev
```

### Commands

```bash
# View logs
journalctl --user -u vype-dev -f

# Restart
systemctl --user restart vype-dev

# Stop
systemctl --user stop vype-dev
```

### Customization

Edit `vype-dev.service` to change:
- Model size: `-s small`, `-s medium`, `-s large`
- GPU backend: change `vulkan` to `cuda` or remove features for CPU
- CLI options: see `vype --help`

### Testing D-Bus

```bash
# Toggle recording
dbus-send --session --dest=tech.bytin.vype --type=method_call \
  /tech/bytin/vype tech.bytin.vype.Recorder.ToggleRecording

# Or use busctl
busctl call tech.bytin.vype /tech/bytin/vype tech.bytin.vype.Recorder ToggleRecording
```

## Production Service

The `install.sh` script installs a systemd user service automatically:

```bash
./install.sh
systemctl --user enable vype
systemctl --user start vype
```

This runs the installed binary from `~/.local/bin/vype`.
