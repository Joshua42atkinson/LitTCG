#!/bin/bash
# Mirror the Android device/emulator screen to the desktop using scrcpy.
# Falls back to a helpful message if scrcpy is not installed.
#
# Usage:
#   ./scripts/android_mirror.sh

set -e

PKG="com.littcg.game"

echo "═══════════════════════════════════════════"
echo "  LitTCG Android Screen Mirror"
echo "═══════════════════════════════════════════"

if ! command -v scrcpy &> /dev/null; then
    echo "ERROR: scrcpy is not installed."
    echo ""
    echo "Install it:"
    echo "  Ubuntu/Debian:  sudo apt install scrcpy"
    echo "  Fedora:         sudo dnf install scrcpy"
    echo "  Arch:           sudo pacman -S scrcpy"
    echo "  macOS:          brew install scrcpy"
    echo "  Other:          https://github.com/Genymobile/scrcpy"
    exit 1
fi

adb wait-for-device

# Launch the app if it's not already running so the mirror has something to show.
if ! adb shell pidof "$PKG" >/dev/null 2>&1; then
    echo "Launching $PKG..."
    adb shell monkey -p "$PKG" -c android.intent.category.LAUNCHER 1 >/dev/null 2>&1 || true
    echo "Waiting 2 seconds for the activity to start..."
    sleep 2
fi

echo "Starting scrcpy (press Ctrl+C to stop)..."
scrcpy --push-target=/sdcard/Download "$@" 2>/dev/null || scrcpy "$@"
