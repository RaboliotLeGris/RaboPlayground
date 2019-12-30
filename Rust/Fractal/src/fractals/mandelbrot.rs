use crate::fractals::traits::FractalGenerator;

pub struct Mandelbrot {
    max_iteration: i32,
    left_bound: f32,
    right_bound: f32,
    upper_bound: f32,
    lower_bound: f32,
}

impl Mandelbrot {
    pub fn new() -> Box<Mandelbrot> {
        Box::new(Mandelbrot{
            max_iteration: 100,
            left_bound: -2.1,
            right_bound: 0.6,
            upper_bound: -1.2,
            lower_bound: 1.2,
        })
    }
}

impl FractalGenerator for Mandelbrot {
    fn generate(&self, width: usize, height: usize) -> Vec<[u8;3]> {
        let zoom_x: f32 = width as f32 / (self.right_bound - self.left_bound);
        let zoom_y: f32 = height as f32 / (self.lower_bound - self.upper_bound);

        let mut buffer: Vec<[u8; 3]> = vec![[0, 0, 0]; (width * height) as usize];
        let colors = self.get_colors();

        for x in 0..width {
            for y in 0..height {
                let c_r: f32 = x as i32 as f32 / zoom_x + self.left_bound;
                let c_i: f32 = y as i32 as f32 / zoom_y + self.upper_bound;
                let mut z_r: f32 = 0.0;
                let mut z_i: f32 = 0.0;
                let mut i: i32 = 0;

                while z_r * z_r + z_i * z_i < 4.0 && i < self.max_iteration {
                    let tmp = z_r;
                    z_r = z_r * z_r - z_i * z_i + c_r;
                    z_i = 2.0 * tmp * z_i + c_i;
                    i = i + 1;
                }

                if i == self.max_iteration {
                    std::mem::replace(&mut buffer[(y * width + x) as usize], [0, 0, 0]);
                } else {
                    std::mem::replace(&mut buffer[(y * width + x) as usize], [0, colors[i as usize], 0]);
                }
            }
        }
        buffer
    }

    fn max_iteration(&self) -> i32 {
        self.max_iteration
    }
}