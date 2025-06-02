use crate::vec3::Vec3;

pub fn random_f64(rng: &mut fastrand::Rng) -> f64 {
    rng.f64()
}

pub fn random_f64_range(min: f64, max: f64, rng: &mut fastrand::Rng) -> f64 {
    min + (max - min) * rng.f64()
}

pub fn random_vec3(rng: &mut fastrand::Rng) -> Vec3 {
    Vec3::new(rng.f64(), rng.f64(), rng.f64())
}

pub fn random_vec3_range(min: f64, max: f64, rng: &mut fastrand::Rng) -> Vec3 {
    Vec3::new(
        random_f64_range(min, max, rng),
        random_f64_range(min, max, rng),
        random_f64_range(min, max, rng),
    )
}

pub fn random_in_unit_sphere(rng: &mut fastrand::Rng) -> Vec3 {
    loop {
        let p = random_vec3_range(-1.0, 1.0, rng);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_unit_vector(rng: &mut fastrand::Rng) -> Vec3 {
    random_in_unit_sphere(rng).unit()
}

pub fn random_in_hemisphere(normal: Vec3, rng: &mut fastrand::Rng) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere(rng);
    if in_unit_sphere.dot(normal) > 0.0 {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

pub fn random_in_unit_disk(rng: &mut fastrand::Rng) -> Vec3 {
    loop {
        let p = Vec3::new(
            random_f64_range(-1.0, 1.0, rng),
            random_f64_range(-1.0, 1.0, rng),
            0.0,
        );
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}
