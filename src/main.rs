// src/main.rs

mod vec3;
mod ray;
mod hittable;
mod sphere;
mod material;
mod camera;

use std::fs::File;
use std::io::Write;
use rand::Rng;

use vec3::{Color, Point3, Vec3};
use ray::Ray;
use hittable::{HittableList};
use sphere::Sphere;
use material::{Lambertian, Metal};
use camera::Camera;

fn ray_color(r: &Ray, world: &HittableList, depth: u32) -> Color {
    if depth == 0 {
        return Color::zero();
    }
    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        if let Some((scattered, attenuation)) = rec.mat.scatter(r, &rec) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        return Color::zero();
    }
    let unit = r.dir.unit();
    let t = 0.5 * (unit.y + 1.0);
    Vec3::lerp(Color::new(1.0,1.0,1.0), Color::new(0.5,0.7,1.0), t)
}

fn main() -> std::io::Result<()> {
    // --- Image Settings ---
    let aspect_ratio     = 16.0 / 9.0;
    let image_width      = 400;
    let image_height     = (image_width as f64 / aspect_ratio) as usize;
    let samples_per_pixel = 100;
    let max_depth         = 50;

    // --- World Setup ---
    let mut world = HittableList::new();
    // ground
    world.add(Box::new(Sphere::new(
        Point3::new( 0.0, -100.5, -1.0),
        100.0,
        Box::new(Lambertian::new(Color::new(0.8,0.8,0.0))),
    )));
    // center
    world.add(Box::new(Sphere::new(
        Point3::new( 0.0,   0.0, -1.0),
        0.5,
        Box::new(Lambertian::new(Color::new(0.7,0.3,0.3))),
    )));
    // right (gold metal)
    world.add(Box::new(Sphere::new(
        Point3::new( 1.0,   0.0, -1.0),
        0.5,
        Box::new(Metal::new(Color::new(0.8,0.6,0.2), 0.3)),
    )));
    // left (mirror)
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0,   0.0, -1.0),
        0.5,
        Box::new(Metal::new(Color::new(0.8,0.8,0.8), 0.0)),
    )));

    // --- Camera ---
    let cam = Camera::new();

    // --- Render to PPM ---
    let mut file = File::create("output.ppm")?;
    writeln!(file, "P3\n{} {}\n255", image_width, image_height)?;

    let mut rng = rand::thread_rng();  // 2) this is correct

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let mut pixel_color = Color::zero();

            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rng.random_range(0.0..1.0)) / ((image_width - 1) as f64);
                let v = (j as f64 + rng.random_range(0.0..1.0)) / ((image_height - 1) as f64);

                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, max_depth);
            }

            pixel_color.write_color(&mut file, samples_per_pixel)?;
        }
    }

    println!("Render complete → output.ppm");
    Ok(())
}
