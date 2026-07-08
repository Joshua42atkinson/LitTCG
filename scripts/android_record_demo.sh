#!/bin/bash
# Record a gameplay demo clip from an Android device or emulator.
# Pulls the video to demo/android_emulator_demo_YYYY-MM-DD.mp4 when finished.
#
# Usage:
#   ./scripts/android_record_demo.sh [duration_seconds]
#
# Requires: adb

set -e

PROJECT="$(cd "$(dirname "$0")/.." && pwd)"
DURATION="${1:-60}"
PKG="com.littcg.game"
DEVICE_FILE="/sdcard/littcg_demo.mp4"
OUT_DIR="$PROJECT/demo"
OUT_FILE="$OUT_DIR/android_emulator_demo_$(date +%Y-%m-%d-%H%M%S).mp4"

echo "═══════════════════════════════════════════"
echo "  LitTCG Android Demo Recorder"
echo "  Duration: ${DURATION}s"
echo "═══════════════════════════════════════════"

adb wait-for-device

# Make sure the app is running
if ! adb shell pidof "$PKG" >/dev/null 2>&1; then
    echo "Launching $PKG..."
    adb shell monkey -p "$PKG" -c android.intent.category.LAUNCHER 1 >/dev/null 2>&1 || true
    sleep 2
fi

mkdir -p "$OUT_DIR"

echo ""
echo "Recording screen for ${DURATION}s..."
echo "  Press Ctrl+C to stop early."
echo ""

# Run screenrecord in the background; it has a hard 180s limit.
adb shell screenrecord --time-limit "$DURATION" "$DEVICE_FILE" &
RECORD_PID=$!

# Show a countdown so the user knows it's alive.
for i in $(seq "$DURATION" -1 1); do
    printf "\r  Recording... %2ds remaining  " "$i"
    sleep 1
done
wait "$RECORD_PID" 2>/dev/null || true
printf "\r  Recording complete.              \n"

echo ""
echo "Pulling video from device..."
adb pull "$DEVICE_FILE" "$OUT_FILE"
adb shell rm "$DEVICE_FILE" 2>/dev/null || true

echo ""
echo "═══════════════════════════════════════════"
echo "  Demo saved to:"
echo "  $OUT_FILE"
echo "═══════════════════════════════════════════"
