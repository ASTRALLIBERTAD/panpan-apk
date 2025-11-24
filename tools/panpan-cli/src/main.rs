// tools/panpan-cli/src/main.rs
// Unified CLI tool for building and running PanPan games

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Parser)]
#[command(name = "panpan")]
#[command(about = "PanPan game engine CLI tool", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a game on desktop
    Run {
        /// Path to the game crate (default: current directory)
        #[arg(default_value = ".")]
        game: String,
    },

    /// Build for a platform
    Build {
        /// Target platform: desktop or android
        #[arg(long, default_value = "desktop")]
        platform: String,

        /// Build in release mode
        #[arg(long)]
        release: bool,

        /// Install APK after building (Android only)
        #[arg(long)]
        install: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { game } => {
            run_desktop(&game)?;
        }
        Commands::Build {
            platform,
            release,
            install,
        } => match platform.as_str() {
            "desktop" => build_desktop(release)?,
            "android" => build_android(release, install)?,
            _ => anyhow::bail!("Unknown platform: {}. Use 'desktop' or 'android'", platform),
        },
    }

    Ok(())
}

fn run_desktop(game_path: &str) -> Result<()> {
    println!("🚀 Running game on desktop...");

    let game_path = Path::new(game_path)
        .canonicalize()
        .context("Failed to find game directory")?;

    // Just use cargo run in demo_runner for now
    let runner_path = find_project_root()?.join("demo_runner");

    println!("   Building and running from: {}", runner_path.display());

    let status = Command::new("cargo")
        .arg("run")
        .current_dir(&runner_path)
        .status()?;

    if !status.success() {
        anyhow::bail!("Failed to run game");
    }

    Ok(())
}

fn build_desktop(release: bool) -> Result<()> {
    println!("🔨 Building for desktop...");

    let runner_path = find_project_root()?.join("demo_runner");

    let mut cmd = Command::new("cargo");
    cmd.arg("build").current_dir(&runner_path);

    if release {
        cmd.arg("--release");
        println!("   Mode: Release");
    } else {
        println!("   Mode: Debug");
    }

    let status = cmd.status()?;

    if !status.success() {
        anyhow::bail!("Build failed");
    }

    let profile = if release { "release" } else { "debug" };
    let exe_name = if cfg!(windows) {
        "demo_runner.exe"
    } else {
        "demo_runner"
    };
    let binary = runner_path.join("target").join(profile).join(exe_name);

    println!("✅ Build complete!");
    println!("   Binary: {}", binary.display());

    Ok(())
}

fn build_android(release: bool, install: bool) -> Result<()> {
    println!("📱 Building for Android...");

    let root = find_project_root()?;
    let jni_path = root.join("runners/android/jni_wrapper");

    // Step 1: Build Rust library
    println!("\n[1/4] Building Rust library for arm64...");

    let mut cmd = Command::new("cargo");
    cmd.arg("ndk")
        .arg("--target")
        .arg("aarch64-linux-android")
        .arg("--platform")
        .arg("21")
        .arg("build")
        .current_dir(&jni_path);

    if release {
        cmd.arg("--release");
        println!("   Mode: Release");
    } else {
        println!("   Mode: Debug");
    }

    let status = cmd
        .status()
        .context("Failed to run cargo-ndk. Install with: cargo install cargo-ndk")?;

    if !status.success() {
        anyhow::bail!("Rust build failed");
    }

    // Step 2: Copy .so file
    println!("\n[2/4] Copying library to Android project...");

    let profile = if release { "release" } else { "debug" };
    let so_source = jni_path.join(format!(
        "target/aarch64-linux-android/{}/libdemo_game_android.so",
        profile
    ));
    let so_dest_dir = root.join("runners/android/android/app/src/main/jniLibs/arm64-v8a");
    let so_dest = so_dest_dir.join("libpanpan.so");

    std::fs::create_dir_all(&so_dest_dir)?;
    std::fs::copy(&so_source, &so_dest)?;

    println!("   ✓ Copied to {}", so_dest.display());

    // Step 3: Build APK
    println!("\n[3/4] Building APK with Gradle...");

    let android_dir = root.join("runners/android/android");
    let gradlew = if cfg!(windows) {
        "gradlew.bat"
    } else {
        "./gradlew"
    };

    let gradle_task = if release {
        "assembleRelease"
    } else {
        "assembleDebug"
    };

    let status = Command::new(gradlew)
        .arg(gradle_task)
        .current_dir(&android_dir)
        .status()
        .context("Failed to run gradlew. Is Android SDK installed?")?;

    if !status.success() {
        anyhow::bail!("Gradle build failed");
    }

    let build_type = if release { "release" } else { "debug" };
    let apk_name = if release {
        "app-release-unsigned.apk"
    } else {
        "app-debug.apk"
    };
    let apk_path = android_dir.join(format!("app/build/outputs/apk/{}/{}", build_type, apk_name));

    println!("✅ APK built successfully!");
    println!("   Location: {}", apk_path.display());

    // Step 4: Install if requested
    if install {
        println!("\n[4/4] Installing APK on device...");

        let status = Command::new("adb")
            .arg("install")
            .arg("-r")
            .arg(&apk_path)
            .status()
            .context("Failed to run adb. Is it in your PATH?")?;

        if status.success() {
            println!("✅ Installed successfully!");
        } else {
            println!("⚠️  Install failed. Is a device connected?");
            println!("   Check with: adb devices");
        }
    } else {
        println!("\nTo install, run:");
        println!("   adb install -r {}", apk_path.display());
        println!("   or: panpan build --platform android --install");
    }

    Ok(())
}

fn find_project_root() -> Result<PathBuf> {
    // Try to find the project root by looking for panpan directory
    let current = std::env::current_dir()?;

    // Check current directory
    if current.join("panpan").join("Cargo.toml").exists() {
        return Ok(current);
    }

    // Check parent directory
    if let Some(parent) = current.parent() {
        if parent.join("panpan").join("Cargo.toml").exists() {
            return Ok(parent.to_path_buf());
        }
    }

    // Check if we're inside the project somewhere
    let mut path = current.clone();
    loop {
        if path.join("panpan").join("Cargo.toml").exists() {
            return Ok(path);
        }

        if !path.pop() {
            break;
        }
    }

    anyhow::bail!(
        "Could not find PanPan project root. Run this command from within your PanPan project directory.\n\
         Looking for: panpan/Cargo.toml"
    )
}
