use panpan::*;

static mut PLAYER_X: f32 = 400.0;
static mut PLAYER_Y: f32 = 300.0;

pub fn init() {
    let screen = screen();
    unsafe {
        PLAYER_X = screen.width as f32 / 2.0;
        PLAYER_Y = screen.height as f32 / 2.0;
    }
}

pub fn resize(width: i32, height: i32) {
    unsafe {
        PLAYER_X = width as f32 / 2.0;
        PLAYER_Y = height as f32 / 2.0;
    }
}

pub fn render() {
    clear_color(Color::rgb(0.1, 0.1, 0.2));
    
    // Move player with touch
    if let Some(touch) = get_touch(0) {
        let speed = 300.0 * delta_time();
        let dx = touch.x - unsafe { PLAYER_X };
        let dy = touch.y - unsafe { PLAYER_Y };
        let dist = (dx*dx + dy*dy).sqrt();
        
        if dist > 5.0 {
            unsafe {
                PLAYER_X += (dx / dist) * speed;
                PLAYER_Y += (dy / dist) * speed;
            }
        }
    }
    
    // Draw player
    unsafe {
        draw_text("@", PLAYER_X - 10.0, PLAYER_Y - 10.0, 3.0, Color::CYAN);
    }
    
    // Draw HUD
    let fps = (1.0 / delta_time()) as i32;
    draw_text(&format!("FPS: {}", fps), 10.0, 10.0, 2.0, Color::GREEN);
    draw_text("Touch to move!", 10.0, 40.0, 1.5, Color::WHITE);
}