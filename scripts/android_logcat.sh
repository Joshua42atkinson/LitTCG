#!/bin/bash
# Stream filtered logcat for LitTCG only.
# Clears the log buffer first, then shows only lines from our package.
#
# Usage:
#   ./scripts/android_logcat.sh

set -e

PKG="com.littcg.game"

echo "═══════════════════════════════════════════"
echo "  LitTCG Filtered Logcat"
echo "  Package: $PKG"
echo "═══════════════════════════════════════════"
echo ""

adb wait-for-device

echo "Clearing logcat buffer..."
adb logcat -c

echo "Streaming LitTCG logs. Press Ctrl+C to stop."
echo ""

# Filter by PID if the app is running, otherwise fall back to package-name grep.
PID=$(adb shell pidof "$PKG" 2>/dev/null | tr -d '\r' || true)
if [ -n "$PID" ]; then
    adb logcat --pid="$PID"
else
    adb logcat | grep --line-buffered -E "$PKG|lit_tcg|RustStdoutStderr"
fi
