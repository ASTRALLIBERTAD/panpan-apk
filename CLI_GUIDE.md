# PanPan CLI - Simple Commands ✨

The `panpan` command is now installed! Use it from anywhere in your project.

## Quick Commands

### Run on Desktop
```powershell
panpan run
```

### Build for Android
```powershell
panpan build --platform android
```

### Build and Install on Android
```powershell
panpan build --platform android --install
```

### Build Desktop Release
```powershell
panpan build --platform desktop --release
```

## All Commands

```
panpan run                              # Run on desktop
panpan build --platform desktop         # Build desktop (debug)
panpan build --platform desktop --release  # Build desktop (release)
panpan build --platform android         # Build Android APK
panpan build --platform android --install  # Build + install APK
panpan build --platform android --release  # Build release APK
```

## How It Works

The CLI automatically:
1. ✅ Finds your project root
2. ✅ Builds Rust libraries (with cargo-ndk for Android)
3. ✅ Copies files to correct locations
4. ✅ Runs Gradle for Android
5. ✅ Installs APK if requested

## Examples

### Daily Development
```powershell
# Test on desktop quickly
cd c:\Users\Boss\Documents\panpan-apk-starter
panpan run

# Build APK and test on phone
panpan build --platform android --install
```

### Release Builds
```powershell
# Desktop release
panpan build --platform desktop --release

# Android release (needs signing)
panpan build --platform android --release
```

## Prerequisites

### First Time Only
```powershell
# Android targets
rustup target add aarch64-linux-android

# cargo-ndk
cargo install cargo-ndk
```

### Environment (Android)
- Android SDK installed
- `ANDROID_HOME` or `ANDROID_SDK_ROOT` set
- `adb` in PATH

## What Changed?

**Before:**
```powershell
cd runners\android\jni_wrapper
cargo ndk --target aarch64-linux-android --platform 21 build --release
New-Item -ItemType Directory -Force ..\android\app\src\main\jniLibs\arm64-v8a
Copy-Item target\aarch64-linux-android\release\libdemo_game_android.so ..\android\app\src\main\jniLibs\arm64-v8a\libpanpan.so
cd ..\android
.\gradlew.bat assembleDebug
adb install -r app\build\outputs\apk\debug\app-debug.apk
```

**Now:**
```powershell
panpan build --platform android --install
```

Much better! 🎉

## Troubleshooting

### "panpan: command not found"
The CLI is installed at: `C:\Users\Boss\.cargo\bin\panpan.exe`

Make sure `~\.cargo\bin` is in your PATH.

### "cargo-ndk not found"
```powershell
cargo install cargo-ndk
```

### "Could not find PanPan project root"
Run the command from within your project directory:
```powershell
cd c:\Users\Boss\Documents\panpan-apk-starter
panpan build --platform android
```

### "adb: command not found"
Add Android platform-tools to PATH or use full path to adb.

## Success! 

You now have a professional game engine CLI:
- ✅ Simple commands
- ✅ Cross-platform builds
- ✅ No complicated scripts
- ✅ Works from anywhere in project

Just like Unity, Unreal, or Godot - but written entirely in Rust! 🦀
