use crate::fractals::traits::FractalGenerator;

pub struct Mandelbrot {}

impl Mandelbrot {
    pub fn new() -> Box<Mandelbrot> {
        Box::new(Mandelbrot {})
    }
}

const MAX_ITERATION: u32 = 250;
const LEFT_BOUND: f32 = -2.1;
const RIGHT_BOUND: f32 = 0.6;
const UPPER_BOUND: f32 = -1.2;
const LOWER_BOUND: f32 = 1.2;

impl FractalGenerator for Mandelbrot {
    fn generate(&self, width: usize, height: usize) -> Vec<[u8; 3]> {
        let zoom_x: f32 = width as f32 / (RIGHT_BOUND - LEFT_BOUND);
        let zoom_y: f32 = height as f32 / (LOWER_BOUND - UPPER_BOUND);

        let mut buffer: Vec<[u8; 3]> = vec![[0, 0, 0]; (width * height) as usize];

        let color_scale = 255.0 / MAX_ITERATION as f32;
        let colors = (0..MAX_ITERATION).map(|x| (x as f32 * color_scale) as u8).collect::<Vec<u8>>();

        let mut previous_percent = 0;
        for x in 0..width {
            let percent: u32 = ((x as f32 / width as f32) * 100.0) as u32;
            if percent != previous_percent && percent % 10 == 0 { println!("Generation progress: {}", percent); previous_percent = percent; }
            for y in 0..height {
                let c_r: f32 = x as i32 as f32 / zoom_x + LEFT_BOUND;
                let c_i: f32 = y as i32 as f32 / zoom_y + UPPER_BOUND;
                let mut z_r: f32 = 0.0;
                let mut z_i: f32 = 0.0;
                let mut i: u32 = 0;

                while z_r * z_r + z_i * z_i < 4.0 && i < MAX_ITERATION {
                    let tmp = z_r;
                    z_r = z_r * z_r - z_i * z_i + c_r;
                    z_i = 2.0 * tmp * z_i + c_i;
                    i = i + 1;
                }

                if i != MAX_ITERATION {
                    std::mem::replace(&mut buffer[(y * width + x) as usize], [0, colors[i as usize], 0]);
                }
            }
        }
        println!("Generation DONE");
        buffer
    }
}