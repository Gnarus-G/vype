# Vype

Push-to-talk speech-to-text application that types transcribed text directly into any application.

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

- `VYPE_GPU=cuda` — Force CUDA backend (default: auto-detect)
- `INSTALL_DIR=/path` — Custom install directory (default: `~/.local/bin`)
- `VERSION=v0.1.0` — Install specific version (default: latest)

Example:

```bash
VYPE_GPU=cuda INSTALL_DIR=~/bin curl -fsSL https://raw.githubusercontent.com/gnarus-g/vype/main/install.sh | bash
```

### Manual Download

Download from [GitHub Releases](https://github.com/gnarus-g/vype/releases):

- `vype-vX.X.X-x86_64-linux-vulkan.tar.gz` — For AMD, NVIDIA, or Intel GPUs
- `vype-vX.X.X-x86_64-linux-cuda.tar.gz` — For NVIDIA GPUs only (may be faster)

## Usage

```bash
# Run (CPU backend)
vype

# Press and hold F12 to record, release to transcribe
```

- Press and hold **F12** (default) to start recording
- Release to transcribe and type the result
- Press **Ctrl+C** to exit

## CLI Options

```
Usage: vype [OPTIONS]

Options:
  -m, --model <PATH>       Custom model path (auto-downloads to ~/.config/vype/)
  -s, --model-size <SIZE>  Model size: tiny, base, small, medium, large (default: small)
  -k, --key <KEY>          PTT key: F1-F12 (default: F12)
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
vype -s large # or medium

# Use F8 as the push-to-talk key
vype -k F8

# Transcribe in Spanish
vype -l es

# Use a custom model path
vype -m /path/to/ggml-small.en.bin
```

## Build from Source

### Prerequisites

- Rust 1.70+
- Optional for GPU acceleration: Vulkan SDK or CUDA Toolkit
- libxdo (xdotool)

### Build Commands

Enable exactly one backend feature per build: `cpu`, `vulkan`, or `cuda`.

```bash
# Clone the repository
git clone https://github.com/gnarus-g/vype.git
cd vype

# CPU build (works everywhere)
cargo build --release --features cpu

# Vulkan build (AMD, NVIDIA, Intel GPUs)
cargo build --release --features vulkan

# CUDA build (NVIDIA GPUs only, requires CUDA Toolkit)
cargo build --release --features cuda

# The binary will be at target/release/vype
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
