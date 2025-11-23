mod engine;
mod platform;
mod renderer;
mod util;

pub use crate::util::{Color, Vec2};

/// Initialize engine with optional GL context for desktop/Android
pub fn init(gl: Option<glow::Context>) {
    engine::init(gl);
}

pub fn resize(width: i32, height: i32) {
    engine::resize(width, height);
}

pub fn render() {
    engine::render();
}

pub fn touch_down(id: i32, x: f32, y: f32) { engine::touch_down(id, x, y); }
pub fn touch_move(id: i32, x: f32, y: f32) { engine::touch_move(id, x, y); }
pub fn touch_up(id: i32) { engine::touch_up(id); }

pub fn set_platform(p: &'static dyn platform::Platform) { platform::set_platform(p); }
