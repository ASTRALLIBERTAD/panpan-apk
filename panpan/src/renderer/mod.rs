pub mod opengles;
// pub mod sdf_text;

pub trait Renderer2D: Send + Sync {
    fn clear(&self, r:f32, g:f32, b:f32, a:f32);
    fn draw_text(&self, text:&str, x:f32, y:f32, scale:f32, color:crate::util::Color);
    fn draw_rect(&self, x:f32, y:f32, w:f32, h:f32, color:crate::util::Color);
}
