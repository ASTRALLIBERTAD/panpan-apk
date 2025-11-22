# panpan-apk Starter Repo

This archive contains a starter implementation of **panpan-apk**:
- `tools/panpan-apk/` — Rust CLI starter tool (generator skeleton).
- `android/` — Android template (minimal, uses GLSurfaceView).
- `example_crate/` — Example Rust library crate (`libpanpan.so`) with `pub fn init/resize/render`.
- `scripts/` — Cross-platform build scripts (`build-android.sh`, `build-android.ps1`).

Quick start:
1. Install Rust toolchain and `cargo-ndk`.
2. Ensure Android NDK/SDK and `adb`, and `gradle` are available.
3. Build and install `panpan-apk`:
   ```
   cd tools/panpan-apk
   cargo install --path .
   ```
4. Run:
   ```
   panpan-apk --crate-path ./example_crate --android-template ./android --release --install
   ```

For app debugging (Windows):
1. Get your app’s PID.
   Run this in PowerShell or CMD:
   ```
   adb shell pidof com.lucidum.panpan
   ```
   Suppose it returns something like: `15322`
2. Save only your app’s error logs to crash.log.

   CMD version:
   ```
   adb logcat --pid=15322 *:E > crash.log
   ```

   PowerShell version:
   ```
   adb logcat --pid=15322 *:E | Out-File crash.log
   ```