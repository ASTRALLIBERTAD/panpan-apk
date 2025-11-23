use super::Renderer2D;
use crate::util::Color;
use glow::HasContext;
use crate::engine::get_screen_size;

pub struct GLESRenderer {
    pub gl: glow::Context,
}

impl GLESRenderer {
    pub fn new(gl: glow::Context) -> Self {
        Self { gl }
    }
}

impl Renderer2D for GLESRenderer {
    fn clear(&self, r: f32, g: f32, b: f32, a: f32) {
        unsafe {
            self.gl.clear_color(r, g, b, a);
            self.gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
        }
    }

    fn draw_text(&self, _text: &str, _x: f32, _y: f32, _scale: f32, _color: Color) {
        // placeholder: call into bitmap font renderer
    }

    fn draw_rect(&self, x: f32, y: f32, w: f32, h: f32, color: Color) {
        // Use scissor test to draw a solid colored rectangle
        let (screen_w, screen_h) = get_screen_size();
        if screen_w == 0 || screen_h == 0 {
            return; // size not set yet
        }
        // Convert coordinates (origin bottom-left) to integer pixels
        let ix = x.max(0.0).min(screen_w as f32) as i32;
        let iy = y.max(0.0).min(screen_h as f32) as i32;
        let iwidth = w.max(0.0).min(screen_w as f32 - ix as f32) as i32;
        let iheight = h.max(0.0).min(screen_h as f32 - iy as f32) as i32;
        unsafe {
            self.gl.enable(glow::SCISSOR_TEST);
            self.gl.scissor(ix, iy, iwidth, iheight);
            self.gl.clear_color(color.r, color.g, color.b, color.a);
            self.gl.clear(glow::COLOR_BUFFER_BIT);
            self.gl.disable(glow::SCISSOR_TEST);
        }
    }
}
