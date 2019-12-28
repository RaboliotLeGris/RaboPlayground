use crate::fractals::traits::FractalGenerator;

pub struct Julia {
    max_iteration: i32,
    left_bound: f32,
    right_bound: f32,
    upper_bound: f32,
    lower_bound: f32,
}

impl Julia {
    pub fn new() -> Box<Julia> {
        Box::new(Julia {
            max_iteration: 250,
            left_bound: -1.0,
            right_bound: 1.0,
            upper_bound: -1.2,
            lower_bound: 1.2,
        })
    }
}

impl FractalGenerator for Julia {
    fn generate(&self, width: usize, height: usize) -> Vec<[u8; 3]> {
        let zoom_x: f32 = width as f32 / (self.right_bound - self.left_bound);
        let zoom_y: f32 = height as f32 / (self.lower_bound - self.upper_bound);

        let mut buffer: Vec<[u8; 3]> = vec![[0, 0, 0]; (width * height) as usize];

        let color_scale = 255.0 / self.max_iteration as f32;
        let colors = (0..self.max_iteration).map(|x| (x as f32 * color_scale) as u8).collect::<Vec<u8>>();

        for x in 0..width {
            for y in 0..height {
                let c_r: f32 = 0.285;
                let c_i: f32 = 0.01;
                let mut z_r: f32 = x as i32 as f32 / zoom_x + self.left_bound;
                let mut z_i: f32 = y as i32 as f32 / zoom_y + self.upper_bound;
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
}