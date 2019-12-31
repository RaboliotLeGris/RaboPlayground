use crate::fractals::zoom::Zoom;

pub trait FractalGenerator {
    fn generate(&self, width: usize, height: usize, zoom_location: Option<&Zoom>) -> Vec<[u8;3]>;

    fn get_colors(&self, iteration_number: &f32) -> Vec<u8>;
}