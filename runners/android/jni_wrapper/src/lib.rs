// Android JNI wrapper for demo_game
use jni::objects::JClass;
use jni::sys::{jfloat, jint};
use jni::JNIEnv;
use std::sync::Mutex;

static GAME: Mutex<Option<demo_game::DemoGame>> = Mutex::new(None);

#[no_mangle]
pub extern "C" fn Java_com_lucidum_panpan_MainActivity_nativeInit(_env: JNIEnv, _class: JClass) {
    println!("[JNI] nativeInit called");

    // Initialize OpenGL context
    unsafe extern "C" {
        fn eglGetProcAddress(procname: *const std::os::raw::c_char) -> *const std::os::raw::c_void;
    }

    let gl = unsafe {
        glow::Context::from_loader_function(|name| {
            let c_str = std::ffi::CString::new(name).unwrap();
            eglGetProcAddress(c_str.as_ptr())
        })
    };

    // Initialize panpan graphics
    panpan::__internal_init_graphics(gl);
    println!("[JNI] PanPan graphics initialized");

    // Create game instance
    use panpan::Game;
    let game = demo_game::DemoGame::new();
    *GAME.lock().unwrap() = Some(game);
    println!("[JNI] Game created");
}

#[no_mangle]
pub extern "C" fn Java_com_lucidum_panpan_MainActivity_nativeResize(
    _env: JNIEnv,
    _class: JClass,
    width: jint,
    height: jint,
) {
    println!("[JNI] nativeResize: {}x{}", width, height);
    panpan::__internal_resize(width, height);
}

#[no_mangle]
pub extern "C" fn Java_com_lucidum_panpan_MainActivity_nativeUpdateTime(
    _env: JNIEnv,
    _class: JClass,
    dt: jfloat,
) {
    use panpan::Game;
    if let Some(game) = GAME.lock().unwrap().as_mut() {
        game.update(dt);
    }
}

#[no_mangle]
pub extern "C" fn Java_com_lucidum_panpan_MainActivity_nativeRender(_env: JNIEnv, _class: JClass) {
    use panpan::Game;
    if let Some(game) = GAME.lock().unwrap().as_ref() {
        game.render();
    }
}

#[no_mangle]
pub extern "C" fn Java_com_lucidum_panpan_MainActivity_nativeTouchDown(
    _env: JNIEnv,
    _class: JClass,
    id: jint,
    x: jfloat,
    y: jfloat,
) {
    use panpan::Game;
    if let Some(game) = GAME.lock().unwrap().as_mut() {
        game.on_touch_down(id, x, y);
    }
}

#[no_mangle]
pub extern "C" fn Java_com_lucidum_panpan_MainActivity_nativeTouchMove(
    _env: JNIEnv,
    _class: JClass,
    id: jint,
    x: jfloat,
    y: jfloat,
) {
    use panpan::Game;
    if let Some(game) = GAME.lock().unwrap().as_mut() {
        game.on_touch_move(id, x, y);
    }
}

#[no_mangle]
pub extern "C" fn Java_com_lucidum_panpan_MainActivity_nativeTouchUp(
    _env: JNIEnv,
    _class: JClass,
    id: jint,
) {
    use panpan::Game;
    if let Some(game) = GAME.lock().unwrap().as_mut() {
        game.on_touch_up(id);
    }
}
