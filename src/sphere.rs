use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hittable::{Hittable, HitRecord};
use crate::material::MaterialType;

pub type Point3 = Vec3;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub mat: MaterialType,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: MaterialType) -> Self {
        Self {
            center,
            radius,
            mat: material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.orig - self.center;
        let a = r.dir.length_squared();
        let half_b = oc.dot(r.dir);
        let c = oc.length_squared() - self.radius * self.radius;
        
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        
        let sqrtd = discriminant.sqrt();
        
        // Find the nearest root that lies in the acceptable range
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }
        
        let p = r.at(root);
        let outward_normal = (p - self.center) / self.radius;
        let (normal, front_face) = HitRecord::set_face_normal(r, outward_normal);
        
        Some(HitRecord {
            t: root,
            p,
            normal,
            front_face,
            mat: self.mat.clone(),
        })
    }
}
