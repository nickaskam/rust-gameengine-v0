use crate::vec2::Vec2;
use crate::renderer::{BOX_X, BOX_Y, BOX_W, BOX_H};

pub const BALL_RADIUS: usize = 15;

pub struct Ball {
    pub pos: Vec2, 
    pub vel: Vec2,
    pub radius: usize,
}

impl Ball {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        use rand::Rng;

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

    pub fn update(&mut self, gravity: f32, friction: f32, restitution: f32) {
        // air resistance - applied every frame
        self.vel.x *= 0.99;
        self.vel.y *= 0.99;

        // Apply gravity 
        self.vel.y += gravity;

        // move ball 
        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;

        let r = self.radius as f32;

        // Bounce off left/right walls
        if self.pos.x - r < BOX_X as f32 {
            self.pos.x = BOX_X as f32 + r;
            self.vel.x *= -restitution;
        }
        if self.pos.x + r > (BOX_X + BOX_W) as f32 {
            self.pos.x = (BOX_X + BOX_W) as f32 - r;
            self.vel.x *= -restitution;
        }
        
        // Bounce off top/bottom walls
        if self.pos.y - r < BOX_Y as f32 {
            self.pos.y = BOX_Y as f32 + r;
            self.vel.y *= -restitution;
        }
        if self.pos.y + r > (BOX_Y + BOX_H) as f32 {
            self.pos.y = (BOX_Y + BOX_H) as f32 - r;
            self.vel.y *= -restitution; // slight energy loss on floor bounce

            // apply friction when touching the floor
            self.vel.x *= friction;
        }
    }
}