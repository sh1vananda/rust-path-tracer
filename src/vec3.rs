use std::ops::{Add, Sub, Mul, Div, AddAssign, Neg};
use std::io::Write;
use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub struct Vec3 { pub x: f64, pub y: f64, pub z: f64 }

pub type Point3 = Vec3;
pub type Color  = Vec3;

impl Vec3 {
    #[inline] pub fn new(x: f64, y: f64, z: f64) -> Self { Self { x, y, z } }
    #[inline] pub fn zero()           -> Self { Self::new(0.0, 0.0, 0.0) }
    #[inline] pub fn length(&self)    -> f64  { self.dot(*self).sqrt() }
    #[inline] pub fn unit(&self)      -> Self { *self / self.length() }
    #[inline] pub fn dot(&self, o: Vec3) -> f64 {
        self.x*o.x + self.y*o.y + self.z*o.z
    }
    #[inline] pub fn cross(&self, o: Vec3) -> Vec3 {
        Vec3::new(
            self.y*o.z - self.z*o.y,
            self.z*o.x - self.x*o.z,
            self.x*o.y - self.y*o.x,
        )
    }
    #[inline] pub fn lerp(a: Vec3, b: Vec3, t: f64) -> Vec3 {
        a*(1.0 - t) + b*t
    }

    #[inline]
    pub fn random_in_unit_sphere() -> Vec3 {
        let mut rng = rand::thread_rng();
        loop {
            let p = Vec3::new(
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
            );
            if p.dot(p) < 1.0 { return p; }
        }
    }

    #[inline]
    pub fn random_unit() -> Vec3 {
        Vec3::random_in_unit_sphere().unit()
    }

    #[inline]
    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }

    #[inline]
    pub fn reflect(&self, n: Vec3) -> Vec3 {
        *self - n * 2.0 * self.dot(n)
    }

    pub fn write_color(
        &self,
        out: &mut dyn Write,
        samples: usize
    ) -> std::io::Result<()> {
        let scale = 1.0 / samples as f64;
        // gamma-correction (γ=2)
        let r = (self.x * scale).sqrt();
        let g = (self.y * scale).sqrt();
        let b = (self.z * scale).sqrt();
        writeln!(
            out,
            "{} {} {}",
            (256.0 * r.clamp(0.0, 0.999)) as u8,
            (256.0 * g.clamp(0.0, 0.999)) as u8,
            (256.0 * b.clamp(0.0, 0.999)) as u8,
        )
    }
}

// operator overloads
impl Add for Vec3       { type Output = Vec3; fn add(self, o: Vec3) -> Vec3 { Vec3::new(self.x+o.x, self.y+o.y, self.z+o.z) } }
impl Sub for Vec3       { type Output = Vec3; fn sub(self, o: Vec3) -> Vec3 { Vec3::new(self.x-o.x, self.y-o.y, self.z-o.z) } }
impl Mul for Vec3       { type Output = Vec3; fn mul(self, o: Vec3) -> Vec3 { Vec3::new(self.x*o.x, self.y*o.y, self.z*o.z) } }
impl Mul<f64> for Vec3  { type Output = Vec3; fn mul(self, t: f64) -> Vec3 { Vec3::new(self.x*t, self.y*t, self.z*t) } }
impl Div<f64> for Vec3  { type Output = Vec3; fn div(self, t: f64) -> Vec3 { Vec3::new(self.x/t, self.y/t, self.z/t) } }
impl AddAssign for Vec3 { fn add_assign(&mut self, o: Vec3) { *self = *self + o; } }
impl Neg for Vec3       { type Output = Vec3; fn neg(self) -> Vec3 { Vec3::new(-self.x, -self.y, -self.z) } }
