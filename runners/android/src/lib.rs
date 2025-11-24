// runners/android/rust/src/lib.rs
// Android JNI bridge - minimal platform code

use jni::objects::JClass;
use jni::sys::jfloat;
use jni::JNIEnv;
use std::sync::Mutex;
use panpan::Game;

// The game instance will be stored here
static GAME: Mutex<Option<Box<dyn GameWrapper>>> = Mutex::new(None);

// Trait to erase the game type
trait GameWrapper: Send {
    fn update(&mut self, dt: f32);
    fn render(&self);
    fn touch_down(&mut self, id: i32, x: f32, y: f32);
    fn touch_move(&mut self, id: i32, x: f32, y: f32);
    fn touch_up(&mut self, id: i32);
}

impl<G: Game + Send + 'static> GameWrapper for G {
    fn update(&mut self, dt: f32) {
        Game::update(self, dt);
    }
    
    fn render(&self) {
        Game::render(self);
    }
    
    fn touch_down(&mut self, id: i32, x: f32, y: f32) {
        Game::on_touch_down(self, id, x, y);
    }
    
    fn touch_move(&mut self, id: i32, x: f32, y: f32) {
        Game::on_touch_move(self, id, x, y);
    }
    
    fn touch_up(&mut self, id: i32) {
        Game::on_touch_up(self, id);
    }
}

// This function will be called by the build tool to register the game type
pub fn register_game<G: Game + Send + 'static>() {
    *GAME.lock().unwrap() = Some(Box::new(G::new()));
}

#[no_mangle]
pub extern "C" fn Java_com_panpan_MainActivity_nativeInit(_env: JNIEnv, _class: JClass) {
    println!("PanPan Android: Initializing...");
    
    // Initialize OpenGL function loader
    extern "C" {
        fn eglGetProcAddress(procname: *const std::os::raw::c_char) -> *const std::os::raw::c_void;
    }
    
    panpan::__internal_init_graphics(|name| {
        let c_str = std::ffi::CString::new(name).unwrap();
        unsafe { eglGetProcAddress(c_str.as_ptr()) }
    });
    
    println!("PanPan Android: Graphics initialized");
    
    // The game will be created by the generated code
}

#[no_mangle]
pub extern "C" fn Java_com_panpan_MainActivity_nativeResize(
    _env: JNIEnv,
    _class: JClass,
    width: i32,
    height: i32,
) {
    println!("PanPan Android: Resize {}x{}", width, height);
    panpan::__internal_resize(width, height);
}

#[no_mangle]
pub extern "C" fn Java_com_panpan_MainActivity_nativeUpdate(
    _env: JNIEnv,
    _class: JClass,
    dt: jfloat,
) {
    if let Some(game) = GAME.lock().unwrap().as_mut() {
        game.update(dt);
    }
}

#[no_mangle]
pub extern "C" fn Java_com_panpan_MainActivity_nativeRender(_env: JNIEnv, _class: JClass) {
    if let Some(game) = GAME.lock().unwrap().as_ref() {
        game.render();
    }
}

#[no_mangle]
pub extern "C" fn Java_com_panpan_MainActivity_nativeTouchDown(
    _env: JNIEnv,
    _class: JClass,
    id: i32,
    x: jfloat,
    y: jfloat,
) {
    if let Some(game) = GAME.lock().unwrap().as_mut() {
        game.touch_down(id, x, y);
    }
}

#[no_mangle]
pub extern "C" fn Java_com_panpan_MainActivity_nativeTouchMove(
    _env: JNIEnv,
    _class: JClass,
    id: i32,
    x: jfloat,
    y: jfloat,
) {
    if let Some(game) = GAME.lock().unwrap().as_mut() {
        game.touch_move(id, x, y);
    }
}

#[no_mangle]
pub extern "C" fn Java_com_panpan_MainActivity_nativeTouchUp(
    _env: JNIEnv,
    _class: JClass,
    id: i32,
) {
    if let Some(game) = GAME.lock().unwrap().as_mut() {
        game.touch_up(id);
    }
}