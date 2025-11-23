mod engine;
mod platform;
mod renderer;
mod util;

pub use crate::util::{Color, Vec2};

// Export touch handler registration for user code
pub use crate::engine::set_touch_handlers;

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

pub fn clear_screen(r: f32, g: f32, b: f32, a: f32) {
    engine::clear_screen(r, g, b, a);
}

pub fn draw_text(text: &str, x: f32, y: f32, scale: f32, color: Color) {
    engine::draw_text(text, x, y, scale, color);
}

pub fn draw_rect(x: f32, y: f32, w: f32, h: f32, color: Color) {
    engine::draw_rect(x, y, w, h, color);
}

pub fn touch_down(id: i32, x: f32, y: f32) { engine::touch_down(id, x, y); }
pub fn touch_move(id: i32, x: f32, y: f32) { engine::touch_move(id, x, y); }
pub fn touch_up(id: i32) { engine::touch_up(id); }

pub fn set_platform(p: &'static dyn platform::Platform) { platform::set_platform(p); }

// Internal API functions for panpan-apk JNI wrapper
#[doc(hidden)]
pub fn panpan_internal_set_screen_size(width: i32, height: i32) {
    resize(width, height);
}

#[doc(hidden)]
pub fn panpan_internal_update_time(_dt: f32) {
    // Update timing state if needed
}

#[doc(hidden)]
pub fn panpan_internal_touch_down(id: i32, x: f32, y: f32) {
    touch_down(id, x, y);
}

#[doc(hidden)]
pub fn panpan_internal_touch_move(id: i32, x: f32, y: f32) {
    touch_move(id, x, y);
}

#[doc(hidden)]
pub fn panpan_internal_touch_up(id: i32) {
    touch_up(id);
}