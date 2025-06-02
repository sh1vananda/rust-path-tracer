use crate::ray::Ray;
use crate::vec3::Point3;

/// Material is a trait object stored in the hit record:
pub struct HitRecord {
    pub p:           Point3,
    pub normal:      Point3,
    pub t:           f64,
    pub front_face:  bool,
    pub mat:         Box<dyn crate::material::Material>,
}

impl HitRecord {
    /// Ensures the normal always opposes the ray direction:
    pub fn set_face_normal(r: &Ray, outward: Point3) -> (Point3, bool) {
        let front = r.dir.dot(outward) < 0.0;
        let normal = if front { outward } else { -outward };
        (normal, front)
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self { Self { objects: Vec::new() } }
    pub fn add(&mut self, obj: Box<dyn Hittable>) { self.objects.push(obj); }
    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest = t_max;
        let mut hit_rec = None;
        for obj in &self.objects {
            if let Some(rec) = obj.hit(r, t_min, closest) {
                closest = rec.t;
                hit_rec = Some(rec);
            }
        }
        hit_rec
    }
}
