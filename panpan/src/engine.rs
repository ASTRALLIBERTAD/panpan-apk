use crate::renderer::Renderer2D;
use crate::util::Color;
use glow::Context;
use std::sync::Mutex;


pub enum InputEvent { TouchDown { id:i32, x:f32, y:f32 }, TouchMove { id:i32, x:f32, y:f32 }, TouchUp { id:i32 } }

static RENDERER: Mutex<Option<Box<dyn Renderer2D>>> = Mutex::new(None);
pub fn init(gl: Option<Context>) {
    #[cfg(feature="android")]
    {
        // Android uses GLES renderer
        let renderer = crate::renderer::opengles::GLESRenderer::new(glow::Context::from_loader_function(|s| s.as_ptr() as *const _));
        *RENDERER.lock().unwrap() = Some(Box::new(renderer));
    }

    #[cfg(feature="desktop")]
    if let Some(gl) = gl {
        let renderer = crate::renderer::opengles::GLESRenderer::new(gl);
        *RENDERER.lock().unwrap() = Some(Box::new(renderer));
    }
}

pub fn resize(w: i32, h: i32) {
    if let Some(_) = RENDERER.lock().unwrap().as_ref() { /* set viewport if needed */ }
}

pub fn render() {
    if let Some(r) = RENDERER.lock().unwrap().as_ref() {
        r.clear(0.1, 0.2, 0.3, 1.0);
    }
}


pub fn touch_down(_id:i32, _x:f32, _y:f32) {}
pub fn touch_move(_id:i32, _x:f32, _y:f32) {}
pub fn touch_up(_id:i32) {}
