use crate::fractals::traits::FractalGenerator;

use std::cmp::min;

pub struct Buddha {}

impl Buddha {
    pub fn new() -> Box<Buddha> {
        Box::new(Buddha {})
    }
}

const MAX_ITERATION: u32 = 50000;
const LEFT_BOUND: f32 = -2.1;
const RIGHT_BOUND: f32 = 0.6;
const UPPER_BOUND: f32 = -1.2;
const LOWER_BOUND: f32 = 1.2;

impl FractalGenerator for Buddha {
    fn generate(&self, width: usize, height: usize) -> Vec<[u8; 3]> {
        let zoom_x: f32 = width as f32 / (RIGHT_BOUND - LEFT_BOUND);
        let zoom_y: f32 = height as f32 / (LOWER_BOUND - UPPER_BOUND);

        let mut buffer: Vec<[u8; 3]> = vec![[0, 0, 0]; (width * height) as usize];

        for x in 0..width {
            for y in 0..height {
                let c_r: f32 = x as i32 as f32 / zoom_x + LEFT_BOUND;
                let c_i: f32 = y as i32 as f32 / zoom_y + UPPER_BOUND;
                let mut z_r: f32 = 0.0;
                let mut z_i: f32 = 0.0;
                let mut i: u32 = 0;
                let mut tmp_pixels = vec!([0, 0]);

                while z_r * z_r + z_i * z_i < 4.0 && i < MAX_ITERATION {
                    let tmp = z_r;
                    z_r = z_r * z_r - z_i * z_i + c_r;
                    z_i = 2.0 * tmp * z_i + c_i;
                    i = i + 1;
                    tmp_pixels.push([((z_r - LEFT_BOUND) * zoom_x) as u32, ((z_i - UPPER_BOUND) * zoom_y) as u32]);
                }

                if i != MAX_ITERATION {
                    for pixel in tmp_pixels {
                        if pixel[0] < std::usize::MAX as u32 && pixel[1] < std::usize::MAX as u32 {
                            let pos = (pixel[1] as usize).saturating_mul(width as usize).saturating_add(pixel[0] as usize);
                            if let Some(buffer_value) = buffer.get(pos) {
                                let updated_value = [buffer_value[0].saturating_add(1), 0, 0];
                                std::mem::replace(&mut buffer[pos], updated_value);
                            }
                        }
                    }
                }
            }
        }

        for pos in 0..width * height {
            let min_value = min(buffer[pos][0], 255);
            std::mem::replace(&mut buffer[pos], [min_value, min_value, min_value]);
        }

        buffer
    }
}