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
    let crate_path = Path::new(&args.crate_path).canonicalize()
        .context("Failed to resolve crate path")?;

    let src_candidates = [
        crate_path.join("src/lib.rs"),
        crate_path.join("src/main.rs"),
    ];
    let src_file = src_candidates.iter().find(|p| p.exists()).context("Could not find src/lib.rs or src/main.rs")?;
    let src = fs::read_to_string(src_file)?;

    let fns = parse_exportable_functions(&src)?;
    println!("Found {} exportable functions: {:?}", fns.len(), fns);

    // create generated crate in target/panpan_jni
    let target = crate_path.join("target/panpan_jni");
    if target.exists() {
        fs::remove_dir_all(&target)?;
    }
    fs::create_dir_all(target.join("src"))?;

    let crate_name = detect_crate_name(&crate_path)?;
    
    // Find panpan crate - look in parent directory
    let panpan_path = crate_path.parent()
        .map(|p| p.join("panpan"))
        .filter(|p| p.join("Cargo.toml").exists());
    
    let panpan_dep = if let Some(pp) = panpan_path {
        // Make path relative to target/panpan_jni
        let relative = pathdiff::diff_paths(&pp, &target)
            .unwrap_or_else(|| pp.clone());
        // Convert backslashes to forward slashes for Cargo.toml compatibility
        let path_str = relative.display().to_string().replace('\\', "/");
        format!("panpan = {{ path = \"{}\" }}\n", path_str)
    } else {
        println!("Warning: panpan crate not found, some features may not work");
        String::new()
    };

    let cargo_toml = format!("[package]
name = \"panpan_jni\"
version = \"0.1.0\"
edition = \"2021\"

[lib]
crate-type = [\"cdylib\"]

[dependencies]
jni = \"0.21\"
gl = \"0.14\"
android_logger = \"0.13\"
log = \"0.4\"
{panpan_dep}{crate_name} = {{ path = \"../..\" }}
");
    fs::write(target.join("Cargo.toml"), cargo_toml)?;

    let lib_rs = generate_librs_with_opengl(&crate_name, &fns);
    fs::write(target.join("src/lib.rs"), lib_rs)?;

    let abis = vec![
        ("arm64-v8a", "aarch64-linux-android"),
        ("armeabi-v7a", "armv7-linux-androideabi"),
    ];
    for (abi, tgt) in &abis {
        let mut cmd = Command::new("cargo");
        cmd.arg("ndk")
            .arg("--target").arg(tgt)
            .arg("build");
        if args.release { cmd.arg("--release"); }
        cmd.current_dir(&target);

        println!("Running: {:?}", cmd);
        let status = cmd.status().context("failed to run cargo ndk")?;
        if !status.success() { anyhow::bail!("cargo ndk failed"); }
        
        let profile = if args.release { "release" } else { "debug" };
        let built = crate_path.join(format!("target/{}/{}/libpanpan_jni.so", tgt, profile));

        if !built.exists() {
            println!("Warning: expected built library at {} not found.", built.display());
        } else {
            let dest = Path::new(&args.android_template).join("app/src/main/jniLibs").join(abi);
            fs::create_dir_all(&dest)?;
            let dest_file = dest.join("libpanpan.so");
            fs::copy(&built, &dest_file)?;
            println!("Copied {} -> {}", built.display(), dest_file.display());
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

fn generate_librs_with_opengl(crate_name: &str, fns: &[String]) -> String {
    let has_init = fns.contains(&"init".to_string());
    let has_resize = fns.contains(&"resize".to_string());
    let has_render = fns.contains(&"render".to_string());

    // Generate wrapper functions that re-export user's pub fn with #[no_mangle]
    let mut wrapper_fns = String::new();
    
    if has_init {
        wrapper_fns.push_str(&format!(
            "#[no_mangle]\npub extern \"C\" fn panpan_user_init() {{\n    {}::init();\n}}\n\n",
            crate_name
        ));
    }
    
    if has_resize {
        wrapper_fns.push_str(&format!(
            "#[no_mangle]\npub extern \"C\" fn panpan_user_resize(width: i32, height: i32) {{\n    {}::resize(width, height);\n}}\n\n",
            crate_name
        ));
    }
    
    if has_render {
        wrapper_fns.push_str(&format!(
            "#[no_mangle]\npub extern \"C\" fn panpan_user_render() {{\n    {}::render();\n}}\n\n",
            crate_name
        ));
    }

    let init_call = if has_init {
        "    panpan_user_init();\n"
    } else {
        "    // No init() function found in user crate\n"
    };

    let resize_call = if has_resize {
        "    panpan_user_resize(width as i32, height as i32);\n    panpan::panpan_internal_set_screen_size(width as i32, height as i32);\n"
    } else {
        "    // No resize() function found in user crate\n"
    };

    let render_call = if has_render {
        "    panpan_user_render();\n"
    } else {
        "    // No render() function found in user crate\n"
    };

    let mut code = String::new();
    
    code.push_str(r#"
use jni::objects::JClass;
use jni::sys::jint;
use jni::JNIEnv;
use std::ffi::CString;
use std::sync::{Mutex, Once};

// OpenGL function loader
extern "C" {
    fn eglGetProcAddress(procname: *const std::os::raw::c_char) -> *const std::os::raw::c_void;
}

// Global text renderer
static TEXT_RENDERER: Mutex<Option<TextRenderer>> = Mutex::new(None);
static mut SCREEN_WIDTH: f32 = 800.0;
static mut SCREEN_HEIGHT: f32 = 600.0;

struct TextRenderer {
    shader_program: u32,
    vao: u32,
    vbo: u32,
    texture: u32,
}

impl TextRenderer {
    fn new() -> Self {
        unsafe {
            let vs = Self::compile_shader(VERTEX_SHADER, gl::VERTEX_SHADER);
            let fs = Self::compile_shader(FRAGMENT_SHADER, gl::FRAGMENT_SHADER);
            let program = gl::CreateProgram();
            gl::AttachShader(program, vs);
            gl::AttachShader(program, fs);
            gl::LinkProgram(program);
            gl::DeleteShader(vs);
            gl::DeleteShader(fs);

            let mut vao = 0;
            let mut vbo = 0;
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (6 * 4 * std::mem::size_of::<f32>()) as isize,
                std::ptr::null(),
                gl::DYNAMIC_DRAW,
            );
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                4,
                gl::FLOAT,
                gl::FALSE,
                (4 * std::mem::size_of::<f32>()) as i32,
                std::ptr::null(),
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);

            let mut texture = 0;
            gl::GenTextures(1, &mut texture);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

            Self::load_font_atlas(texture);
            Self { shader_program: program, vao, vbo, texture }
        }
    }

    unsafe fn compile_shader(src: &str, ty: u32) -> u32 {
        let shader = gl::CreateShader(ty);
        let c_str = CString::new(src.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);
        shader
    }

    unsafe fn load_font_atlas(texture: u32) {
        const W: usize = 128;
        const H: usize = 48;
        let mut atlas = vec![0u8; W * H];

        for ch in 0..96 {
            let cx = (ch % 16) * 8;
            let cy = (ch / 16) * 8;
            for y in 1..7 {
                for x in 1..7 {
                    atlas[(cy + y) * W + (cx + x)] = 255;
                }
            }
        }

        gl::BindTexture(gl::TEXTURE_2D, texture);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RED as i32,
            W as i32,
            H as i32,
            0,
            gl::RED,
            gl::UNSIGNED_BYTE,
            atlas.as_ptr() as *const _,
        );
    }

    fn render(&mut self, text: &str, x: f32, y: f32, scale: f32, color: [f32; 4], sw: f32, sh: f32) {
        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::UseProgram(self.shader_program);

            let color_loc = gl::GetUniformLocation(
                self.shader_program,
                CString::new("textColor").unwrap().as_ptr()
            );
            gl::Uniform4f(color_loc, color[0], color[1], color[2], color[3]);

            let proj_loc = gl::GetUniformLocation(
                self.shader_program,
                CString::new("projection").unwrap().as_ptr()
            );
            let proj = [
                2.0/sw, 0.0,    0.0, 0.0,
                0.0,  -2.0/sh, 0.0, 0.0,
                0.0,   0.0,   -1.0, 0.0,
                -1.0,  1.0,    0.0, 1.0,
            ];
            gl::UniformMatrix4fv(proj_loc, 1, gl::FALSE, proj.as_ptr());

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.texture);
            gl::BindVertexArray(self.vao);

            let cw = 8.0 * scale;
            let ch = 8.0 * scale;

            for (i, c) in text.chars().enumerate() {
                let code = c as i32;
                if code < 32 || code > 127 { continue; }
                let idx = code - 32;
                let tx = (idx % 16) as f32 / 16.0;
                let ty = (idx / 16) as f32 / 6.0;
                let tw = 1.0 / 16.0;
                let th = 1.0 / 6.0;

                let xp = x + (i as f32) * cw;
                let yp = y;

                let verts: [f32; 24] = [
                    xp, yp+ch, tx,    ty,
                    xp, yp,    tx,    ty+th,
                    xp+cw, yp, tx+tw, ty+th,

                    xp, yp+ch, tx,    ty,
                    xp+cw, yp, tx+tw, ty+th,
                    xp+cw, yp+ch, tx+tw, ty,
                ];

                gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
                gl::BufferSubData(
                    gl::ARRAY_BUFFER,
                    0,
                    (verts.len() * std::mem::size_of::<f32>()) as isize,
                    verts.as_ptr() as *const _
                );
                gl::DrawArrays(gl::TRIANGLES, 0, 6);
            }

            gl::BindVertexArray(0);
            gl::BindTexture(gl::TEXTURE_2D, 0);
            gl::Disable(gl::BLEND);
        }
    }
}
"#);

    // Add shader constants as strings
    code.push_str("\nconst VERTEX_SHADER: &str = r###\"#version 300 es\n");
    code.push_str("layout (location = 0) in vec4 vertex;\n");
    code.push_str("out vec2 TexCoords;\n");
    code.push_str("uniform mat4 projection;\n");
    code.push_str("void main() {\n");
    code.push_str("    gl_Position = projection * vec4(vertex.xy, 0.0, 1.0);\n");
    code.push_str("    TexCoords = vertex.zw;\n");
    code.push_str("}\n");
    code.push_str("\"###;\n\n");
    
    code.push_str("const FRAGMENT_SHADER: &str = r###\"#version 300 es\n");
    code.push_str("precision mediump float;\n");
    code.push_str("in vec2 TexCoords;\n");
    code.push_str("out vec4 color;\n");
    code.push_str("uniform sampler2D text;\n");
    code.push_str("uniform vec4 textColor;\n");
    code.push_str("void main() {\n");
    code.push_str("    float alpha = texture(text, TexCoords).r;\n");
    code.push_str("    color = vec4(textColor.rgb, textColor.a * alpha);\n");
    code.push_str("}\n");
    code.push_str("\"###;\n\n");
    
    code.push_str(r#"
static INIT_LOGGER: Once = Once::new();

fn init_logging() {
    INIT_LOGGER.call_once(|| {
        android_logger::init_once(
            android_logger::Config::default()
                .with_max_level(log::LevelFilter::Debug)
                .with_tag("PanPan")
        );
    });
}

// Wrapper functions to call user code
"#);

    code.push_str(&wrapper_fns);
    
    code.push_str(r#"
#[no_mangle]
pub extern "C" fn panpan_draw_text(
    text_ptr: *const u8,
    text_len: usize,
    x: f32,
    y: f32,
    scale: f32,
    red: f32,
    green: f32,
    blue: f32,
    alpha: f32,
) {
    let text = unsafe {
        std::str::from_utf8_unchecked(std::slice::from_raw_parts(text_ptr, text_len))
    };
    
    if let Ok(mut renderer) = TEXT_RENDERER.lock() {
        if let Some(ref mut r) = *renderer {
            unsafe {
                r.render(text, x, y, scale, [red, green, blue, alpha], SCREEN_WIDTH, SCREEN_HEIGHT);
            }
        }
    }
}

// JNI ENTRYPOINTS -------------------------------------------------------

#[no_mangle]
pub extern "C" fn Java_com_lucidum_panpan_MainActivity_nativeInit(_env: JNIEnv, _class: JClass) {
    init_logging();
    
    unsafe {
        gl::load_with(|s| {
            let c_str = CString::new(s).unwrap();
            eglGetProcAddress(c_str.as_ptr()) as *const _
        });
        
        gl::ClearColor(0.1, 0.2, 0.3, 1.0);
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LEQUAL);

        if let Ok(mut renderer) = TEXT_RENDERER.lock() {
            *renderer = Some(TextRenderer::new());
        }
    }
"#);

    code.push_str(init_call);
    code.push_str("}\n\n");
    
    code.push_str(r#"#[no_mangle]
pub extern "C" fn Java_com_lucidum_panpan_MainActivity_nativeResize(_env: JNIEnv, _class: JClass, width: jint, height: jint) {
    unsafe {
        gl::Viewport(0, 0, width, height);
        SCREEN_WIDTH = width as f32;
        SCREEN_HEIGHT = height as f32;
    }
"#);

    code.push_str(resize_call);
    code.push_str("}\n\n");
    
    code.push_str(r#"#[no_mangle]
pub extern "C" fn Java_com_lucidum_panpan_MainActivity_nativeRender(_env: JNIEnv, _class: JClass) {
    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }
"#);

    code.push_str(render_call);
    code.push_str("}\n");
    
    code
}