use crate::light::Light;

pub const WIDTH: usize = 800;
pub const HEIGHT: usize = 600;

pub const BOX_X: usize = 50;
pub const BOX_Y: usize = 50;
pub const BOX_W: usize = 695;
pub const BOX_H: usize = 490;

pub fn draw_rect_outline(buffer: &mut Vec<u32>, x: usize, y: usize, w: usize, h: usize, color: u32) {
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

pub fn draw_circle_lit(buffer: &mut Vec<u32>, cx: f32, cy: f32, radius: usize, light: &Light) {
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

pub fn draw_background_lit(buffer: &mut Vec<u32>, light: &Light) {
    for y in BOX_Y..BOX_Y + BOX_H {
        for x in BOX_X..BOX_X + BOX_W {
            let brightness = calc_brightness(x as f32, y as f32, light);
            let color = apply_light(0x1A_1A_2E, brightness); // dark background color
            buffer[y * WIDTH + x] = color;
        }
    }
}

pub fn calc_brightness(px: f32, py: f32, light: &Light) -> f32 {
    let dx = px - light.pos.x;
    let dy = py - light.pos.y;
    let dist = (dx * dx + dy * dy).sqrt();

    let ambient = 0.15; // dim but visible in dark areas
    let brightness = (1.0 - (dist / light.radius)).max(0.0);
    let brightness = brightness * brightness * light.intensity; // quadratic falloff, feels more natural
    (ambient + brightness).min(1.0)
}

pub fn apply_light(base_color: u32, brightness: f32) -> u32 {
    let r = ((base_color >> 16) & 0xFF) as f32;
    let g = ((base_color >> 8) & 0xFF) as f32;
    let b = (base_color & 0xFF) as f32;

    // warm tint - boost red/green slightly, keep blue cool
    let warm_r = (r * brightness * 1.2).min(255.0) as u32;
    let warm_g = (g * brightness * 1.0).min(255.0) as u32;
    let warm_b = (b * brightness * 0.7).min(255.0) as u32;

    (warm_r << 16) | (warm_g << 8) | warm_b
}
