pub struct RenderConfig {
    pub image_width: usize,
    pub aspect_ratio: f64,
    pub samples_per_pixel: usize,
    pub max_depth: u32,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            image_width: 400,
            aspect_ratio: 16.0 / 9.0,
            samples_per_pixel: 100,
            max_depth: 50,
        }
    }
}
