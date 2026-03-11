use minifb::{Window, WindowOptions};
use rand::Rng;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

// box boundaries
const BOX_X: usize = 50;
const BOX_Y: usize = 50;
const BOX_W: usize = 700;
const BOX_H: usize = 500;

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

fn draw_circle(buffer: &mut Vec<u32>, cx: f32, cy: f32, radius: usize, color: u32) {
    let r = radius as i32;
    let cx = cx as i32;
    let cy = cy as i32;

    for y in -r..=r {
        for x in -r..r {
            if x * x + y * y <= r * r {
                let px = cx + x;
                let py = cy + y;
                if px >= 0 && px < WIDTH as i32 && py >= 0 && py < HEIGHT as i32 {
                    buffer[py as usize * WIDTH + px as usize] = color;
                }
            }
        }
    }
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
    
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new("Bouncing Balls", WIDTH, HEIGHT, WindowOptions::default())
        .expect("Failed to create window");
    window.set_target_fps(60); //~60 fps

    while window.is_open() {
        // Fill the buffer with a dark background color
        buffer.fill(0x00_1A_1A_2E);

        for ball in balls.iter_mut() {
            ball.update();
            draw_circle(&mut buffer, ball.pos.x, ball.pos.y, ball.radius, 0x00_FF_FF_FF);
        }

        draw_rect_outline(&mut buffer, BOX_X, BOX_Y, BOX_W, BOX_H, 0x00_FF_FF_FF);

        window.update_with_buffer(&buffer, WIDTH, HEIGHT)
            .expect("Failed to update window");
    }
}