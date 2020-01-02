use crate::{
    fractals::mandelbrot::Mandelbrot,
    fractals::julia::Julia,
    fractals::buddha::Buddha
};

pub mod traits;

mod mandelbrot;
mod julia;
mod buddha;

pub struct FractalFactory {}

impl FractalFactory {
    pub fn make(fractal_name: Option<&String>) -> Box<dyn traits::FractalGenerator> {
        match fractal_name {
            Some(n) => match n.as_str() {
                "mandelbrot" => Mandelbrot::new(),
                "julia" => Julia::new(),
                "buddha" => Buddha::new(),
                _ => Mandelbrot::new(),
            },
            _ => Mandelbrot::new()
        }
    }
}