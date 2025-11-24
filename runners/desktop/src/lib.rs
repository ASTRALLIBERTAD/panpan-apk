// runners/desktop/src/lib.rs
// Desktop runner using winit + glutin
// This is platform-specific code that creates a window and GL context

use glutin::config::ConfigTemplateBuilder;
use glutin::context::{ContextApi, ContextAttributesBuilder, PossiblyCurrentContext, Version};
use glutin::display::{Display, GetGlDisplay};
use glutin::prelude::*;
use glutin::surface::{Surface, SurfaceAttributesBuilder, WindowSurface};
use glutin_winit::DisplayBuilder;
use raw_window_handle::HasWindowHandle;
use std::num::NonZeroU32;
use std::time::Instant;
use winit::application::ApplicationHandler;
use winit::event::{StartCause, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};

// Import the game crate
use panpan::Game;

struct DesktopRunner<G: Game> {
    window: Option<Window>,
    gl_context: Option<PossiblyCurrentContext>,
    gl_surface: Option<Surface<WindowSurface>>,
    gl_display: Option<Display>,
    game: Option<G>,
    last_frame: Instant,
    cursor_pos: (f32, f32),
}

impl<G: Game> DesktopRunner<G> {
    fn new() -> Self {
        Self {
            window: None,
            gl_context: None,
            gl_surface: None,
            gl_display: None,
            game: None,
            last_frame: Instant::now(),
            cursor_pos: (0.0, 0.0),
        }
    }
}

impl<G: Game + 'static> ApplicationHandler for DesktopRunner<G> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        println!("Creating window and OpenGL context...");

        let window_attributes = WindowAttributes::default()
            .with_title("PanPan Game")
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
            .expect("Failed to create OpenGL config");

        let window = window.unwrap_or_else(|| {
            event_loop
                .create_window(window_attributes)
                .expect("Failed to create window")
        });

        let gl_display = gl_config.display();
        let raw_window_handle = window.window_handle().unwrap().as_raw();

        let context_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::OpenGl(Some(Version::new(3, 3))))
            .build(Some(raw_window_handle));

        let not_current_gl_context = unsafe {
            gl_display
                .create_context(&gl_config, &context_attributes)
                .expect("Failed to create OpenGL context")
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
                .expect("Failed to create OpenGL surface")
        };

        let gl_context = not_current_gl_context
            .make_current(&gl_surface)
            .expect("Failed to make OpenGL context current");

        println!("OpenGL context created successfully");

        // Create glow context
        let gl = unsafe {
            glow::Context::from_loader_function(|s| {
                gl_display.get_proc_address(&std::ffi::CString::new(s).unwrap())
            })
        };

        // Initialize panpan graphics backend
        panpan::__internal_init_graphics(gl);

        println!("PanPan graphics initialized");

        // Set initial viewport
        panpan::__internal_resize(size.width as i32, size.height as i32);

        // Create the game
        let game = G::new();
        println!("Game created");

        self.window = Some(window);
        self.gl_context = Some(gl_context);
        self.gl_surface = Some(gl_surface);
        self.gl_display = Some(gl_display);
        self.game = Some(game);
        self.last_frame = Instant::now();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                println!("Close requested, exiting...");
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                if let (Some(gl_surface), Some(gl_context)) = (&self.gl_surface, &self.gl_context) {
                    let width = NonZeroU32::new(size.width).unwrap_or(NonZeroU32::new(1).unwrap());
                    let height =
                        NonZeroU32::new(size.height).unwrap_or(NonZeroU32::new(1).unwrap());
                    gl_surface.resize(gl_context, width, height);
                    panpan::__internal_resize(size.width as i32, size.height as i32);
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(game) = &mut self.game {
                    let now = Instant::now();
                    let dt = now.duration_since(self.last_frame).as_secs_f32();
                    self.last_frame = now;

                    // Update game
                    game.update(dt);

                    // Render game
                    game.render();

                    // Swap buffers
                    if let (Some(gl_surface), Some(gl_context)) =
                        (&self.gl_surface, &self.gl_context)
                    {
                        gl_surface.swap_buffers(gl_context).unwrap();
                    }
                }

                // Request next frame
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.cursor_pos = (position.x as f32, position.y as f32);
            }
            WindowEvent::MouseInput { state, .. } => {
                if let Some(game) = &mut self.game {
                    match state {
                        winit::event::ElementState::Pressed => {
                            game.on_touch_down(0, self.cursor_pos.0, self.cursor_pos.1);
                        }
                        winit::event::ElementState::Released => {
                            game.on_touch_up(0);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn new_events(&mut self, _event_loop: &ActiveEventLoop, cause: StartCause) {
        if matches!(cause, StartCause::Init) {
            if let Some(window) = &self.window {
                window.request_redraw();
            }
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}

pub fn run<G: Game + 'static>() {
    println!("Starting PanPan Desktop Runner...");

    let event_loop = EventLoop::new().expect("Failed to create event loop");
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = DesktopRunner::<G>::new();
    event_loop.run_app(&mut app).expect("Event loop error");
}
