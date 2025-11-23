#![allow(non_snake_case)]


use jni::JNIEnv;
use jni::objects::JClass;


#[no_mangle]
pub extern "system" fn Java_com_lucidum_panpan_MainActivity_nativeInit(env: JNIEnv, _class: JClass) {
// ensure engine init is called after GL context creation
// call panpan::init()
panpan::init();
}


#[no_mangle]
pub extern "system" fn Java_com_lucidum_panpan_MainActivity_nativeResize(_env: JNIEnv, _class: JClass, w: i32, h: i32) {
panpan::resize(w, h);
}


#[no_mangle]
pub extern "system" fn Java_com_lucidum_panpan_MainActivity_nativeRender(_env: JNIEnv, _class: JClass) {
panpan::render();
}


#[no_mangle]
pub extern "system" fn Java_com_lucidum_panpan_MainActivity_nativeTouchDown(_env: JNIEnv, _class: JClass, id: i32, x: f32, y: f32) {
panpan::touch_down(id, x, y);
}


#[no_mangle]
pub extern "system" fn Java_com_lucidum_panpan_MainActivity_nativeTouchMove(_env: JNIEnv, _class: JClass, id: i32, x: f32, y: f32) {
panpan::touch_move(id, x, y);
}


#[no_mangle]
pub extern "system" fn Java_com_lucidum_panpan_MainActivity_nativeTouchUp(_env: JNIEnv, _class: JClass, id: i32) {
panpan::touch_up(id);
}