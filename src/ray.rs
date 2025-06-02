use crate::vec3::Point3;

#[derive(Clone, Copy)]
pub struct Ray {
    pub orig: Point3,
    pub dir:  Point3,
}

impl Ray {
    #[inline]
    pub fn new(orig: Point3, dir: Point3) -> Self { Self { orig, dir } }
    #[inline]
    pub fn at(&self, t: f64) -> Point3 { self.orig + self.dir * t }
}
