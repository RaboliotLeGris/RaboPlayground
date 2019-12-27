extern crate image;

use super::core::{Display, Core};

pub struct Fs {}

impl Display for Fs {
    fn show(width: usize, height: usize) {
        if let Ok(buffer) = Core::mandelbrot(width as u32, height as u32) {
            let image = image::ImageBuffer::from_fn(width as u32, height as u32, |x, y| {
                let pixel: &[u8; 3] = buffer.get((y * width as u32 + x) as usize).unwrap();
                let red = pixel[0];
                let green = pixel[1];
                let blue = pixel[2];
                image::Rgb([red, green, blue])
            });
            image.save("mandelbrot.png").unwrap();
        }
    }
}