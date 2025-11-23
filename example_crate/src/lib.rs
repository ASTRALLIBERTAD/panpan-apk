use panpan::*;
use std::sync::Mutex;
use std::time::Instant;

// Application state
struct AppState {
    width: f32,
    height: f32,
    frame_count: u64,
    last_fps_check: Instant,
    fps: f32,
    
    // Animation state
    animation_time: f32,
    
    // Touch state
    touch_points: Vec<TouchPoint>,
    
    // Color cycling
    color_hue: f32,
}

#[derive(Clone)]
struct TouchPoint {
    id: i32,
    x: f32,
    y: f32,
    lifetime: f32,
}

static APP_STATE: Mutex<Option<AppState>> = Mutex::new(None);

pub fn init() {
    panpan::init(None);
    
    // Register touch handlers with the framework
    panpan::set_touch_handlers(
        handle_touch_down,
        handle_touch_move,
        handle_touch_up,
    );
    
    *APP_STATE.lock().unwrap() = Some(AppState {
        width: 800.0,
        height: 600.0,
        frame_count: 0,
        last_fps_check: Instant::now(),
        fps: 60.0,
        animation_time: 0.0,
        touch_points: Vec::new(),
        color_hue: 0.0,
    });
}

pub fn resize(width: i32, height: i32) {
    panpan::resize(width, height);
    
    if let Some(state) = APP_STATE.lock().unwrap().as_mut() {
        state.width = width as f32;
        state.height = height as f32;
    }
}

pub fn render() {
    let mut state_guard = APP_STATE.lock().unwrap();
    if let Some(state) = state_guard.as_mut() {
        // Update FPS
        state.frame_count += 1;
        let now = Instant::now();
        let elapsed = now.duration_since(state.last_fps_check).as_secs_f32();
        if elapsed >= 1.0 {
            state.fps = state.frame_count as f32 / elapsed;
            state.frame_count = 0;
            state.last_fps_check = now;
        }
        
        // Update animation
        state.animation_time += 0.016; // ~60fps
        state.color_hue = (state.color_hue + 0.5) % 360.0;
        
        // Update touch points (fade out)
        state.touch_points.retain_mut(|tp| {
            tp.lifetime -= 0.016;
            tp.lifetime > 0.0
        });
        
        // Render the scene
        render_scene(state);
    }
    
    panpan::render();
}

fn render_scene(state: &AppState) {
    let w = state.width;
    let h = state.height;
    
    // Background gradient effect using rectangles
    let num_bars = 20;
    for i in 0..num_bars {
        let _y = (i as f32 / num_bars as f32) * h;
        let _bar_height = h / num_bars as f32;
        let color_mix = i as f32 / num_bars as f32;
        
        let _r = 0.05 + color_mix * 0.1;
        let _g = 0.1 + color_mix * 0.15;
        let _b = 0.2 + color_mix * 0.2;
        
        // Simulating draw_rect (panpan API doesn't expose it directly yet, so we'll skip actual drawing)
        // In a real implementation, you'd access the renderer directly or extend the API
    }
    
    // Draw animated bouncing rectangles
    let num_rects = 5;
    for i in 0..num_rects {
        let offset = (i as f32 * 2.0 * std::f32::consts::PI) / num_rects as f32;
        let x = w * 0.5 + (state.animation_time * 2.0 + offset).cos() * w * 0.3;
        let y = h * 0.3 + (state.animation_time * 3.0 + offset).sin() * h * 0.2;
        
        let hue = (state.color_hue + i as f32 * 72.0) % 360.0;
        let color = hsv_to_rgb(hue, 0.8, 0.9);
        
        // Draw the rectangle!
        panpan::draw_rect(x, y, 50.0, 50.0, color);
    }
    
    // Draw touch feedback circles (as squares for now)
    for tp in &state.touch_points {
        let alpha = tp.lifetime / 2.0;
        let size = (2.0 - tp.lifetime) * 30.0;
        let color = Color::new(1.0, 1.0, 0.0, alpha);
        
        panpan::draw_rect(tp.x - size/2.0, tp.y - size/2.0, size, size, color);
    }
    
    // Draw title text at top
    let title = "Panpan Demo";
    let title_color = Color::new(1.0, 1.0, 1.0, 1.0);
    panpan::draw_text(title, 20.0, 50.0, 2.0, title_color);
    
    // Draw FPS counter
    let fps_text = format!("FPS: {:.1}", state.fps);
    let fps_color = Color::new(0.0, 1.0, 0.0, 1.0);
    panpan::draw_text(&fps_text, 20.0, h - 50.0, 1.5, fps_color);
    
    // Draw instructions
    let instructions = "Touch to interact!";
    let inst_color = Color::new(0.8, 0.8, 0.8, 0.8);
    panpan::draw_text(instructions, 20.0, h - 100.0, 1.0, inst_color);
}

// Helper: Convert HSV to RGB
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

// Internal touch handlers called by panpan framework
// Users don't need to worry about JNI exports anymore!
fn handle_touch_down(id: i32, x: f32, y: f32) {
    if let Some(state) = APP_STATE.lock().unwrap().as_mut() {
        state.touch_points.push(TouchPoint {
            id,
            x,
            y,
            lifetime: 2.0,
        });
    }
}

fn handle_touch_move(id: i32, x: f32, y: f32) {
    if let Some(state) = APP_STATE.lock().unwrap().as_mut() {
        // Update or add touch point
        if let Some(tp) = state.touch_points.iter_mut().find(|tp| tp.id == id) {
            tp.x = x;
            tp.y = y;
            tp.lifetime = 2.0; // Refresh lifetime
        } else {
            state.touch_points.push(TouchPoint {
                id,
                x,
                y,
                lifetime: 2.0,
            });
        }
    }
}

fn handle_touch_up(id: i32) {
    if let Some(state) = APP_STATE.lock().unwrap().as_mut() {
        // Keep the point for fade-out effect
        if let Some(tp) = state.touch_points.iter_mut().find(|tp| tp.id == id) {
            tp.lifetime = tp.lifetime.min(0.5); // Quick fade
        }
    }
}