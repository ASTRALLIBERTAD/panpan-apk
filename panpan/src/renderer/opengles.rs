use super::Renderer2D;
use crate::util::Color;
use glow::HasContext;

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

    fn draw_rect(&self, _x: f32, _y: f32, _w: f32, _h: f32, _color: Color) {
        // placeholder: draw using simple shader/quads
    }
}
