#!/bin/bash
# Fully automated end-to-end test for Vype two-process architecture
# Uses PulseAudio module-pipe-source for virtual microphone

set -euo pipefail

WAV_FILE="${1:-$HOME/tts_output.wav}"
LOG_FILE="/tmp/vype_e2e.log"
PIPE_FILE="/tmp/vype_audio_pipe"
FFMPEG_PID=""
PIPE_MODULE_ID=""
ORIG_SOURCE=""
START_TS=""

cleanup() {
  echo "Cleaning up..."
  [ -n "$FFMPEG_PID" ] && kill "$FFMPEG_PID" 2>/dev/null || true
  if [ -n "$PIPE_MODULE_ID" ]; then
    pactl unload-module "$PIPE_MODULE_ID" 2>/dev/null || true
  else
    pactl unload-module module-pipe-source 2>/dev/null || true
  fi
  rm -f "$PIPE_FILE"
  if [ -n "$ORIG_SOURCE" ]; then
    pactl set-default-source "$ORIG_SOURCE" 2>/dev/null || true
  fi
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

# Stop/restart service cleanly
echo "Stopping existing vype-dev service..."
systemctl --user stop vype-dev 2>/dev/null || true
systemctl --user reset-failed vype-dev 2>/dev/null || true
sleep 1

# Unload any existing pipe-source modules
pactl unload-module module-pipe-source 2>/dev/null || true
sleep 0.5

ORIG_SOURCE=$(pactl get-default-source)
echo "Original default source: $ORIG_SOURCE"

# Create virtual microphone using module-pipe-source
echo "Creating virtual microphone..."
rm -f "$PIPE_FILE"
mkfifo "$PIPE_FILE"
PIPE_MODULE_ID=$(pactl load-module module-pipe-source source_name=vype_test file="$PIPE_FILE" format=s16le rate=16000 channels=1)

# Set as default source
echo "Setting vype_test as default source..."
pactl set-default-source vype_test

# Verify
echo "Default source:"
pactl get-default-source

# Build command client used for control events
echo "Building vypec..."
cargo build -p vypec --release --quiet 2>/dev/null || cargo build -p vypec --release

# Clear log file
>"$LOG_FILE"

START_TS=$(date '+%Y-%m-%d %H:%M:%S')

# Restart daemon service
echo "Starting vype-dev service..."
systemctl --user daemon-reload
systemctl --user restart vype-dev
sleep 2

if ! systemctl --user is-active --quiet vype-dev; then
  echo "ERROR: vype-dev service is not active"
  systemctl --user --no-pager status vype-dev || true
  exit 1
fi

echo "✓ vype-dev service is active"

echo "Waiting for daemon readiness..."
for _ in {1..90}; do
  if journalctl --user -u vype-dev --since "$START_TS" --no-pager | grep -q "Listening for PTT key and IPC control events"; then
    echo "✓ vyped is ready"
    break
  fi
  sleep 1
done

if ! journalctl --user -u vype-dev --since "$START_TS" --no-pager | grep -q "Listening for PTT key and IPC control events"; then
  echo "ERROR: vyped did not become ready in time"
  journalctl --user -u vype-dev --since "$START_TS" --no-pager | tail -40
  exit 1
fi

# Start recording
echo "Starting recording..."
./target/release/vypec start

# Feed audio to the pipe (in background)
echo "Feeding audio to virtual mic..."
ffmpeg -i "$WAV_FILE" -ar 16000 -ac 1 -f s16le -y "$PIPE_FILE" 2>/dev/null &
FFMPEG_PID=$!

# Wait for audio duration
sleep $(echo "$DURATION + 1" | bc)

# Stop recording
echo "Stopping recording..."
./target/release/vypec stop

# Send a second stop to reduce chance of one-shot IPC loss under load
sleep 1
./target/release/vypec stop

# Wait for stop + final transcription
echo "Waiting for final transcription..."
for _ in {1..60}; do
  journalctl --user -u vype-dev --since "$START_TS" --no-pager >"$LOG_FILE"
  if grep -q "Recording stopped" "$LOG_FILE" && grep -q "Transcribed:" "$LOG_FILE"; then
    break
  fi
  sleep 1
done

journalctl --user -u vype-dev --since "$START_TS" --no-pager >"$LOG_FILE"

# Check results from logs
echo ""
echo "=== Results ==="

# Look for transcription results in the log
TRANSCRIBED_TEXT=$(grep "Transcribed:" "$LOG_FILE" | tail -1 | sed -E 's/.*Transcribed:[[:space:]]*//' || true)
TYPED_CHARS=${#TRANSCRIBED_TEXT}
SAMPLES_INFO=$(grep "Samples:" "$LOG_FILE" | tail -1 || true)
RESAMPLED=$(grep "Resampled" "$LOG_FILE" | tail -1 || true)

if [ -n "$TRANSCRIBED_TEXT" ] && [ "$TYPED_CHARS" -gt 0 ]; then
  echo "✓ SUCCESS: Transcribed $TYPED_CHARS characters"
  echo ""
  echo "Details:"
  echo "  $SAMPLES_INFO"
  echo "  $RESAMPLED"
  echo "  Transcribed text: $TRANSCRIBED_TEXT"
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
