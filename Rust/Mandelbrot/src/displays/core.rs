use std::error::Error;

pub struct Core {}

const X1: f32 = -2.1;
const X2: f32 = 0.6;
const Y1: f32 = -1.2;
const Y2: f32 = 1.2;

const ZOOM: f32 = 100.0;

const MAX_ITERATION: i32 = 50;

impl Core {
    pub fn mandelbrot() -> Result<Vec<[u8; 3]>, Box<dyn Error>> {
        let image_x: usize = ((X2 - X1) * ZOOM) as u32 as usize;
        let image_y: usize = ((Y2 - Y1) * ZOOM) as u32 as usize;

        let mut buffer: Vec<[u8; 3]> = vec![[0, 0, 0]; (image_x * image_y) as usize];

        for x in 0..image_x {
            for y in 0..image_y {
                let c_r: f32 = x as i32 as f32 / ZOOM + X1;
                let c_i: f32 = y as i32 as f32 / ZOOM + Y1;
                let mut z_r: f32 = 0.0;
                let mut z_i: f32 = 0.0;
                let mut i: i32 = 0;

                while {
                    let tmp = z_r;
                    z_r = z_r * z_r - z_i * z_i + c_r;
                    z_i = 2.0 * tmp * z_i + c_i;
                    i = i + 1;

                    z_r * z_r + z_i * z_i < 4.0 && i < MAX_ITERATION
                } {}

                if i == MAX_ITERATION {
                    std::mem::replace(&mut buffer[y * image_x + x], [255, 255, 255]);
                }
            }
        }

        Ok(buffer)
    }
}

// Display part
pub trait Display {
    fn show();
}
