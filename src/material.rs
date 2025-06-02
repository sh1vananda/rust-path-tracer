use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::color::Color;

#[derive(Clone, Debug)]
pub enum MaterialType {
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz: f64 },
}

impl MaterialType {
    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        match self {
            MaterialType::Lambertian { albedo } => {
                let mut scatter_dir = rec.normal + Vec3::random_unit();
                
                // Catch degenerate scatter direction
                if scatter_dir.near_zero() {
                    scatter_dir = rec.normal;
                }
                
                let scattered = Ray::new(rec.p, scatter_dir);
                Some((scattered, *albedo))
            }
            MaterialType::Metal { albedo, fuzz } => {
                let reflected = r_in.dir.unit().reflect(rec.normal);
                let fuzz_clamped = fuzz.min(1.0);
                let scattered = Ray::new(
                    rec.p,
                    reflected + Vec3::random_in_unit_sphere() * fuzz_clamped,
                );
                
                if scattered.dir.dot(rec.normal) > 0.0 {
                    Some((scattered, *albedo))
                } else {
                    None
                }
            }
        }
    }
}
