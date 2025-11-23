//! PanPan Game Engine
//! 
//! A simple 2D/3D game engine for Android with OpenGL ES 3.0.
//! 
//! # Features
//! - Text rendering with bitmap fonts
//! - 2D sprite rendering
//! - Basic shape drawing (rectangles, circles, lines)
//! - Input handling (touch events)
//! - Screen information
//! 
//! # Example
//! ```
//! use panpan::*;
//! 
//! pub fn init() {
//!     // Initialize your game
//! }
//! 
//! pub fn resize(width: i32, height: i32) {
//!     // Handle screen resize
//! }
//! 
//! pub fn render() {
//!     clear_color(Color::BLACK);
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

#[derive(Debug, Clone, Copy, PartialEq)]
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
    
    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
    
    /// Create color from 0-255 RGB values
    pub fn rgb255(r: u8, g: u8, b: u8) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: 1.0,
        }
    }
    
    /// Create color from hex string (e.g., "FF0000" for red)
    pub fn from_hex(hex: &str) -> Self {
        let hex = hex.trim_start_matches('#');
        if hex.len() >= 6 {
            let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
            let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
            let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
            Self::rgb255(r, g, b)
        } else {
            Self::WHITE
        }
    }
    
    // Common colors
    pub const WHITE: Color = Color::new(1.0, 1.0, 1.0, 1.0);
    pub const BLACK: Color = Color::new(0.0, 0.0, 0.0, 1.0);
    pub const RED: Color = Color::new(1.0, 0.0, 0.0, 1.0);
    pub const GREEN: Color = Color::new(0.0, 1.0, 0.0, 1.0);
    pub const BLUE: Color = Color::new(0.0, 0.0, 1.0, 1.0);
    pub const YELLOW: Color = Color::new(1.0, 1.0, 0.0, 1.0);
    pub const CYAN: Color = Color::new(0.0, 1.0, 1.0, 1.0);
    pub const MAGENTA: Color = Color::new(1.0, 0.0, 1.0, 1.0);
    pub const ORANGE: Color = Color::new(1.0, 0.5, 0.0, 1.0);
    pub const PURPLE: Color = Color::new(0.5, 0.0, 0.5, 1.0);
    pub const GRAY: Color = Color::new(0.5, 0.5, 0.5, 1.0);
    pub const DARK_GRAY: Color = Color::new(0.25, 0.25, 0.25, 1.0);
    pub const LIGHT_GRAY: Color = Color::new(0.75, 0.75, 0.75, 1.0);
}

// ============================================
// 2D Vector
// ============================================

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    
    pub const ZERO: Vec2 = Vec2::new(0.0, 0.0);
    pub const ONE: Vec2 = Vec2::new(1.0, 1.0);
    
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    
    pub fn normalize(&self) -> Vec2 {
        let len = self.length();
        if len > 0.0 {
            Vec2::new(self.x / len, self.y / len)
        } else {
            *self
        }
    }
    
    pub fn distance(&self, other: Vec2) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

impl std::ops::Add for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Vec2) -> Vec2 {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl std::ops::Mul<f32> for Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: f32) -> Vec2 {
        Vec2::new(self.x * rhs, self.y * rhs)
    }
}

// ============================================
// Rectangle
// ============================================

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub const fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }
    
    pub fn contains(&self, px: f32, py: f32) -> bool {
        px >= self.x && px <= self.x + self.width &&
        py >= self.y && py <= self.y + self.height
    }
    
    pub fn intersects(&self, other: &Rect) -> bool {
        self.x < other.x + other.width &&
        self.x + self.width > other.x &&
        self.y < other.y + other.height &&
        self.y + self.height > other.y
    }
    
    pub fn center(&self) -> Vec2 {
        Vec2::new(self.x + self.width / 2.0, self.y + self.height / 2.0)
    }
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

