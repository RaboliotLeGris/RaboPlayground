extern crate minifb;

use minifb::{Key, Window, WindowOptions, MouseMode, MouseButton};

use crate::fractals::traits::FractalGenerator;
use crate::displays::traits::Display;
use crate::fractals::zoom::Zoom;

pub struct Live {}

impl Display for Live {
    fn show(width: usize, height: usize, fractal_generator: Box<dyn FractalGenerator>) {
        let mut zoom = Zoom::new(0.0, 0.0, 1);
        let mut buffer: Vec<u32> = Live::get_fractal_buffer(&fractal_generator, width, height, None);

        let mut window = Window::new(
            "Mandelbrot - ESC to exit",
            width,
            height,
            WindowOptions::default(),
        ).unwrap_or_else(|e| {
            panic!("{}", e);
        });

        window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

        let mut update_is_needed: bool = false;
        while window.is_open() && !window.is_key_down(Key::Escape) {
            window.get_mouse_pos(MouseMode::Discard).map(|(x, y)| {
                if window.get_mouse_down(MouseButton::Left) {
                    println!("MOUSE LEFT CLICK {} {}", x, y);
                    zoom.increase_zoom();
                    zoom.set_location(x, y);
                    update_is_needed =  true;
                }
            });

            if update_is_needed {
                buffer = Live::get_fractal_buffer(&fractal_generator, width, height, Some(&zoom));
                update_is_needed = false;
            }

            window
                .update_with_buffer(&buffer, width, height)
                .unwrap();
        }
    }
}

impl Live {
    fn from_u8_rgba(r: u8, g: u8, b: u8, a: u8) -> u32 {
        let (r, g, b, a) = (r as u32, g as u32, b as u32, a as u32);
        b | (g << 8) | (r << 16) | (a << 24)
    }

    fn get_fractal_buffer(fractal_generator: &Box<dyn FractalGenerator>, width: usize, height: usize, zoom_location: Option<&Zoom>) -> Vec<u32> {
        let mut buffer: Vec<u32> = vec![0; width * height];
        let res_buffer = fractal_generator.generate(width, height, zoom_location);
        for (length, item) in res_buffer.iter().enumerate() {
            buffer[length] = Live::from_u8_rgba(item[0], item[1], item[2], 255)
        }
        buffer
    }
}
