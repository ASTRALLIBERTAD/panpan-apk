// Example Rust game library for PanPan

pub fn init() {
    // initialize engine state
    println!("panpan_example: init");
}

pub fn resize(width: i32, height: i32) {
    println!("panpan_example: resize {} {}", width, height);
}

pub fn render() {
    // render one frame (called from Android)
}