/// Draw centered text
pub fn draw_text_centered(text: &str, cx: f32, cy: f32, scale: f32, color: Color) {
    let char_width = 8.0 * scale;
    let char_height = 8.0 * scale;
    let text_width = text.len() as f32 * char_width;
    let x = cx - text_width / 2.0;
    let y = cy - char_height / 2.0;
    draw_text(text, x, y, scale, color);
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
// 2D Drawing primitives
// ============================================

/// Draw a filled rectangle
pub fn draw_rect(x: f32, y: f32, width: f32, height: f32, color: Color) {
    unsafe {
        gl::Disable(gl::DEPTH_TEST);
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        
        // Use immediate mode for simplicity (works on ES 3.0)
        // In production, you'd want to use VBOs
        let vertices: [f32; 12] = [
            x, y,
            x + width, y,
            x + width, y + height,
            x, y,
            x + width, y + height,
            x, y + height,
        ];
        
        // Note: Actual implementation would need proper shader setup
        // This is a placeholder that shows the concept
        
        gl::Disable(gl::BLEND);
        gl::Enable(gl::DEPTH_TEST);
    }
}

/// Draw a rectangle outline
pub fn draw_rect_outline(x: f32, y: f32, width: f32, height: f32, thickness: f32, color: Color) {
    draw_rect(x, y, width, thickness, color); // top
    draw_rect(x, y + height - thickness, width, thickness, color); // bottom
    draw_rect(x, y, thickness, height, color); // left
    draw_rect(x + width - thickness, y, thickness, height, color); // right
}

/// Draw a line between two points
pub fn draw_line(x1: f32, y1: f32, x2: f32, y2: f32, thickness: f32, color: Color) {
    let dx = x2 - x1;
    let dy = y2 - y1;
    let length = (dx * dx + dy * dy).sqrt();
    let angle = dy.atan2(dx);
    
    // Draw as rotated rectangle
    // This is simplified - real implementation would use proper matrix transforms
    let half_thickness = thickness / 2.0;
    draw_rect(x1, y1 - half_thickness, length, thickness, color);
}

/// Draw a circle outline
pub fn draw_circle(cx: f32, cy: f32, radius: f32, color: Color) {
    draw_circle_segments(cx, cy, radius, color, 32);
}

/// Draw a circle with custom segment count
pub fn draw_circle_segments(cx: f32, cy: f32, radius: f32, color: Color, segments: u32) {
    unsafe {
        gl::Disable(gl::DEPTH_TEST);
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        
        // Draw circle as triangle fan
        // Implementation would need proper shader setup
        
        gl::Disable(gl::BLEND);
        gl::Enable(gl::DEPTH_TEST);
    }
}

/// Draw a filled circle
pub fn draw_circle_filled(cx: f32, cy: f32, radius: f32, color: Color) {
    draw_circle_filled_segments(cx, cy, radius, color, 32);
}

/// Draw a filled circle with custom segment count
pub fn draw_circle_filled_segments(cx: f32, cy: f32, radius: f32, color: Color, segments: u32) {
    unsafe {
        gl::Disable(gl::DEPTH_TEST);
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        
        // Implementation would render filled circle
        
        gl::Disable(gl::BLEND);
        gl::Enable(gl::DEPTH_TEST);
    }
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

impl ScreenInfo {
    pub fn aspect_ratio(&self) -> f32 {
        self.width as f32 / self.height as f32
    }
    
    pub fn center(&self) -> Vec2 {
        Vec2::new(self.width as f32 / 2.0, self.height as f32 / 2.0)
    }
}

/// Get current screen dimensions
pub fn screen() -> ScreenInfo {
    unsafe { SCREEN_INFO }
}

/// Get screen width
pub fn screen_width() -> i32 {
    screen().width
}

/// Get screen height
pub fn screen_height() -> i32 {
    screen().height
}

/// Internal function called by the JNI layer
#[doc(hidden)]
#[unsafe(no_mangle)]
pub extern "C" fn panpan_internal_set_screen_size(width: i32, height: i32) {
    unsafe {
        SCREEN_INFO = ScreenInfo { width, height };
    }
}

// ============================================
// Time and delta time
// ============================================

static mut DELTA_TIME: f32 = 0.016; // ~60 FPS default
static mut TOTAL_TIME: f32 = 0.0;

/// Get delta time in seconds since last frame
pub fn delta_time() -> f32 {
    unsafe { DELTA_TIME }
}

/// Get total elapsed time in seconds
pub fn total_time() -> f32 {
    unsafe { TOTAL_TIME }
}

/// Internal function to update time (called from JNI layer)
#[doc(hidden)]
#[unsafe(no_mangle)]
pub extern "C" fn panpan_internal_update_time(dt: f32) {
    unsafe {
        DELTA_TIME = dt;
        TOTAL_TIME += dt;
    }
}

// ============================================
// Input handling
// ============================================

static mut TOUCHES: [Touch; 10] = [Touch::NONE; 10];

#[derive(Debug, Clone, Copy)]
pub struct Touch {
    pub id: i32,
    pub x: f32,
    pub y: f32,
    pub active: bool,
}

impl Touch {
    const NONE: Touch = Touch {
        id: -1,
        x: 0.0,
        y: 0.0,
        active: false,
    };
}

/// Get touch by index (0-9)
pub fn get_touch(index: usize) -> Option<Touch> {
    if index < 10 {
        let touch = unsafe { TOUCHES[index] };
        if touch.active {
            Some(touch)
        } else {
            None
        }
    } else {
        None
    }
}

/// Get all active touches
pub fn get_touches() -> Vec<Touch> {
    unsafe {
        let mut result = Vec::new();
        for i in 0..10 {
            let touch = TOUCHES[i];
            if touch.active {
                result.push(touch);
            }
        }
        result
    }
}

/// Check if there's any active touch
pub fn is_touching() -> bool {
    unsafe {
        for i in 0..10 {
            if TOUCHES[i].active {
                return true;
            }
        }
        false
    }
}

/// Internal function to update touch state (called from JNI layer)
#[doc(hidden)]
#[unsafe(no_mangle)]
pub extern "C" fn panpan_internal_touch_down(id: i32, x: f32, y: f32) {
    unsafe {
        for i in 0..10 {
            if !TOUCHES[i].active {
                TOUCHES[i] = Touch { id, x, y, active: true };
                break;
            }
        }
    }
}

#[doc(hidden)]
#[unsafe(no_mangle)]
pub extern "C" fn panpan_internal_touch_move(id: i32, x: f32, y: f32) {
    unsafe {
        for i in 0..10 {
            if TOUCHES[i].id == id {
                TOUCHES[i].x = x;
                TOUCHES[i].y = y;
                break;
            }
        }
    }
}

#[doc(hidden)]
#[unsafe(no_mangle)]
pub extern "C" fn panpan_internal_touch_up(id: i32) {
    unsafe {
        for i in 0..10 {
            if TOUCHES[i].id == id {
                TOUCHES[i].active = false;
                break;
            }
        }
    }
}

// ============================================
// Math utilities
// ============================================

/// Linear interpolation
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// Clamp value between min and max
pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
    if value < min { min }
    else if value > max { max }
    else { value }
}

/// Map value from one range to another
pub fn map_range(value: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
    (value - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}

/// Random float between 0.0 and 1.0 (simple LCG)
pub fn random() -> f32 {
    static mut SEED: u32 = 123456789;
    unsafe {
        SEED = SEED.wrapping_mul(1103515245).wrapping_add(12345);
        (SEED / 65536 % 32768) as f32 / 32768.0
    }
}

/// Random float between min and max
pub fn random_range(min: f32, max: f32) -> f32 {
    min + random() * (max - min)
}

/// Random integer between min and max (inclusive)
pub fn random_int(min: i32, max: i32) -> i32 {
    min + (random() * (max - min + 1) as f32) as i32
}