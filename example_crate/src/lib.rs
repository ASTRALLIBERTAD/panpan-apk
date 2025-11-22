use panpan::*;

pub fn init() {
    println!("Game initialized!");
}

pub fn resize(width: i32, height: i32) {
    println!("Screen resized: {}x{}", width, height);
}

pub fn render() {
    // Clear screen to dark blue
    clear_rgb(0.1, 0.2, 0.3);
    
    // Draw some text
    draw_text("Hello World!", 100.0, 100.0, 3.0, Color::WHITE);
    draw_text("PanPan Engine", 100.0, 150.0, 2.0, Color::YELLOW);
    
    let screen = screen();
    let fps_text = format!("Screen: {}x{}", screen.width, screen.height);
    draw_text(&fps_text, 10.0, 10.0, 1.5, Color::GREEN);
    
    // You can also use raw OpenGL:
    // unsafe { gl::... }
}