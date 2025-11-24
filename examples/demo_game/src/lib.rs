// examples/demo_game/src/lib.rs
use panpan::*;

pub struct DemoGame {
    rectangles: Vec<AnimatedRect>,
    time: f32,
}

struct AnimatedRect {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    color: Color,
}

impl Game for DemoGame {
    fn new() -> Self {
        println!("DemoGame: Initializing...");
        
        let mut rectangles = Vec::new();
        for i in 0..5 {
            let angle = (i as f32 / 5.0) * std::f32::consts::PI * 2.0;
            rectangles.push(AnimatedRect {
                x: 400.0,
                y: 300.0,
                vx: angle.cos() * 100.0,
                vy: angle.sin() * 100.0,
                color: Self::hsv_to_color(i as f32 * 72.0, 0.8, 0.9),
            });
        }
        
        Self {
            rectangles,
            time: 0.0,
        }
    }
    
    fn update(&mut self, dt: f32) {
        self.time += dt;
        
        // Update rectangle positions
        for rect in &mut self.rectangles {
            rect.x += rect.vx * dt;
            rect.y += rect.vy * dt;
            
            // Bounce off walls
            if rect.x < 0.0 || rect.x > 750.0 {
                rect.vx = -rect.vx;
            }
            if rect.y < 0.0 || rect.y > 550.0 {
                rect.vy = -rect.vy;
            }
        }
    }
    
    fn render(&self) {
        // Clear screen
        clear_screen(Color::new(0.1, 0.12, 0.15, 1.0));
        
        // Draw animated rectangles
        for rect in &self.rectangles {
            draw_rect(rect.x, rect.y, 50.0, 50.0, rect.color);
        }
        
        // Draw FPS indicator
        let fps_text = format!("FPS: {:.0}", 1.0 / 0.016);
        draw_text(&fps_text, 20.0, 20.0, 20.0, Color::GREEN);
        
        // Draw title
        draw_text("PanPan Demo Game", 300.0, 20.0, 24.0, Color::WHITE);
    }
    
    fn on_touch_down(&mut self, id: i32, x: f32, y: f32) {
        println!("Touch down: id={}, pos=({:.1}, {:.1})", id, x, y);
        
        // Add a new rectangle at touch position
        self.rectangles.push(AnimatedRect {
            x,
            y,
            vx: (rand() * 200.0) - 100.0,
            vy: (rand() * 200.0) - 100.0,
            color: Self::hsv_to_color(rand() * 360.0, 0.8, 0.9),
        });
    }
    
    fn on_touch_up(&mut self, id: i32) {
        println!("Touch up: id={}", id);
    }
}

impl DemoGame {
    fn hsv_to_color(h: f32, s: f32, v: f32) -> Color {
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
}

// Simple pseudo-random number generator
fn rand() -> f32 {
    use std::cell::Cell;
    thread_local! {
        static SEED: Cell<u32> = Cell::new(12345);
    }
    
    SEED.with(|seed| {
        let mut s = seed.get();
        s = s.wrapping_mul(1103515245).wrapping_add(12345);
        seed.set(s);
        ((s / 65536) % 32768) as f32 / 32768.0
    })
}