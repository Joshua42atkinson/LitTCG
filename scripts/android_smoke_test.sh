#!/bin/bash
# Android emulator/device smoke test for LitTCG
# Builds a debug APK, installs it, launches the app, and captures logcat.
# Exits non-zero if a crash, ANR, or fatal Rust panic is detected.
#
# Usage:
#   ./scripts/android_smoke_test.sh           # device test
#   ./scripts/android_smoke_test.sh --emulator
#   ./scripts/android_smoke_test.sh --duration 45

set -e

PROJECT="$(cd "$(dirname "$0")/.." && pwd)"
PKG="com.littcg.game"
DURATION=30
TARGET_FLAG=""
APK_SUFFIX="arm64"

while [[ $# -gt 0 ]]; do
    case "$1" in
        --emulator)
            TARGET_FLAG="--emulator"
            APK_SUFFIX="x86_64"
            shift
            ;;
        --duration)
            DURATION="$2"
            shift 2
            ;;
        *)
            echo "Unknown argument: $1"
            echo "Usage: $0 [--emulator] [--duration N]"
            exit 1
            ;;
    esac
done

APK="$PROJECT/target/littcg-v0.1.0-$APK_SUFFIX.apk"
LOG_DIR="$PROJECT/target/android-logs"
LOGCAT_FILE="$LOG_DIR/logcat-$(date +%Y%m%d-%H%M%S).txt"

echo "═══════════════════════════════════════════"
echo "  LitTCG Android Smoke Test"
echo "  Target: ${TARGET_FLAG:-device}"
echo "  Duration: ${DURATION}s"
echo "═══════════════════════════════════════════"

mkdir -p "$LOG_DIR"

# ─── 1. Build APK ───
echo ""
echo "[1/5] Building APK..."
"$PROJECT/scripts/build-apk.sh" $TARGET_FLAG

if [ ! -f "$APK" ]; then
    echo "ERROR: APK not found at $APK"
    exit 1
fi

# ─── 2. Wait for device/emulator ───
echo ""
echo "[2/5] Waiting for Android device/emulator..."
adb wait-for-device
DEVICE=$(adb shell getprop ro.product.model 2>/dev/null | tr -d '\r')
echo "  Connected: $DEVICE"

# ─── 3. Install APK ───
echo ""
echo "[3/5] Installing APK..."
adb install -r "$APK" | tail -n 3

# ─── 4. Clear and capture logcat ───
echo ""
echo "[4/5] Capturing logcat for ${DURATION}s..."
adb logcat -c || true
adb logcat -v threadtime > "$LOGCAT_FILE" &
LOGCAT_PID=$!

# ─── 5. Launch app and wait ───
echo ""
echo "[5/5] Launching LitTCG..."
adb shell monkey -p "$PKG" -c android.intent.category.LAUNCHER 1 || true

# Give the app up to 10 seconds to reach the main menu.
MAIN_MENU_TIMEOUT=10
MAIN_MENU_OK=0
echo ""
echo "  Waiting up to ${MAIN_MENU_TIMEOUT}s for MainMenu..."
for i in $(seq 1 "$MAIN_MENU_TIMEOUT"); do
    sleep 1
    if grep -qiE "State transition: Loading -> MainMenu" "$LOGCAT_FILE" 2>/dev/null; then
        echo "  MainMenu reached after ${i}s"
        MAIN_MENU_OK=1
        break
    fi
done

if [ "$MAIN_MENU_OK" -eq 0 ]; then
    echo "  MainMenu not reached within ${MAIN_MENU_TIMEOUT}s"
fi

# Wait out the remainder of the test duration
REMAINING=$((DURATION - MAIN_MENU_TIMEOUT))
if [ "$REMAINING" -gt 0 ]; then
    sleep "$REMAINING"
fi

# Stop logcat
kill "$LOGCAT_PID" 2>/dev/null || true
wait "$LOGCAT_PID" 2>/dev/null || true

# ─── 6. Analyze logcat for crashes / ANRs ───
echo ""
echo "Analyzing logcat..."

# Extract only log lines from our package so system-level matches don't false-positive.
# Bevy/Rust logs appear under tags like "RustStdoutStderr" and "event", but the
# process name is always present in logcat for our package's PID.
APP_LOGCAT_FILE="$LOG_DIR/app-$(date +%Y%m%d-%H%M%S).txt"
grep -E "$PKG|$(adb shell pidof "$PKG" 2>/dev/null | tr -d '\r')" "$LOGCAT_FILE" > "$APP_LOGCAT_FILE" 2>/dev/null || true

CRASH_PATTERNS=(
    "FATAL EXCEPTION"
    "AndroidRuntime"
    "Process .* has died"
    "ANR in .*$PKG"
    "panicked at"
    "rust_panic"
    "signal 6"
    "signal 11"
)

CRASH_FOUND=0
for pattern in "${CRASH_PATTERNS[@]}"; do
    if grep -qiE "$pattern" "$APP_LOGCAT_FILE"; then
        echo "  ⚠ CRASH SIGNATURE: $pattern"
        CRASH_FOUND=1
    fi
done

# Count known-good startup markers
STARTUP_OK=0
if grep -qiE "State transition: Loading -> MainMenu|Database parsed successfully" "$APP_LOGCAT_FILE"; then
    STARTUP_OK=1
fi

# Check for ANRs specifically in our app process
ANR_FOUND=0
if grep -qiE "ANR in .*$PKG|ANR.*$PKG|Application Not Responding" "$APP_LOGCAT_FILE"; then
    echo "  ⚠ ANR detected for $PKG"
    ANR_FOUND=1
fi

echo ""
echo "═══════════════════════════════════════════"
if [ "$CRASH_FOUND" -eq 0 ] && [ "$ANR_FOUND" -eq 0 ] && [ "$STARTUP_OK" -eq 1 ] && [ "$MAIN_MENU_OK" -eq 1 ]; then
    echo "  Smoke test PASSED"
    echo "  Logcat: $LOGCAT_FILE"
    echo "═══════════════════════════════════════════"
    exit 0
else
    echo "  Smoke test FAILED"
    if [ "$CRASH_FOUND" -ne 0 ]; then
        echo "  Crashes detected in logcat."
    fi
    if [ "$ANR_FOUND" -ne 0 ]; then
        echo "  ANR detected in logcat."
    fi
    if [ "$STARTUP_OK" -eq 0 ]; then
        echo "  App did not log startup (Loading -> MainMenu) message."
    fi
    if [ "$MAIN_MENU_OK" -eq 0 ]; then
        echo "  Main menu was not reached within ${MAIN_MENU_TIMEOUT}s."
    fi
    echo "  Logcat: $LOGCAT_FILE"
    echo "═══════════════════════════════════════════"
    exit 1
fi
