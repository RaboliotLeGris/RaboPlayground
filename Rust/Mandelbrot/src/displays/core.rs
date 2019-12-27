use std::error::Error;

pub struct Core {}

const X1: f32 = -2.1;
const X2: f32 = 0.6;
const Y1: f32 = -1.2;
const Y2: f32 = 1.2;

const MAX_ITERATION: i32 = 250;

impl Core {
    pub fn mandelbrot(width: u32, height: u32) -> Result<Vec<[u8; 3]>, Box<dyn Error>> {
        let zoom_x: f32 = width as f32 / (X2 - X1);
        let zoom_y: f32 = height as f32 / (Y2 - Y1);

        let mut buffer: Vec<[u8; 3]> = vec![[0, 0, 0]; (width * height) as usize];

        for x in 0..width {
            for y in 0..height {
                let c_r: f32 = x as i32 as f32 / zoom_x + X1;
                let c_i: f32 = y as i32 as f32 / zoom_y + Y1;
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
                    std::mem::replace(&mut buffer[(y * width + x) as usize], [255, 255, 255]);
                }
            }
        }

        Ok(buffer)
    }
}

// Display part
pub trait Display {
    fn show(width: usize, height: usize);
}
