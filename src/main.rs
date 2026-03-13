mod vec2;
mod ball;
mod light;
mod renderer;

use ball::Ball;
use light::Light;
use renderer::{draw_rect_outline, draw_circle_lit, draw_background_lit, WIDTH, HEIGHT};


struct SimApp {
    balls: Vec<Ball>, 
    light: Light,
    gravity: f32, 
    friction: f32, 
    restitution: f32, 
    ball_count: usize,
}

impl SimApp {
    fn new(count: usize) -> Self {
        SimApp { 
            balls: (0..count).map(|_| Ball::new()).collect(), 
            light: Light::new(), 
            gravity: 0.5,
            friction: 0.85,
            restitution: 0.6,
            ball_count: count,
        }
    }

    fn reset(&mut self) {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        self.balls = (0..self.ball_count).map(|_| Ball::new()).collect();
        self.light = Light::new();
        self.gravity = rng.gen_range(0.0..=1.0);
        self.friction = rng.gen_range(0.5..=1.0);
        self.restitution = rng.gen_range(0.3..=1.0);
        self.light.radius = rng.gen_range(100.0..=500.0);
        self.light.intensity = rng.gen_range(0.5..=2.5);
    }
}

impl eframe::App for SimApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        // Side panel UI
        egui::SidePanel::right("controls")
            .resizable(false)
            .exact_width(200.0)
            .show(ctx, |ui| {
                ui.heading("Controls");
                ui.separator();

                ui.label("Gravity");
                ui.add(egui::Slider::new(&mut self.gravity, 0.0..=2.0).step_by(0.01));

                ui.label("Friction");
                ui.add(egui::Slider::new(&mut self.friction, 0.0..=1.0).step_by(0.01));

                ui.label("Restitution");
                ui.add(egui::Slider::new(&mut self.restitution, 0.0..=1.0).step_by(0.01));

                ui.separator();

                ui.label("Light Radius");
                ui.add(egui::Slider::new(&mut self.light.radius, 50.0..=600.0).step_by(1.0));

                ui.label("Light Intensity");
                ui.add(egui::Slider::new(&mut self.light.intensity, 0.0..=3.0).step_by(0.01));

                ui.separator();

                ui.label("Ball Count");
                ui.add(egui::Slider::new(&mut self.ball_count, 1..=10));

                if ui.button("Respawn").clicked() {
                    self.balls = (0..self.ball_count).map(|_| Ball::new()).collect();
                }

                ui.separator();

                if ui.button("Shuffle").clicked() {
                    self.reset();
                }
            });        

        // Main simulation panel
        egui::CentralPanel::default().show(ctx, |ui| {
            // Handle light dragging
            if let Some(pos) = ctx.pointer_latest_pos() {
                if ctx.input(|i| i.pointer.primary_down()) {
                    self.light.pos.x = pos.x;
                    self.light.pos.y = pos.y;
                }
            }

            // update physics
            for ball in self.balls.iter_mut() {
                ball.update(self.gravity, self.friction, self.restitution);
            }

            // ball to ball collisions
            for i in 0..self.balls.len() {
                for j in (i + 1)..self.balls.len() {
                    // split_at_mut gives us two separate mutable slices
                    // so the borrow checker is happy
                    let (left, right) = self.balls.split_at_mut(j);
                    let ball_a = &mut left[i];
                    let ball_b = &mut right[0];

                    let dx = ball_b.pos.x - ball_a.pos.x;
                    let dy = ball_b.pos.y - ball_a.pos.y;
                    let distance = (dx * dx + dy * dy).sqrt();
                    let min_dist = (ball_a.radius + ball_b.radius) as f32;

                    if distance < min_dist && distance > 0.0 {
                        // normalize the collision vector
                        let nx = dx / distance;
                        let ny = dy / distance;

                        // separate the balls so they don't overlap
                        let overlap = min_dist - distance;
                        ball_a.pos.x -= nx * overlap * 0.5;
                        ball_a.pos.y -= ny * overlap * 0.5;
                        ball_b.pos.x += nx * overlap * 0.5;
                        ball_b.pos.y += nx * overlap * 0.5;

                        // relative velocity along collision normal
                        let relative_vel = (ball_b.vel.x - ball_a.vel.x) * nx
                                            + (ball_b.vel.y - ball_a.vel.y) * ny;

                        // only resolve if balls are moving toward each other
                        if relative_vel < 0.0 {
                            let restitution = 0.5; // bounciness
                            let impulse = relative_vel * restitution * 0.7;

                            ball_a.vel.x -= impulse * nx;
                            ball_a.vel.y -= impulse * ny;
                            ball_b.vel.x += impulse * nx;
                            ball_b.vel.y += impulse * ny;
                        }
                    }
                }
            }

            // Draw into pixel buffer
            let mut buffer: Vec<u32> = vec![0x00_0A_0A_1A; WIDTH * HEIGHT];
            draw_background_lit(&mut buffer, &self.light);
            for ball in self.balls.iter() {
                draw_circle_lit(&mut buffer, ball.pos.x, ball.pos.y, ball.radius, &self.light);
            }
            draw_rect_outline(&mut buffer, 
                renderer::BOX_X, renderer::BOX_Y, 
                renderer::BOX_W, renderer::BOX_H, 
                0x00_FF_FF_FF);
            
            // convert buffer to egui image
            let pixels: Vec<egui::Color32> = buffer.iter().map(|&p| {
                let r = ((p >> 16) & 0xFF) as u8;
                let g = ((p >> 8) & 0xFF) as u8;
                let b = (p & 0xFF) as u8;
                egui::Color32::from_rgb(r, g, b)
            }).collect();

            let image = egui::ColorImage {
                size: [WIDTH, HEIGHT],
                pixels,
            };

            let texture = ctx.load_texture("sim", image, egui::TextureOptions::default());
            ui.image(&texture);
        });

        ctx.request_repaint(); // keep animating
    }
        
}


fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([(WIDTH + 200) as f32, HEIGHT as f32]),
        ..Default::default()
    };

    eframe::run_native(
        "Bouncing Balls",
        options,
        Box::new(move |_cc| Ok(Box::new(SimApp::new(3))))
    )
}