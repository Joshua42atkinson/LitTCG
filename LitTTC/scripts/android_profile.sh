#!/bin/bash
# Capture Android performance data for LitTCG.
#
# Because LitTCG uses a Bevy NativeActivity with direct OpenGL/Vulkan rendering,
# `dumpsys gfxinfo` reports 0 rendered Android Views. This script therefore:
#   1. Captures logcat and parses the Bevy FPS diagnostics.
#   2. Captures memory (dumpsys meminfo) and CPU (dumpsys cpuinfo) snapshots.
#   3. Saves raw gfxinfo for reference.
#
# Usage:
#   ./scripts/android_profile.sh [duration_seconds]
#
# Requires: adb, python3

set -e

PROJECT="$(cd "$(dirname "$0")/.." && pwd)"
DURATION="${1:-30}"
PKG="com.littcg.game"
OUT_DIR="$PROJECT/target/android-logs"
STAMP="$(date +%Y%m%d-%H%M%S)"
LOGCAT_FILE="$OUT_DIR/profile-logcat-$STAMP.txt"
MEMINFO_FILE="$OUT_DIR/meminfo-$STAMP.txt"
CPUINFO_FILE="$OUT_DIR/cpuinfo-$STAMP.txt"
GFXINFO_FILE="$OUT_DIR/gfxinfo-$STAMP.txt"

echo "═══════════════════════════════════════════"
echo "  LitTCG Android Performance Profile"
echo "  Duration: ${DURATION}s"
echo "═══════════════════════════════════════════"

mkdir -p "$OUT_DIR"
adb wait-for-device

# Launch the app if it isn't running
if ! adb shell pidof "$PKG" >/dev/null 2>&1; then
    echo "  Launching $PKG..."
    adb shell monkey -p "$PKG" -c android.intent.category.LAUNCHER 1 >/dev/null 2>&1 || true
    echo "  Waiting 3 seconds for activity start..."
    sleep 3
fi

adb shell dumpsys gfxinfo "$PKG" reset >/dev/null 2>&1 || true

echo ""
echo "  Collecting performance data for ${DURATION}s..."
adb logcat -c || true
adb logcat -v threadtime > "$LOGCAT_FILE" &
LOGCAT_PID=$!

for i in $(seq "$DURATION" -1 1); do
    printf "\r  %2ds remaining  " "$i"
    sleep 1
done
printf "\r  Done.              \n"

kill "$LOGCAT_PID" 2>/dev/null || true
wait "$LOGCAT_PID" 2>/dev/null || true

echo ""
echo "  Pulling system snapshots..."
adb shell dumpsys meminfo "$PKG" > "$MEMINFO_FILE" 2>/dev/null || true
adb shell dumpsys cpuinfo | grep -iE "$PKG|LitTCG" > "$CPUINFO_FILE" 2>/dev/null || true
adb shell dumpsys gfxinfo "$PKG" > "$GFXINFO_FILE" 2>/dev/null || true

echo ""
echo "═══════════════════════════════════════════"
echo "  Raw artifacts:"
echo "    Logcat:   $LOGCAT_FILE"
echo "    Meminfo:  $MEMINFO_FILE"
echo "    Cpuinfo:  $CPUINFO_FILE"
echo "    Gfxinfo:  $GFXINFO_FILE"
echo "═══════════════════════════════════════════"

# Parse the Bevy FPS diagnostics from logcat
python3 - "$LOGCAT_FILE" <<'PY'
import re, sys
path = sys.argv[1]
with open(path, errors='ignore') as f:
    text = f.read()

fps_values = [float(x) for x in re.findall(r'FPS Diagnostic Overlay:\s*([\d.]+)\s*fps', text, re.IGNORECASE)]

print("")
if fps_values:
    avg = sum(fps_values) / len(fps_values)
    p90 = sorted(fps_values)[int(len(fps_values) * 0.9)]
    print("Bevy FPS summary (from logcat):")
    print(f"  Samples: {len(fps_values)}")
    print(f"  Average: {avg:.1f} fps")
    print(f"  90th percentile: {p90:.1f} fps")
    print(f"  Min: {min(fps_values):.1f} fps")
    print(f"  Max: {max(fps_values):.1f} fps")
else:
    print("No 'FPS Diagnostic Overlay' samples found in logcat.")
    print("Ensure FrameDiagnostics is registered and running.")
PY
