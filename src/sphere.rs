use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use crate::material::Material;

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat:    Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Box<dyn Material>) -> Self {
        Self { center, radius, mat }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.orig - self.center;
        let a = r.dir.dot(r.dir);
        let half_b = oc.dot(r.dir);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = half_b*half_b - a*c;
        if discriminant < 0.0 { return None }
        let sqrtd = discriminant.sqrt();

        // find the nearest root in range
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let p = r.at(root);
        let outward = (p - self.center) / self.radius;
        let (normal, front_face) = HitRecord::set_face_normal(r, outward);

        Some(HitRecord {
            p,
            normal,
            t: root,
            front_face,
            mat: self.mat.clone_box(),
        })
    }
}
