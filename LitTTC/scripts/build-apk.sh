#!/bin/bash
# Build LitTCG APK for Android — manual packaging via Android SDK tools
# Supports device (aarch64) and emulator (x86_64) targets.
#
# Usage:
#   ./scripts/build-apk.sh            # phone build (arm64, no XR)
#   ./scripts/build-apk.sh --emulator   # emulator build (x86_64, no XR)
#   ./scripts/build-apk.sh --xr         # XR headset build (arm64)

set -e

PROJECT="$(cd "$(dirname "$0")/.." && pwd)"
SDK="${ANDROID_HOME:-/home/joshua/Android/Sdk}"
NDK_HOME="${NDK_HOME:-/home/joshua/Android/Sdk/ndk/30.0.14904198}"
BUILD_TOOLS_VERSION="$(ls -1 "$SDK/build-tools" 2>/dev/null | sort -V | tail -n 1)"
BUILD_TOOLS="$SDK/build-tools/${BUILD_TOOLS_VERSION:-37.0.0}"
PLATFORM="$SDK/platforms/android-36"

APP_NAME="LitTCG"
PKG="com.littcg.game"
VERSION=1
VERSION_NAME="0.1.0"
PLATFORM_API=30

# Default target
TARGET="aarch64-linux-android"
ABI="arm64-v8a"
APK_SUFFIX="arm64"
FEATURES=""
MANIFEST="AndroidManifest.phone.xml"

# Parse flags
while [[ $# -gt 0 ]]; do
    case "$1" in
        --emulator)
            TARGET="x86_64-linux-android"
            ABI="x86_64"
            APK_SUFFIX="x86_64"
            shift
            ;;
        --xr)
            MANIFEST="AndroidManifest.xml"
            FEATURES="xr"
            APK_SUFFIX="${APK_SUFFIX}-xr"
            shift
            ;;
        *)
            echo "Unknown argument: $1"
            echo "Usage: $0 [--emulator] [--xr]"
            exit 1
            ;;
    esac
done

echo "═══════════════════════════════════════════"
echo "  LitTCG APK Builder"
echo "  Target: $TARGET ($ABI)"
echo "═══════════════════════════════════════════"

# ─── 0. Validate toolchain ───
if [ ! -d "$BUILD_TOOLS" ]; then
    echo "ERROR: Android build-tools not found at $BUILD_TOOLS"
    echo "Install via Android Studio SDK Manager or set ANDROID_HOME."
    exit 1
fi
if [ ! -d "$PLATFORM" ]; then
    echo "ERROR: Android platform not found at $PLATFORM"
    exit 1
fi
if ! command -v cargo-ndk &> /dev/null; then
    echo "ERROR: cargo-ndk is not installed."
    echo "Install with: cargo install cargo-ndk"
    exit 1
fi

# ─── 1. Generate icons if missing ───
if [ ! -f "$PROJECT/res/mipmap-mdpi/ic_launcher.png" ]; then
    echo ""
    echo "[0/5] Generating launcher icons..."
    python3 "$PROJECT/scripts/generate_icon.py"
fi

# ─── 2. Build Rust library ───
echo ""
echo "[1/5] Building Rust library with cargo-ndk..."
cd "$PROJECT"
SO="$PROJECT/target/$TARGET/release/liblit_tcg.so"

# cargo-ndk sometimes panics on cleanup even when the build succeeds.
if [ -n "$FEATURES" ]; then
    cargo ndk -t $TARGET -P $PLATFORM_API build --release --lib --features "$FEATURES,bevy/debug" || true
else
    cargo ndk -t $TARGET -P $PLATFORM_API build --release --lib --features bevy/debug || true
fi

if [ ! -f "$SO" ]; then
    echo "ERROR: Build failed — $SO not found"
    exit 1
fi
echo "  Build succeeded"

# ─── 3. Prepare APK staging ───
echo ""
echo "[2/5] Preparing APK staging..."
STAGE="$PROJECT/target/android-apk"
rm -rf "$STAGE"
mkdir -p "$STAGE/lib/$ABI"

cp "$SO" "$STAGE/lib/$ABI/"

# Copy the C++ shared runtime library required by our Rust code.
NDK_LIB="$NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/lib/$TARGET"
if [ -f "$NDK_LIB/libc++_shared.so" ]; then
    cp "$NDK_LIB/libc++_shared.so" "$STAGE/lib/$ABI/"
else
    echo "ERROR: libc++_shared.so not found at $NDK_LIB/libc++_shared.so"
    exit 1
fi

# aapt expects the manifest file to be named AndroidManifest.xml
cp "$PROJECT/$MANIFEST" "$STAGE/AndroidManifest.xml"

# ─── 4. Package with aapt ───
echo ""
echo "[3/5] Packaging APK with aapt..."
APK_BASE="$STAGE/littcg-base.apk"

"$BUILD_TOOLS/aapt" package -f \
    -F "$APK_BASE" \
    -M "$STAGE/AndroidManifest.xml" \
    -I "$PLATFORM/android.jar" \
    -S "$PROJECT/res" \
    -A "$PROJECT/assets" \
    -0 png -0 ogg -0 json -0 ttf

# Add native libs into the APK
cd "$STAGE" && "$BUILD_TOOLS/aapt" add "$APK_BASE" lib/$ABI/liblit_tcg.so lib/$ABI/libc++_shared.so > /dev/null 2>&1

# ─── 5. Align and sign ───
echo ""
echo "[4/5] Aligning and signing APK..."
APK_ALIGNED="$STAGE/littcg-aligned.apk"
APK_FINAL="$PROJECT/target/littcg-v$VERSION_NAME-$APK_SUFFIX.apk"

"$BUILD_TOOLS/zipalign" -f 4 "$APK_BASE" "$APK_ALIGNED"

# Debug key (auto-generated if missing)
KEYSTORE="$PROJECT/target/debug.keystore"
if [ ! -f "$KEYSTORE" ]; then
    echo "  Generating debug keystore..."
    keytool -genkey -v \
        -keystore "$KEYSTORE" \
        -alias androiddebugkey \
        -keyalg RSA -keysize 2048 -validity 10000 \
        -storepass android -keypass android \
        -dname "CN=Android Debug,O=Android,C=US" \
        > /dev/null 2>&1
fi

echo "  Signing APK..."
"$BUILD_TOOLS/apksigner" sign \
    --ks "$KEYSTORE" \
    --ks-pass pass:android \
    --key-pass pass:android \
    --out "$APK_FINAL" \
    "$APK_ALIGNED" 2>&1 | grep -v "WARNING:" || true

# ─── Done ───
echo ""
echo "═══════════════════════════════════════════"
echo "  APK built successfully!"
echo "═══════════════════════════════════════════"
echo "  Output: $APK_FINAL"
echo "  Size:   $(du -h "$APK_FINAL" | cut -f1)"
echo ""
echo "  Install on device:"
echo "    adb install -r $APK_FINAL"
echo ""
echo "  Launch:"
echo "    adb shell monkey -p $PKG -c android.intent.category.LAUNCHER 1"
echo "═══════════════════════════════════════════"
