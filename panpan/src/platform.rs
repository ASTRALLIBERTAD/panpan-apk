pub trait Platform: Send + Sync {
fn set_screen_size(&self, w: i32, h: i32);
fn swap_buffers(&self);
fn time_seconds(&self) -> f32;
fn poll_events(&self) -> Vec<crate::engine::InputEvent>;
}


static mut PLATFORM_IMPL: Option<&'static dyn Platform> = None;


pub fn set_platform(p: &'static dyn Platform) {
unsafe { PLATFORM_IMPL = Some(p); }
}


pub fn with_platform<F: FnOnce(&'static dyn Platform)>(f: F) {
unsafe {
if let Some(p) = PLATFORM_IMPL {
f(p);
}
}
}