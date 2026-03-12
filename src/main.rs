use minifb::{Window, WindowOptions, MouseButton, MouseMode};
use rand::Rng;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

// box boundaries
const BOX_X: usize = 50;
const BOX_Y: usize = 50;
const BOX_W: usize = 695;
const BOX_H: usize = 490;

const GRAVITY: f32 = 0.5;
const BALL_RADIUS: usize = 15;

struct Vec2 {
    x: f32,
    y: f32,
}

struct Ball {
    pos: Vec2, 
    vel: Vec2,
    radius: usize,
}

struct Light {
    pos: Vec2,
    radius: f32, // how far the light reaches
}

impl Ball {
    fn new() -> Self {
        let mut rng = rand::thread_rng();

        // Random x position inside the box
        let x = rng.gen_range((BOX_X + BALL_RADIUS) as f32..(BOX_X + BOX_W - BALL_RADIUS) as f32);

        // Random y position inside the box
        let y = rng.gen_range((BOX_Y + BALL_RADIUS) as f32..(BOX_Y + BOX_H / 2) as f32);

        let vel_x = rng.gen_range(-3.0..3.0);
        let vel_y = rng.gen_range(-3.0..3.0);

        Ball {
            pos: Vec2 { x, y },
            vel: Vec2 { x: vel_x, y: vel_y },
            radius: BALL_RADIUS,
        }
    }

    fn update(&mut self) {
        // air resistance - applied every frame
        self.vel.x *= 0.99;
        self.vel.y *= 0.99;

        // Apply gravity 
        self.vel.y += GRAVITY;

        // move ball 
        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;

        let r = self.radius as f32;

        // Bounce off left/right walls
        if self.pos.x - r < BOX_X as f32 {
            self.pos.x = BOX_X as f32 + r;
            self.vel.x *= -0.85;
        }
        if self.pos.x + r > (BOX_X + BOX_W) as f32 {
            self.pos.x = (BOX_X + BOX_W) as f32 - r;
            self.vel.x *= -0.85;
        }
        
        // Bounce off top/bottom walls
        if self.pos.y - r < BOX_Y as f32 {
            self.pos.y = BOX_Y as f32 + r;
            self.vel.y *= -0.85;
        }
        if self.pos.y + r > (BOX_Y + BOX_H) as f32 {
            self.pos.y = (BOX_Y + BOX_H) as f32 - r;
            self.vel.y *= -0.85; // slight energy loss on floor bounce

            // apply friction when touching the floor
            self.vel.x *= 0.92;
        }
    }
}

impl Light {
    fn new() -> Self {
        Light { 
            pos: Vec2 { x: 400.0, y: 300.0 }, 
            radius: 250.0 
        }
    }
}

fn draw_rect_outline(buffer: &mut Vec<u32>, x: usize, y: usize, w: usize, h: usize, color: u32) {
    // top and bottom edges
    for i in x..x + w {
        buffer[y * WIDTH + i] = color;
        buffer[(y + h) * WIDTH + i] = color;
    }
    // left and right edges
    for i in y..y + h {
        buffer[i * WIDTH + x] = color;
        buffer[i * WIDTH + (x + w)] = color;
    }
}

fn draw_circle_lit(buffer: &mut Vec<u32>, cx: f32, cy: f32, radius: usize, light: &Light) {
    let r = radius as i32;
    let cx = cx as i32;
    let cy = cy as i32;

    for y in -r..=r {
        for x in -r..r {
            if x * x + y * y <= r * r {
                let px = cx + x;
                let py = cy + y;
                if px >= 0 && px < WIDTH as i32 && py >= 0 && py < HEIGHT as i32 {
                    let brightness = calc_brightness(px as f32, py as f32, light);
                    let color = apply_light(0xFF_6B_6B, brightness); // coral ball color
                    buffer[py as usize * WIDTH + px as usize] = color;
                }
            }
        }
    }
}

fn draw_background_lit(buffer: &mut Vec<u32>, light: &Light) {
    for y in BOX_Y..BOX_Y + BOX_H {
        for x in BOX_X..BOX_X + BOX_W {
            let brightness = calc_brightness(x as f32, y as f32, light);
            let color = apply_light(0x1A_1A_2E, brightness); // dark background color
            buffer[y * WIDTH + x] = color;
        }
    }
}

fn calc_brightness(px: f32, py: f32, light: &Light) -> f32 {
    let dx = px - light.pos.x;
    let dy = py - light.pos.y;
    let dist = (dx * dx + dy * dy).sqrt();

    let ambient = 0.15; // dim but visible in dark areas
    let brightness = (1.0 - (dist / light.radius)).max(0.0);
    let brightness = brightness * brightness; // quadratic falloff, feels more natural
    (ambient + brightness).min(1.0)
}

fn apply_light(base_color: u32, brightness: f32) -> u32 {
    let r = ((base_color >> 16) & 0xFF) as f32;
    let g = ((base_color >> 8) & 0xFF) as f32;
    let b = (base_color & 0xFF) as f32;

    // warm tint - boost red/green slightly, keep blue cool
    let warm_r = (r * brightness * 1.2).min(255.0) as u32;
    let warm_g = (g * brightness * 1.0).min(255.0) as u32;
    let warm_b = (b * brightness * 0.7).min(255.0) as u32;

    (warm_r << 16) | (warm_g << 8) | warm_b
}

fn get_ball_count() -> usize {
    println!("How many balls? (1-10):");

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim().parse::<usize>() {
            Ok(n) if n >= 1 && n <= 10 => return n,
            Ok(_) => println!("Please enter a number between 1 and 10:"),
            Err(_) => println!("That's not a valid number, try again:"),
        }
    }
}


fn main() {
    let count = get_ball_count();
    let mut balls: Vec<Ball> = (0..count).map(|_| Ball::new()).collect();
    let mut light = Light::new();
    
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new("Bouncing Balls", WIDTH, HEIGHT, WindowOptions::default())
        .expect("Failed to create window");

    window.set_target_fps(60); //~60 fps

    while window.is_open() {
        if window.get_mouse_down(MouseButton::Left) {
            if let Some((mx, my)) = window.get_mouse_pos(MouseMode::Clamp) {
                light.pos.x = mx;
                light.pos.y = my;
            }
        }

        // Fill the buffer with a dark background color
        buffer.fill(0x00_0A_0A_1A);

        // Draw lit background inside box
        draw_background_lit(&mut buffer, &light);

        for ball in balls.iter_mut() {
            ball.update();
        }

        // Handle ball to ball collisions
        for i in 0..balls.len() {
            for j in (i + 1)..balls.len() {
                // split_at_mut gives us two separate mutable slices
                // so the borrow checker is happy
                let (left, right) = balls.split_at_mut(j);
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

        for ball in balls.iter_mut() {
            draw_circle_lit(&mut buffer, ball.pos.x, ball.pos.y, ball.radius, &light);
        }

        draw_rect_outline(&mut buffer, BOX_X, BOX_Y, BOX_W, BOX_H, 0x00_FF_FF_FF);

        window.update_with_buffer(&buffer, WIDTH, HEIGHT)
            .expect("Failed to update window");
    }
}