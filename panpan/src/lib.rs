// panpan/src/lib.rs
// Core engine API - platform agnostic

pub mod types;
pub mod graphics;
pub mod input;

// Re-export commonly used items
pub use types::{Color, Vec2, Rect};
pub use graphics::{clear_screen, draw_rect, draw_circle, draw_text};
pub use input::{Touch, TouchPhase, Key, InputEvent};

/// Main game trait that users must implement
pub trait Game: Sized {
    /// Create a new game instance
    fn new() -> Self;
    
    /// Update game logic
    fn update(&mut self, dt: f32);
    
    /// Render the game
    fn render(&self);
    
    /// Optional: handle input events
    fn on_touch_down(&mut self, _id: i32, _x: f32, _y: f32) {}
    fn on_touch_move(&mut self, _id: i32, _x: f32, _y: f32) {}
    fn on_touch_up(&mut self, _id: i32) {}
}

/// Internal: Runner will call this to initialize the rendering backend
#[doc(hidden)]
pub fn __internal_init_graphics(gl: glow::Context) {
    graphics::init(gl);
}

/// Internal: Runner will call this on resize
#[doc(hidden)]
pub fn __internal_resize(width: i32, height: i32) {
    graphics::set_viewport(width, height);
}