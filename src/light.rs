use crate::vec2::Vec2;

pub struct Light {
    pub pos: Vec2,
    pub radius: f32, // how far the light reaches
    pub intensity: f32,
}

impl Light {
    pub fn new() -> Self {
        Light { 
            pos: Vec2 { x: 400.0, y: 300.0 }, 
            radius: 250.0, 
            intensity: 1.0,
        }
    }
}