use std::io::{self, Write};
use crate::vec3::Vec3;

pub type Color = Vec3;

impl Color {
    pub fn write_color(&self, out: &mut dyn Write, samples: usize) -> io::Result<()> {
        let scale = 1.0 / samples as f64;
        
        // Gamma correction (gamma = 2.0)
        let to_byte = |component: f64| -> u8 {
            let corrected = (component * scale).sqrt().clamp(0.0, 0.999);
            (256.0 * corrected) as u8
        };
        
        writeln!(out, "{} {} {}", 
            to_byte(self.x), 
            to_byte(self.y), 
            to_byte(self.z)
        )
    }
}
