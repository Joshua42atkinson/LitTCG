# LitTCG Android Total System Integration Plan
## NDK build, SDK packaging, store compliance, and cloud integration

**Date:** July 7, 2026  
**Goal:** Define the complete Android build, packaging, distribution, and integration plan for LitTCG, using existing nearby projects and 2026 best practices.

---

## 1. Current LitTCG Android State

| Item | Status | Notes |
|------|--------|-------|
| `Cargo.toml` Android metadata | ✅ | `package.metadata.android` with package, SDK versions, permissions, signing |
| `src/lib.rs` Android entry | ✅ | `#[bevy_main]` with XR and non-XR branches |
| `AndroidManifest.xml` | ✅ | Has XR hand tracking + spatial anchoring features |
| `cargo check --features xr` | ✅ | Compiles |
| APK build script | ✅ | `scripts/build-apk.sh` builds signed phone APK |
| AAB / Google Play pipeline | ❌ | Not yet created |
| App icons | ✅ | `scripts/generate_icon.py` + `res/mipmap-*` |
| Touch/mobile input polish | ✅ | Phase 3 complete; needs device testing |
| Voice input (Android SpeechRecognizer) | ❌ | Optional |
| In-app billing (Google Play / Polar) | ❌ | Future |
| Google Play Store listing | ❌ | Future |

---

## 2. 2026 Bevy Android Build Pipeline Options

There are three viable paths. We should use **Path 1 for development** and **Path 2 for store release**.

### Path 1: cargo-ndk + Manual APK Script (Fastest for Dev)

Same pattern as `trinity-ndk`.

```
cargo-ndk builds liblit_tcg.so
    ↓
build-apk.sh packages .so + manifest + res with aapt/zipalign/apksigner
    ↓
APK for sideloading / direct sales
```

**Pros:** Fast, no Android Studio, matches current code.  
**Cons:** Cannot produce AAB for Google Play; manual asset management.

### Path 2: cargo-ndk + Android Studio + Gradle (Production / Google Play)

Modern Bevy Android recommendation.

```
cargo-ndk builds liblit_tcg.so
    ↓
Android Studio Gradle project packages .so + Kotlin wrapper + assets
    ↓
AAB signed and uploaded to Google Play Console
```

**Pros:** Produces AAB, Play Store compliant, easy JNI bridges (billing, TTS, voice), modern tooling.  
**Cons:** More files, requires Android Studio or Gradle CLI.

### Path 3: xbuild (Cross-Platform CI/CD)

Used by `bevy_game_template` for release AAB.

```
xbuild compiles Rust and packages AAB in one command
    ↓
AAB signed and uploaded to Google Play Console
```

**Pros:** CLI-first, good for CI/CD.  
**Cons:** Requires a fork of xbuild for AAB support; less flexible for custom JNI.

**Recommendation:** Start with Path 1, migrate to Path 2 for Play Store release, and evaluate Path 3 for CI automation.

---

## 3. Reusable Assets from `/home/joshua/Workflow/ARCHIVE_VAULT/Phone/trinity-ndk`

Highest-value source. Working Bevy native Android NDK app.

| Asset | File Path | Use in LitTCG | Priority |
|-------|-----------|---------------|----------|
| **APK build script** | `scripts/build-apk.sh` | Path 1: adapt for LitTCG package name, library name, paths | **P0** |
| **Icon generator** | `scripts/generate_icon.py` | Generate LitTCG-branded Android icons at all densities | **P0** |
| `build.rs` linker flags | `build.rs` | Add `c++_shared` / `stdc++` link for Android | **P0** |
| **NativeActivity Java subclass** | `gen/android/app/src/main/java/.../TrinityActivity.java` | Template for `LitTCGActivity.java` if we need voice/TTS/camera | **P1** |
| **Voice JNI bridge** | `src/voice.rs` | Optional: Android SpeechRecognizer → Rust for hands-free spelling | **P2** |
| **Cargo.toml Android config** | `Cargo.toml` | Reference for `package.metadata.android`, signing, permissions | **P1** |
| **AndroidManifest.xml (basic)** | `AndroidManifest.xml` | Simpler fallback if XR features cause install issues | **P2** |

---

## 4. Reusable Assets from Voix Vive (`/home/joshua/Workflow/Bertrand-Masterclass`)

| Asset | File Path | Use in LitTCG | Priority |
|-------|-----------|---------------|----------|
| **Jetpack XR SDK app** | `apps/xr-prototype/android-xr/` | Path 3 alternative: full Kotlin Android XR app | **P2** |
| **Hand tracking manager** | `.../HandTrackingManager.kt` | Real hand tracking for XR mode | **P1** |
| **Google OAuth → Gemini** | companion app | Optional AI tutor with student-paid API quota | **P1** |
| **Compose spatial UI** | `.../VoixViveXrApp.kt` | Reference for XR UI if we go Kotlin path | **P3** |

