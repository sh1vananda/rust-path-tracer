use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::color::Color;
use crate::utils::*;

#[derive(Clone, Debug)]
pub enum MaterialType {
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz: f64 },
    Dielectric { ir: f64 }, // Index of refraction
}

impl MaterialType {
    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord, rng: &mut fastrand::Rng) -> Option<(Ray, Color)> {
        match self {
            MaterialType::Lambertian { albedo } => {
                let mut scatter_dir = rec.normal + random_unit_vector(rng);
                
                // Catch degenerate scatter direction
                if scatter_dir.near_zero() {
                    scatter_dir = rec.normal;
                }
                
                // Better shadow bias
                let scattered = Ray::new(rec.p + rec.normal * 1e-6, scatter_dir);
                Some((scattered, *albedo))
            }
            
            MaterialType::Metal { albedo, fuzz } => {
                let reflected = r_in.dir.unit().reflect(rec.normal);
                let fuzz_clamped = fuzz.min(1.0);
                let scattered = Ray::new(
                    rec.p + rec.normal * 1e-6,  // Better bias
                    reflected + random_in_unit_sphere(rng) * fuzz_clamped,
                );
                
                if scattered.dir.dot(rec.normal) > 0.0 {
                    Some((scattered, *albedo))
                } else {
                    None
                }
            }
            
            MaterialType::Dielectric { ir } => {
                let attenuation = Color::new(1.0, 1.0, 1.0);
                let refraction_ratio = if rec.front_face { 1.0 / ir } else { *ir };
                
                let unit_direction = r_in.dir.unit();
                let cos_theta = (-unit_direction).dot(rec.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
                
                let cannot_refract = refraction_ratio * sin_theta > 1.0;
                
                let direction = if cannot_refract || reflectance(cos_theta, refraction_ratio) > rng.f64() {
                    unit_direction.reflect(rec.normal)
                } else {
                    refract(unit_direction, rec.normal, refraction_ratio)
                };
                
                let scattered = Ray::new(rec.p, direction);
                Some((scattered, attenuation))
            }
        }
    }
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    // Schlick's approximation for reflectance
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = (-uv).dot(n).min(1.0);
    let r_out_perp = (uv + n * cos_theta) * etai_over_etat;
    let r_out_parallel = n * -((1.0 - r_out_perp.length_squared()).abs().sqrt());
    r_out_perp + r_out_parallel
}
