use crate::fractals::traits::FractalGenerator;

pub struct Mandelbrot {}

impl Mandelbrot {
    pub fn new() -> Box<Mandelbrot> {
        Box::new(Mandelbrot{})
    }
}

const X1: f32 = -2.1;
const X2: f32 = 0.6;
const Y1: f32 = -1.2;
const Y2: f32 = 1.2;

const MAX_ITERATION: i32 = 100;

impl FractalGenerator for Mandelbrot {
    fn generate(&self, width: usize, height: usize) -> Vec<[u8;3]> {
        let zoom_x: f32 = width as f32 / (X2 - X1);
        let zoom_y: f32 = height as f32 / (Y2 - Y1);

        let mut buffer: Vec<[u8; 3]> = vec![[0, 0, 0]; (width * height) as usize];

        let color_scale = 255.0 / MAX_ITERATION as f32;
        let colors = (0..MAX_ITERATION).map(|x| (x as f32 * color_scale) as u8).collect::<Vec<u8>>();

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
                    std::mem::replace(&mut buffer[(y * width + x) as usize], [0, 0, 0]);
                } else {
                    std::mem::replace(&mut buffer[(y * width + x) as usize], [0, colors[i as usize], 0]);
                }
            }
        }
        buffer
    }
}