---

## 5. Total System Integration Todo

### Phase 0: Foundation (P0 — Done)

- [x] **0.1 Add `LitTTC/build.rs` linker config**
  - [x] Added `build.rs` linking `c++_shared` and `stdc++` on Android.
  - [x] Verified `cargo check --features xr` still passes.

- [x] **0.2 Create `LitTTC/scripts/build-apk.sh`**
  - [x] Created script with project, package, app name, and library name configured.
  - [x] Supports `--emulator` and `--xr` flags; default is phone (non-XR).
  - [x] Output: `target/littcg-v0.1.0-arm64.apk` (21 MB).

- [x] **0.3 Generate app icons**
  - [x] Created `scripts/generate_icon.py` generating LitTCG-branded icons at all densities.
  - [x] Outputs to `res/mipmap-*/ic_launcher.png` and `assets/branding/ic_launcher_source.png`.

- [x] **0.4 First APK install and smoke test**
  - [x] `scripts/build-apk.sh` produces a signed phone APK (`target/littcg-v0.1.0-arm64.apk`, 21 MB).
  - [x] `scripts/build-apk.sh --xr` produces a signed XR APK (`target/littcg-v0.1.0-arm64-xr.apk`, 24 MB).
  - [x] `scripts/build-apk.sh --emulator` produces `x86_64` APK for Android Emulator.
  - [x] Fixed missing `libc++_shared.so` packaging.
  - [x] Patched `jni` crate race condition in exception handling.
  - [x] Fixed APK asset compression causing invalid PNG/OGG/TTF data (`aapt -0` flags).
  - [x] Fixed Android `lib.rs` missing common resource initialization (`CurrentSpelling`, `GameDatabase`, etc.).
  - [x] Fixed Bevy ECS query conflict in `render.rs::update_pet_expressions`.
  - [x] Fixed Android `lib.rs` missing `.init_state::<GameState>()` and `add_message::<GameCommand>()` (was causing runtime panics in `spatial_ui` and `altar` systems on launch).
  - [x] Hardened `scripts/android_smoke_test.sh` to filter logcat to the LitTCG process and avoid false-positive crash signatures from system logs.
  - [x] Fixed `assets/textures/avatars/barnaby.png` — file was a JPEG with `.png` extension; converted to a real PNG so Bevy's image loader no longer logs an error.
  - [x] APK installs and launches on emulator; app reaches Bevy render and scene initialization.
  - [x] Stable 60-second smoke test without runtime panic confirmed (`android_smoke_test.sh --emulator --duration 60` PASSED).

### Phase 1: Android Runtime Polish (P1 — Make It Playable)

- [/] **1.1 Async asset loading for Android**
  - [x] `build-apk.sh` includes `assets/` in the APK; APK contains JSON, textures, audio, and fonts.
  - [x] Async loading systems (`database.rs`) are already in place from Phase 3.
  - [x] Fixed binary asset compression (`aapt -0 png -0 ogg -0 json -0 ttf`) so Bevy AssetLoader can read them.
  - [ ] Verify on device that loading screen completes and assets appear.

- [/] **1.2 Touch-first UI on Android**
  - [x] Larger tap targets, HUD scaling, and letter crystal sizes implemented in Phase 3.
  - [ ] Test on a real phone/tablet.

- [x] **1.3 Save file location on Android**
  - [x] Added `src/core/platform_paths.rs` with `data_dir()` helper.
  - [x] `save.rs` and `settings.rs` save to `data_dir()` on all platforms (Android app private dir; desktop cwd).
  - [ ] Verify persistence across restarts on a real device.

- [x] **1.4 Performance presets for Android**
  - [x] Added `PerformancePlugin` with low shadow map resolution on Android/XR.
  - [x] Added `disable_msaa_on_cameras` system to turn off MSAA on mobile/XR.
  - [x] Registered in both `main.rs` and `lib.rs` desktop/Android/XR paths.
  - [ ] Verify 30 FPS on a mid-range phone.

- [x] **1.5 Non-XR manifest variant**
  - [x] Created `AndroidManifest.phone.xml` without XR requirements.
  - [x] `build-apk.sh` uses `AndroidManifest.phone.xml` by default and `AndroidManifest.xml` with `--xr`.

### Phase 2: XR Polish (P1 — For Headset Demos)

- [ ] **2.1 Port real hand tracking from Voix Vive**
  - Use `apps/spatial-engine-bevy/src/hand_tracking.rs` logic.
  - Replace LitTCG hand tracking stubs.

