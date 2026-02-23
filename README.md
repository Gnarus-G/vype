# Vype

Push-to-talk speech-to-text application that types transcribed text directly into any application.

## Features

- **Push-to-talk**: Hold a key to record, release to transcribe and type
- **GPU-accelerated**: Uses Vulkan for fast Whisper transcription
- **Auto-model download**: Downloads the Whisper model from HuggingFace on first run
- **Configurable**: Custom PTT key, language, and recording duration

## Installation

```bash
# Clone the repository
git clone https://github.com/YOUR_USERNAME/vype.git
cd vype

# Build with transcription support
cargo build --features transcription --release
```

## Usage

```bash
# Run the application
cargo run --features transcription --release

# Or run the binary directly
./target/release/vype
```

- Press and hold **F12** (default) to start recording
- Release to transcribe and type the result
- Press **Ctrl+C** to exit

## CLI Options

```
-m, --model <PATH>        Custom model path (auto-downloads to ~/.config/vype/)
-k, --key <KEY>           PTT key: F1-F12 (default: F12)
-l, --language <LANG>     Transcription language (default: en)
-d, --max-duration <SEC>  Max recording duration in seconds (default: 30)
```

### Examples

```bash
# Use F8 as the push-to-talk key
vype -k F8

# Transcribe in Spanish
vype -l es

# Use a custom model
vype -m /path/to/ggml-small.en.bin

# Limit recordings to 15 seconds
vype -d 15
```

## Requirements

- Rust 1.70 or later
- Vulkan-capable GPU (for hardware acceleration)
- Microphone

## Development

```bash
# Run tests (fast, without whisper compilation)
cargo test --no-default-features

# Build without transcription (for faster iteration)
cargo build --no-default-features
```

## Architecture

```
src/
├── main.rs              # Application entry point
├── lib.rs               # Library exports
├── actors.rs            # Actor model infrastructure
├── audio.rs             # Audio capture actor
├── keyboard.rs          # Keyboard orchestration actor
├── transcriber.rs       # Transcription actor
├── config.rs            # CLI configuration
├── model.rs             # Model download logic
├── sources/
│   ├── mod.rs           # Traits (AudioSource, KeyboardSink, Transcriber)
│   ├── cpal_audio.rs    # Real microphone capture
│   ├── rdev_keyboard.rs # Real keyboard typing
│   ├── whisper_transcriber.rs  # Real transcription
│   └── fakes.rs         # Test doubles
└── pure/
    ├── typer.rs         # Text to key events
    ├── resample.rs      # Audio resampling
    └── synth_audio.rs   # Test audio generation
```

## License

GPL-2.0-only
