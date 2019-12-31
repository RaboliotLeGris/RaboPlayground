extern crate image;

use crate::fractals::traits::FractalGenerator;
use crate::displays::traits::Display;

pub struct Fs {}

impl Display for Fs {
    fn show(width: usize, height: usize, fractal_generator: Box<dyn FractalGenerator>) {
        let buffer = fractal_generator.generate(width, height, None);
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