- [ ] **2.2 Spatial UI panels for XR**
  - Fix Bevy ParamSet conflicts using Voix Vive gated modules.
  - Add XR menu and pet card positioning.

- [ ] **2.3 Test on Quest / Android XR device**
  - Verify pinch-to-spell and pinch-to-select.

### Phase 3: Store-Ready Packaging (P2 — For Google Play)

- [ ] **3.1 Create Android Studio project (`mobile/android/`)**
  - Create Gradle project with Kotlin build scripts.
  - Configure `cargo-ndk` via `cargo-ndk-android-gradle` plugin or custom gradle task.
  - Package `liblit_tcg.so`, assets, and manifest.

- [ ] **3.2 Configure GameActivity vs NativeActivity**
  - Decide: `NativeActivity` (simpler, already works) or `GameActivity` (modern, better input).
  - Update `Cargo.toml` feature flag accordingly.
  - Note: `GameActivity` requires AppCompat theme and cannot use `cargo-apk`.

- [ ] **3.3 Build AAB instead of APK**
  - Use Gradle `bundleRelease`.
  - Configure signing keystore.
  - Verify AAB opens in Android Studio Bundle Analyzer.

- [ ] **3.4 App signing and version management**
  - Create upload key / use Play App Signing.
  - Automate versionCode bump in CI.

- [ ] **3.5 Internal testing track**
  - Upload AAB to Google Play Console internal testing.
  - Invite testers.

### Phase 4: Google Play Compliance for Kids Apps (P2 — Critical)

- [ ] **4.1 Target audience declaration**
  - Declare primary audience as children (under 13).
  - This triggers stricter policies.

- [ ] **4.2 Data safety form**
  - Declare what data is collected: none, or only local save data.
  - LitTCG is local-first → "No data collected" is accurate.

- [ ] **4.3 Privacy policy page**
  - Required for all apps, especially children's apps.
  - Host on Cloudflare Pages or your domain.

- [ ] **4.4 Content rating (ESRB/PEGI/IARC)**
  - Complete questionnaire in Play Console.
  - Expected rating: Everyone / E.

- [ ] **4.5 Ads and monetization policy review**
  - No ads in children's apps.
  - In-app purchases require parent gate or may be restricted.
  - Consider upfront paid app ($9.99) instead of IAP.

- [ ] **4.6 COPPA / GDPR-K compliance documentation**
  - No cloud accounts, no tracking, no third-party SDKs.
  - Document this for Play Console appeal if flagged.

### Phase 5: Optional Native Integrations (P2/P3)

- [ ] **5.1 Android SpeechRecognizer for voice spelling**
  - Port `trinity-ndk/src/voice.rs` + `TrinityActivity.java`.
  - Add "Spell by voice" accessibility button.

- [ ] **5.2 Android TTS (native)**
  - Use Android TTS instead of Kokoro sidecar on mobile.
  - Better offline support and smaller app size.

- [ ] **5.3 Google Play Billing (if needed)**
  - Implement Kotlin billing client.
  - Or keep using Polar.sh webview checkout to avoid Play billing fees.

- [ ] **5.4 Google Sign-In (optional)**
  - For optional AI tutor (Google OAuth → Gemini pattern from Voix Vive).
  - Not required for core game.

### Phase 6: Cloud / Web Services (You mentioned setting up online — P2/P3)

- [ ] **6.1 Cloudflare Pages landing site**
  - One-page site with hero GIF, demo link, buy button.
  - Use Voix Vive companion-app deployment pattern.

- [ ] **6.2 Polar.sh checkout links**
  - "Unlock LitTCG Full Game — $9.99".
  - Works for web + Android direct sales.

- [ ] **6.3 itch.io page**
  - Host web demo + APK download.
  - Good for early adopters and press.

- [ ] **6.4 Supabase (avoid for MVP)**
  - Only if adding accounts/cloud save later.
  - Conflicts with COPPA/local-first positioning.

### Phase 7: CI/CD and Automation (P3)

- [ ] **7.1 GitHub Actions: Android build**
  - Build AAB on tagged release.
  - Cache NDK and cargo dependencies.

- [ ] **7.2 GitHub Actions: Deploy web demo**
  - `trunk build` + upload to Cloudflare Pages.

- [ ] **7.3 GitHub Actions: Google Play upload**
  - Upload AAB to Play Console internal track on release.
  - Requires service account JSON.

### Phase 8: Demo & Emulator Playtesting (P1 — For Wife Review / QA)

The goal of this phase is to **experience the game as a player** on Android, not just compile it.

