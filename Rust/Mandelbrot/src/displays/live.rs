extern crate minifb;

use minifb::{Key, Window, WindowOptions};
use super::core::{Display, Core};

const X1: f32 = -2.1;
const X2: f32 = 0.6;
const Y1: f32 = -1.2;
const Y2: f32 = 1.2;

const ZOOM: f32 = 100.0;

pub struct Live {}

impl Live {
    fn from_u8_rgba(r: u8, g: u8, b: u8, a: u8) -> u32 {
        let (r, g, b, a) = (r as u32, g as u32, b as u32, a as u32);
        b | (g << 8) | (r << 16) | (a << 24)
    }
}

impl Display for Live {
    fn show() {
        let image_x = ((X2 - X1) * ZOOM) as usize;
        let image_y = ((Y2 - Y1) * ZOOM) as usize;

        let mut buffer: Vec<u32> = vec![0; image_x * image_y];

        let mut window = Window::new(
            "Mandelbrot - ESC to exit",
            image_x,
            image_y,
            WindowOptions::default(),
        ).unwrap_or_else(|e| {
            panic!("{}", e);
        });

        window.limit_update_rate(Some(std::time::Duration::from_secs(1)));

        while window.is_open() && !window.is_key_down(Key::Escape) {
            let res_buffer = Core::mandelbrot().unwrap();

            for (length, item) in res_buffer.iter().enumerate() {
                buffer[length] = Live::from_u8_rgba(item[0], item[1], item[2], 255)
            }

            window
                .update_with_buffer(&buffer, image_x, image_y)
                .unwrap();
        }
    }
}