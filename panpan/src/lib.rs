

// ==============================================
// panpan/src/lib.rs
// ==============================================

//! PanPan Game Engine
//! 
//! A simple 2D/3D game engine for Android.
//! 
//! # Example
//! ```
//! use panpan::*;
//! 
//! pub fn init() {
//!     // Initialize your game
//! }
//! 
//! pub fn render() {
//!     draw_text("Hello World", 100.0, 100.0, 3.0, Color::WHITE);
//! }
//! ```

pub use gl;

// ============================================
// FFI bindings to the generated JNI code
// ============================================

unsafe extern "C" {
    fn panpan_draw_text(
        text_ptr: *const u8,
        text_len: usize,
        x: f32,
        y: f32,
        scale: f32,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    );
}

// ============================================
// Color utilities
// ============================================

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
    
    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }
    
    pub const WHITE: Color = Color::new(1.0, 1.0, 1.0, 1.0);
    pub const BLACK: Color = Color::new(0.0, 0.0, 0.0, 1.0);
    pub const RED: Color = Color::new(1.0, 0.0, 0.0, 1.0);
    pub const GREEN: Color = Color::new(0.0, 1.0, 0.0, 1.0);
    pub const BLUE: Color = Color::new(0.0, 0.0, 1.0, 1.0);
    pub const YELLOW: Color = Color::new(1.0, 1.0, 0.0, 1.0);
    pub const CYAN: Color = Color::new(0.0, 1.0, 1.0, 1.0);
    pub const MAGENTA: Color = Color::new(1.0, 0.0, 1.0, 1.0);
}

// ============================================
// Text rendering API
// ============================================

/// Draw text on screen
/// 
/// # Arguments
/// * `text` - The text to draw
/// * `x` - X position in pixels (left edge)
/// * `y` - Y position in pixels (top edge)
/// * `scale` - Text scale factor (1.0 = 8x8 pixels per character)
/// * `color` - Text color
/// 
/// # Example
/// ```
/// panpan::draw_text("Score: 100", 10.0, 10.0, 2.0, Color::WHITE);
/// ```
pub fn draw_text(text: &str, x: f32, y: f32, scale: f32, color: Color) {
    unsafe {
        panpan_draw_text(
            text.as_ptr(),
            text.len(),
            x,
            y,
            scale,
            color.r,
            color.g,
            color.b,
            color.a,
        );
    }
}

/// Draw text with individual RGB values
pub fn draw_text_rgb(text: &str, x: f32, y: f32, scale: f32, r: f32, g: f32, b: f32) {
    draw_text(text, x, y, scale, Color::rgb(r, g, b));
}

// ============================================
// OpenGL convenience wrappers
// ============================================

/// Clear the screen with a color
pub fn clear_color(color: Color) {
    unsafe {
        gl::ClearColor(color.r, color.g, color.b, color.a);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }
}

/// Clear the screen with RGB values
pub fn clear_rgb(r: f32, g: f32, b: f32) {
    clear_color(Color::rgb(r, g, b));
}

// ============================================
// 2D Drawing primitives (future expansion)
// ============================================

/// Draw a colored rectangle
pub fn draw_rect(x: f32, y: f32, width: f32, height: f32, color: Color) {
    // TODO: Implement rectangle drawing
    // For now, users can use raw OpenGL
}

/// Draw a line
pub fn draw_line(x1: f32, y1: f32, x2: f32, y2: f32, thickness: f32, color: Color) {
    // TODO: Implement line drawing
}

// ============================================
// Screen info
// ============================================

static mut SCREEN_INFO: ScreenInfo = ScreenInfo {
    width: 800,
    height: 600,
};

#[derive(Debug, Clone, Copy)]
pub struct ScreenInfo {
    pub width: i32,
    pub height: i32,
}

/// Get current screen dimensions
pub fn screen() -> ScreenInfo {
    unsafe { SCREEN_INFO }
}

/// Internal function called by the JNI layer
#[doc(hidden)]
#[unsafe(no_mangle)]
pub extern "C" fn panpan_internal_set_screen_size(width: i32, height: i32) {
    unsafe {
        SCREEN_INFO = ScreenInfo { width, height };
    }
}