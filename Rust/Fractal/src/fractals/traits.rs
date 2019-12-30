pub trait FractalGenerator {
    fn generate(&self, width: usize, height: usize) -> Vec<[u8;3]>;
    fn max_iteration(&self) -> i32;

    fn get_colors(&self) -> Vec<u8> {
        let color_scale = 255.0 / self.max_iteration() as f32;
        (0..self.max_iteration()).map(|x| (x as f32 * color_scale) as u8).collect::<Vec<u8>>()
    }
}