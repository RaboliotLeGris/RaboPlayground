pub struct Zoom {
    pub x: f32,
    pub y: f32,
    pub zoom_factor: u32,
}

impl Zoom {
    pub fn new(x: f32, y: f32, zoom_factor: u32) -> Self {
        Zoom {
            x,
            y,
            zoom_factor,
        }
    }

    pub fn increase_zoom(&mut self) {
        self.zoom_factor = self.zoom_factor * 2;
    }

    pub fn set_location(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }
}