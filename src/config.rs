pub struct RenderConfig {
    pub image_width: usize,
    pub aspect_ratio: f64,
    pub samples_per_pixel: usize,
    pub max_depth: u32,
}

impl RenderConfig {
    pub fn new() -> Self {
        Self {
            image_width: 800,  // Higher resolution
            aspect_ratio: 3.0 / 2.0,
            samples_per_pixel: 500,  // More samples for quality
            max_depth: 50,
        }
    }

    pub fn quick() -> Self {
        Self {
            image_width: 400,
            aspect_ratio: 3.0 / 2.0,
            samples_per_pixel: 100,
            max_depth: 20,
        }
    }
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self::quick()
    }
}
