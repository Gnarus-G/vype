# Vype

Push-to-talk speech-to-text application that types transcribed text directly into any application.

Vype uses a two-process architecture:

- `vyped` daemon: keyboard/PTT, audio capture, transcription, typing
- `vypec` client: sends control commands (`start`, `stop`, `toggle`, `partial`)

## Features

- **Push-to-talk**: Hold a key to record, release to transcribe and type
- **Flexible backends**: CPU works everywhere, with optional Vulkan (AMD/NVIDIA/Intel) or CUDA (NVIDIA) acceleration
- **Auto-model download**: Downloads the Whisper model from HuggingFace on first run
- **Configurable**: Custom PTT key, language, model size, and recording duration

## System Requirements

### Acceleration Backends

- **CPU build**: Works on any x86_64 Linux system
- **Vulkan build**: Any GPU with Vulkan support (AMD, NVIDIA, Intel)
- **CUDA build**: NVIDIA GPU + CUDA Toolkit

### System Libraries

**Arch Linux:**

```bash
sudo pacman -S xdotool vulkan-tools
```

**Ubuntu/Debian:**

```bash
sudo apt install libxdo-dev libvulkan1
```

## Installation

### Quick Install

```bash
curl -fsSL https://raw.githubusercontent.com/gnarus-g/vype/main/install.sh | bash
```

Options:

- `INSTALL_DIR=/path` — Custom install directory (default: `~/.local/bin`)
- `VERSION=v0.1.0` — Install specific version (default: latest)

### Manual Download

Download from [GitHub Releases](https://github.com/gnarus-g/vype/releases):

- `vype-vX.X.X-x86_64-linux-vulkan.tar.gz` — For AMD, NVIDIA, or Intel GPUs
- `vype-vX.X.X-x86_64-linux-cuda.tar.gz` — For NVIDIA GPUs only (may be faster)
- `vype-vX.X.X-x86_64-linux-cpu.tar.gz` — CPU-only fallback

## Usage

```bash
# Terminal 1: run daemon (default backend: cpu)
vyped -k F9

# Terminal 2: send one-shot commands
vypec toggle
vypec start
vypec stop
vypec partial
```

- Hold configured key (default **F9**) to record, release to transcribe and type.
- `vypec toggle` starts/stops recording in toggle mode.
- Press **Ctrl+C** in daemon terminal to exit.

## CLI Options

```
Usage: vyped [OPTIONS]

Options:
  -m, --model <PATH>       Custom model path (auto-downloads to ~/.config/vype/)
  -s, --model-size <SIZE>  Model size: tiny, base, small, medium, large (default: small)
  -k, --key <KEY>          PTT key: F1-F12 (default: F9)
  -l, --language <LANG>    Transcription language (default: en)
  -d, --max-duration <SEC> Max recording duration in seconds (default: 30)
  -h, --help               Print help
```

### Model Sizes

| Size   | Disk Space | Quality | Speed   |
| ------ | ---------- | ------- | ------- |
| tiny   | 75 MB      | Lowest  | Fastest |
| base   | 142 MB     | Low     | Fast    |
| small  | 466 MB     | Good    | Medium  |
| medium | 1.5 GB     | Better  | Slower  |
| large  | 2.9 GB     | Best    | Slowest |

### Examples

```bash
# Use a larger model for better accuracy
vyped -s large # or medium

# Use F8 as the push-to-talk key
vyped -k F8

# Transcribe in Spanish
vyped -l es

# Use a custom model path
vyped -m /path/to/ggml-small.en.bin
```

## Build from Source

### Prerequisites

- Rust 1.70+
- Optional for GPU acceleration: Vulkan SDK or CUDA Toolkit
- libxdo (xdotool)

### Build Commands

Enable exactly one backend feature for `vyped`: `cpu`, `vulkan`, or `cuda`.

```bash
# Clone the repository
git clone https://github.com/gnarus-g/vype.git
cd vype

# CPU build (default backend)
cargo build --release -p vyped -p vypec

# Vulkan build (AMD, NVIDIA, Intel GPUs)
cargo build --release -p vyped -p vypec --no-default-features --features vulkan

# CUDA build (NVIDIA GPUs only, requires CUDA Toolkit)
cargo build --release -p vyped -p vypec --no-default-features --features cuda

# Binaries will be at target/release/vyped and target/release/vypec
```

## Development

```bash
# Run tests (fast, without whisper compilation)
cargo test

# Check code without building
cargo check
```

## License

GPL-2.0-only
