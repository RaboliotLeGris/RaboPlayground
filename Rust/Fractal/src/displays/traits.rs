use crate::fractals::traits::FractalGenerator;

pub trait Display {
    fn show(width: usize, height: usize, fractal_generator: Box<dyn FractalGenerator>);
}
