use panpan::*;

pub fn init() {
    // Initialization
}

pub fn resize(width: i32, height: i32) {
    // Handle resize
}

pub fn render() {
    // BRIGHT RED - if you see this, rendering works!
    unsafe {
        gl::ClearColor(1.0, 0.0, 0.0, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }
}