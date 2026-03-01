#!/bin/bash
# Fully automated end-to-end test for Vype
# Uses PulseAudio module-pipe-source for virtual microphone

set -e

WAV_FILE="${1:-$HOME/tts_output.wav}"
LOG_FILE="/tmp/vype_e2e.log"
PIPE_FILE="/tmp/vype_audio_pipe"
VYPE_PID=""
FFMPEG_PID=""

cleanup() {
    echo "Cleaning up..."
    [ -n "$VYPE_PID" ] && kill $VYPE_PID 2>/dev/null || true
    [ -n "$FFMPEG_PID" ] && kill $FFMPEG_PID 2>/dev/null || true
    pactl unload-module module-pipe-source 2>/dev/null || true
    rm -f "$PIPE_FILE"
    pactl set-default-source alsa_input.usb-3142_fifine_Microphone-00.analog-stereo 2>/dev/null || true
}

trap cleanup EXIT

echo "=== Vype Automated E2E Test ==="
echo "Audio file: $WAV_FILE"

# Check audio file exists
if [ ! -f "$WAV_FILE" ]; then
    echo "ERROR: Audio file not found: $WAV_FILE"
    exit 1
fi

# Get audio duration
DURATION=$(ffprobe -v error -show_entries format=duration -of default=noprint_wrappers=1:nokey=1 "$WAV_FILE" 2>/dev/null)
echo "Audio duration: ${DURATION}s"

# Stop any running vype
echo "Stopping existing vype..."
systemctl --user stop vype-dev 2>/dev/null || true
pkill -f "target/debug/vype" 2>/dev/null || true
sleep 1

# Unload any existing pipe-source modules
pactl unload-module module-pipe-source 2>/dev/null || true
sleep 0.5

# Create virtual microphone using module-pipe-source
echo "Creating virtual microphone..."
rm -f "$PIPE_FILE"
mkfifo "$PIPE_FILE"
pactl load-module module-pipe-source source_name=vype_test file="$PIPE_FILE" format=s16le rate=16000 channels=1

# Set as default source
echo "Setting vype_test as default source..."
pactl set-default-source vype_test

# Verify
echo "Default source:"
pactl get-default-source

# Build vype
echo "Building vype..."
cargo build --features vulkan --quiet 2>/dev/null || cargo build --features vulkan

# Clear log file
> "$LOG_FILE"

# Start vype
echo "Starting vype..."
RUST_LOG=vype=info DISPLAY=:0 XAUTHORITY="${XAUTHORITY:-$HOME/.Xauthority}" ./target/debug/vype -s base 2>&1 | tee "$LOG_FILE" &
VYPE_PID=$!
sleep 4

# Wait for D-Bus to be ready
for i in {1..15}; do
    if busctl --user status tech.bytin.vype &>/dev/null; then
        echo "✓ Vype D-Bus ready"
        break
    fi
    sleep 0.5
done

if ! busctl --user status tech.bytin.vype &>/dev/null; then
    echo "ERROR: D-Bus service not ready"
    exit 1
fi

# Start recording
echo "Starting recording..."
busctl --user call tech.bytin.vype /tech/bytin/vype tech.bytin.vype.Recorder StartRecording

# Feed audio to the pipe (in background)
echo "Feeding audio to virtual mic..."
ffmpeg -i "$WAV_FILE" -ar 16000 -ac 1 -f s16le -y "$PIPE_FILE" 2>/dev/null &
FFMPEG_PID=$!

# Wait for audio duration
sleep $(echo "$DURATION + 1" | bc)

# Stop recording
echo "Stopping recording..."
busctl --user call tech.bytin.vype /tech/bytin/vype tech.bytin.vype.Recorder StopRecording

# Wait for transcription
sleep 3

# Check results from logs
echo ""
echo "=== Results ==="

# Look for transcription results in the log
TYPED_CHARS=$(grep "Transcribed" "$LOG_FILE" | tail -1 | grep -oP 'Transcribed \K[0-9]+')
SAMPLES_INFO=$(grep "Samples:" "$LOG_FILE" | tail -1)
RESAMPLED=$(grep "Resampled" "$LOG_FILE" | tail -1)

if [ -n "$TYPED_CHARS" ] && [ "$TYPED_CHARS" -gt 0 ]; then
    echo "✓ SUCCESS: Transcribed $TYPED_CHARS characters"
    echo ""
    echo "Details:"
    echo "  $SAMPLES_INFO"
    echo "  $RESAMPLED"
    echo "  Transcribed: $TYPED_CHARS chars"
    exit 0
elif grep -q "BLANK_AUDIO" "$LOG_FILE"; then
    echo "✗ FAIL: Got BLANK_AUDIO"
    echo "Details: $SAMPLES_INFO"
    exit 1
else
    echo "Log output:"
    echo "  $SAMPLES_INFO"
    echo "  $RESAMPLED"
    echo ""
    echo "Last 10 lines:"
    tail -10 "$LOG_FILE"
    exit 1
fi