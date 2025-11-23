use panpan::Color;
use std::sync::Mutex;
use std::time::Instant;

struct AppState {
    width: f32,
    height: f32,
    fps: f32,
    last_fps_check: Instant,
    frames: u64,
    animation_time: f32,
}

static APP: Mutex<Option<AppState>> = Mutex::new(None);

pub fn init() {
    println!("[example_crate] init");
    *APP.lock().unwrap() = Some(AppState {
        width: 800.0,
        height: 600.0,
        fps: 0.0,
        frames: 0,
        last_fps_check: Instant::now(),
        animation_time: 0.0,
    });
    println!("[example_crate] init complete");
}

pub fn resize(w: i32, h: i32) {
    println!("[example_crate] resize: {}x{}", w, h);
    panpan::resize(w, h);
    if let Some(app) = APP.lock().unwrap().as_mut() {
        app.width = w as f32;
        app.height = h as f32;
    }
}

pub fn render() {
    let mut app_guard = APP.lock().unwrap();
    let app = match app_guard.as_mut() {
        Some(a) => a,
        None => {
            println!("[example_crate] ERROR: AppState not initialized!");
            return;
        }
    };

    // Clear background
    panpan::clear_screen(0.1, 0.12, 0.15, 1.0);

    // Update FPS
    app.frames += 1;
    let now = Instant::now();
    let dt = now.duration_since(app.last_fps_check).as_secs_f32();
    if dt >= 1.0 {
        app.fps = app.frames as f32 / dt;
        app.frames = 0;
        app.last_fps_check = now;
        println!("[example_crate] FPS: {:.1}", app.fps);
    }

    // Update animation
    app.animation_time += 0.016;

    // Draw animated rectangles
    let num_rects = 5;
    for i in 0..num_rects {
        let offset = (i as f32 * 2.0 * std::f32::consts::PI) / num_rects as f32;
        let x = app.width * 0.5 + (app.animation_time * 2.0 + offset).cos() * app.width * 0.25;
        let y = app.height * 0.5 + (app.animation_time * 3.0 + offset).sin() * app.height * 0.25;
        
        let hue = (i as f32 * 72.0) % 360.0;
        let color = hsv_to_rgb(hue, 0.8, 0.9);
        
        panpan::draw_rect(x - 25.0, y - 25.0, 50.0, 50.0, color);
    }

    // Draw FPS indicator
    let fps_x = 20.0;
    let fps_y = 20.0;
    for i in 0..10 {
        panpan::draw_rect(fps_x + i as f32 * 12.0, fps_y, 8.0, 20.0, Color::new(0.0, 1.0, 0.0, 1.0));
    }
}

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> Color {
    let c = v * s;
    let h_prime = h / 60.0;
    let x = c * (1.0 - ((h_prime % 2.0) - 1.0).abs());
    
    let (r1, g1, b1) = match h_prime as i32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    
    let m = v - c;
    Color::new(r1 + m, g1 + m, b1 + m, 1.0)
}
