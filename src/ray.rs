use crate::vec3::Vec3;

pub type Point3 = Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self {
            orig: origin,
            dir: direction,
        }
    }

    #[inline]
    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }
}
