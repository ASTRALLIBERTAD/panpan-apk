# Build Android APK for PanPan Demo Game
# Run this script from panpan-apk-starter root

Write-Host "Building PanPan for Android..." -ForegroundColor Cyan

# Step 1: Build Rust library
Write-Host "`n[1/4] Building Rust library for arm64..." -ForegroundColor Yellow
cd runners\android\jni_wrapper
cargo ndk --target aarch64-linux-android --platform 21 build --release

if ($LASTEXITCODE -ne 0) {
    Write-Host "Failed to build Rust library!" -ForegroundColor Red
    exit 1
}

# Step 2: Copy .so file
Write-Host "`n[2/4] Copying library to Android project..." -ForegroundColor Yellow
New-Item -ItemType Directory -Force android\app\src\main\jniLibs\arm64-v8a | Out-Null
Copy-Item target\aarch64-linux-android\release\libdemo_game_android.so `
          android\app\src\main\jniLibs\arm64-v8a\libpanpan.so -Force

# Step 3: Build APK
Write-Host "`n[3/4] Building APK..." -ForegroundColor Yellow
cd android
.\gradlew assembleDebug

if ($LASTEXITCODE -ne 0) {
    Write-Host "Failed to build APK!" -ForegroundColor Red
    exit 1
}

# Step 4: Install (if device connected)
Write-Host "`n[4/4] Installing on device..." -ForegroundColor Yellow
adb devices
$apkPath = "app\build\outputs\apk\debug\app-debug.apk"

if (Test-Path $apkPath) {
    adb install -r $apkPath
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "`n✓ SUCCESS! APK installed on device" -ForegroundColor Green
        Write-Host "APK location: runners\android\jni_wrapper\android\$apkPath" -ForegroundColor Cyan
    } else {
        Write-Host "`n✓ APK built successfully (install failed - is device connected?)" -ForegroundColor Yellow
        Write-Host "APK location: runners\android\jni_wrapper\android\$apkPath" -ForegroundColor Cyan
        Write-Host "Manual install: adb install -r $apkPath" -ForegroundColor Yellow
    }
} else {
    Write-Host "APK not found at expected location!" -ForegroundColor Red
}
