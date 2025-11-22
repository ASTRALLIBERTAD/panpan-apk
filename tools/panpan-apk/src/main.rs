use anyhow::{Context, Result};
use clap::Parser;
use regex::Regex;
use std::{fs, path::Path, process::Command};

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value = ".")]
    crate_path: String,
    #[arg(short, long, default_value = "android")]
    android_template: String,
    #[arg(long, default_value_t = false)]
    release: bool,
    #[arg(long, default_value_t = false)]
    install: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let crate_path = Path::new(&args.crate_path);

    let src_candidates = [
        crate_path.join("src/lib.rs"),
        crate_path.join("src/main.rs"),
    ];
    let src_file = src_candidates.iter().find(|p| p.exists()).context("Could not find src/lib.rs or src/main.rs")?;
    let src = fs::read_to_string(src_file)?;

    let fns = parse_exportable_functions(&src)?;
    if fns.is_empty() {
        println!("No exportable functions found (pub fn or #[panpan_export]). Exiting.");
        return Ok(());
    }

    println!("Found {} exportable functions (will generate JNI stubs).", fns.len());

    // create generated crate in target/panpan_jni
    let target = crate_path.join("target/panpan_jni");
    if target.exists() {
        fs::remove_dir_all(&target)?;
    }
    fs::create_dir_all(target.join("src"))?;

    let crate_name = detect_crate_name(crate_path)?;
    let cargo_toml = format!("[package]
name = \"panpan_jni\"
version = \"0.1.0\"
edition = \"2021\"

[lib]
crate-type = [\"cdylib\"]

[dependencies]
jni = \"0.21\"
{crate_name} = {{ path = \"../..\" }}
", crate_name = crate_name);
    fs::write(target.join("Cargo.toml"), cargo_toml)?;

    let lib_rs = generate_basic_librs();
    fs::write(target.join("src/lib.rs"), lib_rs)?;

    let abis = vec![
        ("arm64-v8a", "aarch64-linux-android"),
        ("armeabi-v7a", "armv7-linux-androideabi"),
    ];
    for (abi, tgt) in &abis {
        let mut cmd = Command::new("cargo");
        cmd.arg("ndk")
            .arg("--target").arg(tgt)
            .arg("build");  // Remove the canonicalize line
        if args.release { cmd.arg("--release"); }
        cmd.current_dir(&target);  // This is correct - run from the crate directory

        println!("Running: {:?}", cmd);
        let status = cmd.status().context("failed to run cargo ndk")?;
        if !status.success() { anyhow::bail!("cargo ndk failed"); }
        let built = crate_path.join(format!("target/{}/release/libpanpan_jni.so", tgt));
        let built_debug = crate_path.join(format!("target/{}/debug/libpanpan_jni.so", tgt));
        let found = if built.exists() { built } else if built_debug.exists() { built_debug } else {
            target.join(format!("target/{}/release/libpanpan_jni.so", tgt))
        };

        if !found.exists() {
            println!("Warning: expected built library at {} not found.", found.display());
        } else {
            let dest = Path::new(&args.android_template).join("app/src/main/jniLibs").join(abi);
            fs::create_dir_all(&dest)?;
            let dest_file = dest.join("libpanpan.so");
            fs::copy(&found, &dest_file)?;
            println!("Copied {} -> {}", found.display(), dest_file.display());
        }
    }

    let gradle_dir = Path::new(&args.android_template);
    let gradle_exec = if cfg!(windows) { gradle_dir.join("gradlew.bat") } else { gradle_dir.join("gradlew") };
    let mut gradle = Command::new(gradle_exec);
    gradle.current_dir(gradle_dir).arg("assembleDebug");
    let status = gradle.status().context("failed to run gradle")?;
    if !status.success() { anyhow::bail!("gradle assemble failed"); }

    let apk = gradle_dir.join("app/build/outputs/apk/debug/app-debug.apk");
    println!("APK build location: {}", apk.display());

    if args.install {
        Command::new("adb").arg("install").arg("-r").arg(apk).status().context("adb install failed")?;
        println!("Installed APK to device");
    }

    Ok(())
}

fn parse_exportable_functions(src: &str) -> Result<Vec<String>, anyhow::Error> {
    let mut out = Vec::new();
    let re_pub = Regex::new(r"pub\s+fn\s+([a-zA-Z0-9_]+)").unwrap();
    for cap in re_pub.captures_iter(src) {
        out.push(cap[1].to_string());
    }
    let re_attr = Regex::new(r"#\s*\[\s*panpan_export\s*\]\s*pub\s+fn\s+([a-zA-Z0-9_]+)").unwrap();
    for cap in re_attr.captures_iter(src) {
        if !out.contains(&cap[1].to_string()) {
            out.push(cap[1].to_string());
        }
    }
    Ok(out)
}

fn detect_crate_name(crate_path: &Path) -> Result<String, anyhow::Error> {
    let cargo = fs::read_to_string(crate_path.join("Cargo.toml"))?;
    for line in cargo.lines() {
        if line.trim_start().starts_with("name") {
            let parts: Vec<&str> = line.split('=').collect();
            if parts.len() >= 2 {
                let n = parts[1].trim().trim_matches('"').to_string();
                return Ok(n);
            }
        }
    }
    Err(anyhow::anyhow!("Could not detect crate name"))
}

fn generate_basic_librs() -> String {
    r#"
use jni::objects::JClass;
use jni::sys::{jint};
use jni::JNIEnv;

#[no_mangle]
pub extern "C" fn Java_com_lucidum_panpan_MainActivity_nativeInit(_env: JNIEnv, _class: JClass) {
    // call user's crate init() if present
    // e.g., your crate should define: pub fn init() { ... }
    // crate::init();
}

#[no_mangle]
pub extern "C" fn Java_com_lucidum_panpan_MainActivity_nativeResize(_env: JNIEnv, _class: JClass, width: jint, height: jint) {
    // crate::resize(width as i32, height as i32);
}

#[no_mangle]
pub extern "C" fn Java_com_lucidum_panpan_MainActivity_nativeRender(_env: JNIEnv, _class: JClass) {
    // crate::render();
}
"#
    .to_string()
}
