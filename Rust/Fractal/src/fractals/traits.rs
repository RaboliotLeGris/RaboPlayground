pub trait FractalGenerator {
    fn generate(&self, width: usize, height: usize) -> Vec<[u8;3]>;
}