- [x] **8.1 Emulator build pipeline**
  - [x] `scripts/build-apk.sh --emulator` produces `x86_64-linux-android` APK for emulator.
  - [x] `scripts/android_smoke_test.sh --emulator --duration N` builds, installs, captures logcat, and reports crashes.
  - [x] Using medium-phone AVD with API 36.
  - [x] Automated pass/fail based on a clean 60-second run verified (script filters app-process logs and checks for `Loading -> MainMenu` startup marker).

- [ ] **8.2 Screen mirroring / recording tools**
  - `adb shell screenrecord /sdcard/littcg_demo.mp4` for gameplay capture.
  - `scrcpy` for interactive mirroring during QA.
  - `adb logcat -s lit_tcg:* RustStdout:*` for filtered logs.

- [ ] **8.3 Android emulator smoke tests**
  - App launches without crash in emulator.
  - Main menu appears within 10 seconds.
  - Tap "Start" and reach `GameState::Collecting`.
  - Tap letters, submit a word, see pet reveal animation.
  - Verify no ANR (Application Not Responding) dialogs.

- [ ] **8.4 Touch playtest checklist**
  - Spell a 3-letter word using only touch.
  - Tap the submit button.
  - Observe card flip, particles, sound, and pet spawn.
  - Tap to continue back to collecting.
  - Check that UI buttons are large enough for a child's finger.

- [ ] **8.5 Performance capture on emulator**
  - `adb shell dumpsys gfxinfo com.littcg.game` to check frame times.
  - Target: stable 30 FPS on medium AVD.
  - Note thermal throttling or memory pressure warnings.

- [ ] **8.6 Real device sanity check**
  - Install APK on an actual Android phone/tablet.
  - Repeat smoke tests 8.3 and 8.4.
  - Verify audio plays and touch responds.

- [ ] **8.7 Demo recording for wife review**
  - Record a 60-second vertical or horizontal gameplay clip.
  - Include: menu → collect letters → spell word → pet reveal → return to play.
  - Save to `demo/android_emulator_demo_YYYY-MM-DD.mp4`.

- [x] **8.8 Automated emulator test script**
  - Write `scripts/android_smoke_test.sh` that:
    1. Builds debug APK.
    2. Waits for emulator/device.
    3. Installs and launches.
    4. Captures logcat for 30 seconds.
    5. Greps for panic / ANR / fatal errors.
    6. Exits non-zero if any crash detected.

- [ ] **8.9 Quest / XR demo build (optional)**
  - Build XR variant and sideload to Meta Quest 3 / 3S.
  - Verify hand tracking pinch works for letter selection.
  - Record clip for headset demo reel.

---

## 6. Decision Matrix

| Decision | Recommended Choice | Rationale |
|----------|-------------------|-------------|
| **Activity type** | `NativeActivity` for now | Already works; simpler; no AppCompat theme needed |
| **Build tool (dev)** | `cargo-ndk` + custom APK script | Matches `trinity-ndk`; fast iteration |
| **Build tool (release)** | Android Studio + Gradle | Required for AAB and Google Play |
| **Payment method** | Polar.sh web checkout | Avoids Play billing fees and children's-app IAP restrictions |
| **Distribution (MVP)** | Direct APK / itch.io | Avoids Play Store review delays and policies |
| **Distribution (scale)** | Google Play + AAB | Largest reach, but requires compliance work |
| **Cloud backend** | None for MVP | Preserves COPPA/local-first trust |

---

## 7. What to Do Right Now (Demo-Focused)

1. ✅ `LitTTC/scripts/build-apk.sh` — device + `--emulator` builds
2. ✅ `LitTTC/scripts/android_smoke_test.sh` — install, launch, logcat crash check
3. ✅ `LitTTC/scripts/generate_icon.py` — LitTCG-branded launcher icons
4. Launch Android Emulator API 36 medium phone:
   ```bash
   cd /home/joshua/LitTCG/LitTTC
   ./scripts/android_smoke_test.sh --emulator --duration 60
   ```
5. Play through menu → letters → word → pet reveal.
6. Record with `adb shell screenrecord /sdcard/littcg_demo.mp4` for wife review.

This gives you a **playable Android demo**, not just a compile check.

---

## 8. Important Notes

- **Do not switch to Kotlin/Jetpack XR for the demo.** The Bevy + NDK path is started and closer to shipping.
- **Google Play is not required for MVP.** Direct sales via Polar.sh and itch.io let you sell immediately without compliance overhead.
- **Children's app policies are strict.** If you do go to Play Store, expect extra review time and documentation requirements.
- **Android is a parallel platform, not the demo priority.** The 30-day demo sprint still focuses on web/Chromebook first.
- **Emulator testing is the fastest way to feel the player experience.** Use it before real-device testing to iterate on touch sizing, performance, and the reveal moment.
