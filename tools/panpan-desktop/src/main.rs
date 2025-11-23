// tools/panpan-desktop/src/main.rs
use clap::Parser;
use std::path::Path;
use std::process::Command;
use anyhow::{Context, Result};

#[derive(Parser)]
struct Args {
    /// Path to the crate to run
    #[arg(short, long, default_value = ".")]
    crate_path: String,
    
    /// Run in release mode
    #[arg(long, default_value_t = false)]
    release: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let crate_path = Path::new(&args.crate_path).canonicalize()
        .context("Failed to resolve crate path")?;
    
    println!("Building desktop runner for: {}", crate_path.display());
    
    // Create a temporary runner binary
    let runner_dir = crate_path.join("target/panpan_desktop_runner");
    if runner_dir.exists() {
        std::fs::remove_dir_all(&runner_dir)?;
    }
    std::fs::create_dir_all(runner_dir.join("src"))?;
    
    // Get crate name
    let crate_name = detect_crate_name(&crate_path)?;
    println!("Crate name: {}", crate_name);
    
    // Generate Cargo.toml
    let cargo_toml = format!(r#"[package]
name = "panpan_desktop_runner"
version = "0.1.0"
edition = "2021"

[dependencies]
{crate_name} = {{ path = "../.." }}
panpan = {{ path = "../../../panpan", features = ["desktop"] }}
winit = "0.30.12"
glutin = "0.32.3"
glutin-winit = "0.5"
raw-window-handle = "0.6"
glow = "0.16.0"
"#, crate_name = crate_name);
    
    std::fs::write(runner_dir.join("Cargo.toml"), cargo_toml)?;
    
    // Generate main.rs
    let main_rs = generate_runner_main(&crate_name);
    std::fs::write(runner_dir.join("src/main.rs"), main_rs)?;
    
    // Build the runner
    let mut cmd = Command::new("cargo");
    cmd.arg("build")
        .current_dir(&runner_dir);
    
    if args.release {
        cmd.arg("--release");
    }
    
    println!("Building desktop runner...");
    let status = cmd.status().context("Failed to build runner")?;
    if !status.success() {
        anyhow::bail!("Build failed");
    }
    
    // Run the binary
    let profile = if args.release { "release" } else { "debug" };
    let binary = runner_dir.join(format!("target/{}/panpan_desktop_runner.exe", profile));
    
    if !binary.exists() {
        anyhow::bail!("Built binary not found at: {}", binary.display());
    }
    
    println!("Running: {}", binary.display());
    let status = Command::new(&binary)
        .status()
        .context("Failed to run binary")?;
    
    if !status.success() {
        anyhow::bail!("Runner exited with error");
    }
    
    Ok(())
}

fn detect_crate_name(crate_path: &Path) -> Result<String> {
    let cargo_toml = std::fs::read_to_string(crate_path.join("Cargo.toml"))?;
    for line in cargo_toml.lines() {
        if line.trim_start().starts_with("name") {
            let parts: Vec<&str> = line.split('=').collect();
            if parts.len() >= 2 {
                let name = parts[1].trim().trim_matches('"').to_string();
                return Ok(name);
            }
        }
    }
    anyhow::bail!("Could not detect crate name")
}

fn generate_runner_main(crate_name: &str) -> String {
    let code = r#"
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId, WindowAttributes};
use glutin::config::ConfigTemplateBuilder;
use glutin::context::{ContextApi, ContextAttributesBuilder, PossiblyCurrentContext, Version};
use glutin::display::{Display, GetGlDisplay};
use glutin::prelude::*;
use glutin::surface::{Surface, SurfaceAttributesBuilder, WindowSurface};
use glutin_winit::DisplayBuilder;
use raw_window_handle::HasWindowHandle;
use std::num::NonZeroU32;

struct App {
    window: Option<Window>,
    gl_context: Option<PossiblyCurrentContext>,
    gl_surface: Option<Surface<WindowSurface>>,
    gl_display: Option<Display>,
}

impl App {
    fn new() -> Self {
        Self {
            window: None,
            gl_context: None,
            gl_surface: None,
            gl_display: None,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        let window_attributes = WindowAttributes::default()
            .with_title("PanPan Demo")
            .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0));

        let template = ConfigTemplateBuilder::new()
            .with_alpha_size(8)
            .with_transparency(false);

        let display_builder = DisplayBuilder::new();

        let (window, gl_config) = display_builder
            .build(event_loop, template, |configs| {
                configs
                    .reduce(|accum, config| {
                        if config.num_samples() > accum.num_samples() {
                            config
                        } else {
                            accum
                        }
                    })
                    .unwrap()
            })
            .unwrap();

        let window = window.unwrap_or_else(|| {
            event_loop.create_window(window_attributes).unwrap()
        });

        let gl_display = gl_config.display();

        let raw_window_handle = window.window_handle().unwrap().as_raw();

        let context_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::OpenGl(Some(Version::new(3, 3))))
            .build(Some(raw_window_handle));

        let not_current_gl_context = unsafe {
            gl_display
                .create_context(&gl_config, &context_attributes)
                .unwrap()
        };

        let size = window.inner_size();
        let width = NonZeroU32::new(size.width).unwrap();
        let height = NonZeroU32::new(size.height).unwrap();

        let attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
            raw_window_handle,
            width,
            height,
        );

        let gl_surface = unsafe {
            gl_display
                .create_window_surface(&gl_config, &attrs)
                .unwrap()
        };

        let gl_context = not_current_gl_context.make_current(&gl_surface).unwrap();

        let gl = unsafe {
            glow::Context::from_loader_function(|s| {
                gl_display.get_proc_address(&std::ffi::CString::new(s).unwrap())
            })
        };

        println!("OpenGL context created");

        // Initialize panpan with the GL context
        panpan::init(Some(gl));
        println!("Panpan initialized");

        // Call user's init
        CRATE_NAME::init();
        println!("User init complete");

        // Initial resize
        CRATE_NAME::resize(size.width as i32, size.height as i32);
        println!("Window size: {}x{}", size.width, size.height);

        self.window = Some(window);
        self.gl_context = Some(gl_context);
        self.gl_surface = Some(gl_surface);
        self.gl_display = Some(gl_display);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                println!("Resized: {}x{}", size.width, size.height);
                if let (Some(gl_surface), Some(gl_context)) = (&self.gl_surface, &self.gl_context) {
                    let width = NonZeroU32::new(size.width).unwrap_or(NonZeroU32::new(1).unwrap());
                    let height = NonZeroU32::new(size.height).unwrap_or(NonZeroU32::new(1).unwrap());
                    gl_surface.resize(gl_context, width, height);
                    CRATE_NAME::resize(size.width as i32, size.height as i32);
                }
            }
            WindowEvent::RedrawRequested => {
                CRATE_NAME::render();
                if let (Some(gl_surface), Some(gl_context)) = (&self.gl_surface, &self.gl_context) {
                    gl_surface.swap_buffers(gl_context).unwrap();
                }
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}

fn main() {
    println!("Starting PanPan Desktop Runner...");

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::new();
    event_loop.run_app(&mut app).unwrap();
}
"#;
    code.replace("CRATE_NAME", crate_name)
}