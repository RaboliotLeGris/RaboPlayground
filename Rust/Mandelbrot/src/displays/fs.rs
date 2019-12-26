extern crate image;

use super::core::{Display, Core};

const X1: f32 = -2.1;
const X2: f32 = 0.6;
const Y1: f32 = -1.2;
const Y2: f32 = 1.2;

const ZOOM: f32 = 100.0;

pub struct Fs {}

impl Display for Fs {
    fn show() {
        let image_x: u32 = ((X2 - X1) * ZOOM) as u32;
        let image_y: u32 = ((Y2 - Y1) * ZOOM) as u32;

        if let Ok(buffer) = Core::mandelbrot() {
            let image = image::ImageBuffer::from_fn(image_x, image_y, |x, y| {
                let pixel: &[u8; 3] = buffer.get( (y * image_x + x) as usize).unwrap();
                let red = pixel[0];
                let green = pixel[1];
                let blue = pixel[2];
                image::Rgb([red, green, blue])
            });
            image.save("mandelbrot.png").unwrap();
        }
    }
}