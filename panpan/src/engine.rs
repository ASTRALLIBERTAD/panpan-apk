use crate::renderer::Renderer2D;
use glow::Context;
use std::sync::Mutex;


pub enum InputEvent { TouchDown { id:i32, x:f32, y:f32 }, TouchMove { id:i32, x:f32, y:f32 }, TouchUp { id:i32 } }

static RENDERER: Mutex<Option<Box<dyn Renderer2D>>> = Mutex::new(None);

// Global screen size for renderer
static SCREEN_SIZE: Mutex<(i32, i32)> = Mutex::new((0, 0));

pub fn get_screen_size() -> (i32, i32) {
    *SCREEN_SIZE.lock().unwrap()
}

pub fn set_screen_size(width: i32, height: i32) {
    *SCREEN_SIZE.lock().unwrap() = (width, height);
}

pub fn init(_gl: Option<Context>) {
    #[cfg(feature="android")]
    {
        // Android: use eglGetProcAddress to load GL functions
        // EGL is already linked via .cargo/config.toml
        unsafe extern "C" {
            fn eglGetProcAddress(procname: *const std::os::raw::c_char) -> *const std::os::raw::c_void;
        }
        
        let gl = unsafe {
            glow::Context::from_loader_function(|name| {
                let name_cstr = std::ffi::CString::new(name).unwrap();
                eglGetProcAddress(name_cstr.as_ptr())
            })
        };
        let renderer = crate::renderer::opengles::GLESRenderer::new(gl);
        *RENDERER.lock().unwrap() = Some(Box::new(renderer));
    }

    #[cfg(feature="desktop")]
    if let Some(gl) = _gl {
        let renderer = crate::renderer::opengles::GLESRenderer::new(gl);
        *RENDERER.lock().unwrap() = Some(Box::new(renderer));
    }
}

pub fn resize(w: i32, h: i32) {
    set_screen_size(w, h);
    if let Some(_) = RENDERER.lock().unwrap().as_ref() { /* set viewport if needed */ }
}

pub fn render() {
    if let Some(r) = RENDERER.lock().unwrap().as_ref() {
        r.clear(0.1, 0.2, 0.3, 1.0);
    }
}

pub fn clear_screen(r: f32, g: f32, b: f32, a: f32) {
    if let Some(renderer) = RENDERER.lock().unwrap().as_ref() {
        renderer.clear(r, g, b, a);
    }
}

pub fn draw_text(text: &str, x: f32, y: f32, scale: f32, color: crate::util::Color) {
    if let Some(r) = RENDERER.lock().unwrap().as_ref() {
        r.draw_text(text, x, y, scale, color);
    }
}

pub fn draw_rect(x: f32, y: f32, w: f32, h: f32, color: crate::util::Color) {
    if let Some(r) = RENDERER.lock().unwrap().as_ref() {
        r.draw_rect(x, y, w, h, color);
    }
}


type TouchDownFn = Box<dyn Fn(i32, f32, f32) + Send + Sync>;
type TouchMoveFn = Box<dyn Fn(i32, f32, f32) + Send + Sync>;
type TouchUpFn = Box<dyn Fn(i32) + Send + Sync>;

static TOUCH_DOWN_HANDLER: Mutex<Option<TouchDownFn>> = Mutex::new(None);
static TOUCH_MOVE_HANDLER: Mutex<Option<TouchMoveFn>> = Mutex::new(None);
static TOUCH_UP_HANDLER: Mutex<Option<TouchUpFn>> = Mutex::new(None);

pub fn set_touch_handlers<D, M, U>(down: D, move_fn: M, up: U)
where
    D: Fn(i32, f32, f32) + Send + Sync + 'static,
    M: Fn(i32, f32, f32) + Send + Sync + 'static,
    U: Fn(i32) + Send + Sync + 'static,
{
    *TOUCH_DOWN_HANDLER.lock().unwrap() = Some(Box::new(down));
    *TOUCH_MOVE_HANDLER.lock().unwrap() = Some(Box::new(move_fn));
    *TOUCH_UP_HANDLER.lock().unwrap() = Some(Box::new(up));
}

pub fn touch_down(id:i32, x:f32, y:f32) {
    if let Some(handler) = TOUCH_DOWN_HANDLER.lock().unwrap().as_ref() {
        handler(id, x, y);
    }
}

pub fn touch_move(id:i32, x:f32, y:f32) {
    if let Some(handler) = TOUCH_MOVE_HANDLER.lock().unwrap().as_ref() {
        handler(id, x, y);
    }
}

pub fn touch_up(id:i32) {
    if let Some(handler) = TOUCH_UP_HANDLER.lock().unwrap().as_ref() {
        handler(id);
    }
}
