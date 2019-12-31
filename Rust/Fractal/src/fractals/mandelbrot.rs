use crate::fractals::traits::FractalGenerator;
use crate::fractals::zoom::Zoom;

const DEFAULT_LEFT_BOUND: f32 = -2.1;
const DEFAULT_RIGHT_BOUND: f32 = 0.6;
const DEFAULT_UPPER_BOUND: f32 = -1.2;
const DEFAULT_LOWER_BOUND: f32 = 1.2;

const DEFAULT_ITERATION: u32 = 500;

pub struct Mandelbrot {}

impl Mandelbrot {
    pub fn new() -> Box<Mandelbrot> {
        Box::new(Mandelbrot {})
    }
}


impl FractalGenerator for Mandelbrot {
    fn generate(&self, width: usize, height: usize, zoom: Option<&Zoom>) -> Vec<[u8; 3]> {
        let mut max_iteration = DEFAULT_ITERATION;

        let mut left_bound: f32 = DEFAULT_LEFT_BOUND;
        let mut right_bound: f32 = DEFAULT_RIGHT_BOUND;
        let mut upper_bound: f32 = DEFAULT_UPPER_BOUND;
        let mut lower_bound: f32 = DEFAULT_LOWER_BOUND;

        if let Some(zoom) = zoom {
//            max_iteration = max_iteration * zoom.zoom_factor;
            let diff_x = zoom.x / width as u32 as f32;
            let diff_y = zoom.y / height as u32 as f32;
            let x_size = (DEFAULT_LEFT_BOUND - DEFAULT_RIGHT_BOUND).abs();
            let y_size = (DEFAULT_UPPER_BOUND - DEFAULT_LOWER_BOUND).abs();

            let possible_left_bound = DEFAULT_LEFT_BOUND + (x_size * diff_x) - (x_size * diff_x) / (2.0 * zoom.zoom_factor as f32);
            let possible_right_bound = DEFAULT_RIGHT_BOUND - (x_size * (1.0 - diff_x)) + (x_size * diff_x) / (2.0 * zoom.zoom_factor as f32);
            let possible_upper_bound = DEFAULT_UPPER_BOUND + (y_size * diff_y) - (y_size * diff_y) / (2.0 * zoom.zoom_factor as f32);
            let possible_lower_bound = DEFAULT_LOWER_BOUND - (y_size * (1.0 - diff_y)) + (y_size * diff_y) / (2.0 * zoom.zoom_factor as f32);

            left_bound = possible_left_bound;
            right_bound = possible_right_bound;
            upper_bound = possible_upper_bound;
            lower_bound = possible_lower_bound;
        }


        let zoom_x: f32 = width as f32 / (right_bound - left_bound);
        let zoom_y: f32 = height as f32 / (lower_bound - upper_bound);

        let mut buffer: Vec<[u8; 3]> = vec![[0, 0, 0]; (width * height) as usize];
        let colors = self.get_colors(&(max_iteration as f32));

        for x in 0..width {
            for y in 0..height {
                let c_r: f32 = x as i32 as f32 / zoom_x + left_bound;
                let c_i: f32 = y as i32 as f32 / zoom_y + upper_bound;
                let mut z_r: f32 = 0.0;
                let mut z_i: f32 = 0.0;
                let mut i: u32 = 0;

                while z_r * z_r + z_i * z_i < 4.0 && i < max_iteration {
                    let tmp = z_r;
                    z_r = z_r * z_r - z_i * z_i + c_r;
                    z_i = 2.0 * tmp * z_i + c_i;
                    i = i + 1;
                }

                if i == max_iteration {
                    std::mem::replace(&mut buffer[(y * width + x) as usize], [0, 0, 0]);
                } else {
                    std::mem::replace(&mut buffer[(y * width + x) as usize], [0, colors[i as usize], 0]);
                }
            }
        }
        buffer
    }

    fn get_colors(&self, iteration_number: &f32) -> Vec<u8> {
        let color_scale = 255.0 / *iteration_number;
        (0..*iteration_number as i32).map(|x| (x as f32 * color_scale) as u8).collect::<Vec<u8>>()
    }
}