# ⚠️ CRITICAL FILES - DO NOT DELETE ⚠️

## These directories are REQUIRED for the project to work:

### ✅ MUST KEEP - Core Engine
```
panpan/
├── src/
│   ├── lib.rs
│   ├── graphics.rs
│   ├── types.rs
│   └── input.rs
└── Cargo.toml
```

### ✅ MUST KEEP - Platform Runners
```
runners/
├── desktop/
│   ├── src/lib.rs
│   └── Cargo.toml
│
└── android/
    ├── src/lib.rs          # Android runner (old, keep for reference)
    ├── jni_wrapper/        # ⚠️ CRITICAL - DO NOT DELETE!
    │   ├── Cargo.toml      # ⚠️ REQUIRED
    │   ├── src/lib.rs      # ⚠️ REQUIRED
    │   └── .cargo/config.toml  # ⚠️ REQUIRED
    │
    └── android/            # Android Studio project
        ├── app/
        ├── gradle/
        ├── gradlew.bat     # ⚠️ REQUIRED FOR BUILDING
        └── build.gradle
```

### ✅ MUST KEEP - Game & Tools
```
examples/
└── demo_game/
    ├── src/lib.rs
    └── Cargo.toml

demo_runner/
├── src/main.rs
└── Cargo.toml

tools/
└── panpan-cli/
    ├── src/main.rs
    └── Cargo.toml
```

## ❌ SAFE TO DELETE - Build Artifacts

These can be regenerated:
```
target/                    # All target directories
*.lock                     # Cargo.lock files  
.gradle/                   # Gradle cache
*/build/                   # Android build outputs
```

## 🔴 NEVER DELETE

These are critical and cannot be easily regenerated:
1. **`runners/android/jni_wrapper/`** - Android JNI bridge (connects Rust to Android)
2. **`runners/android/android/`** - Android Studio project
3. **`panpan/src/`** - Core engine source
4. **`examples/demo_game/src/`** - Your game code
5. **Any `.rs` or `.toml` files** - Source code!

## If You Accidentally Delete Something

Check `CRITICAL_FILES.md` for restore instructions or ask for help!
