mod vec3;
mod ray;
mod sphere;
mod material;
mod camera;
mod hittable;
mod color;
mod config;
mod utils;

use std::io::{self, Write};
use rayon::prelude::*;
use vec3::Vec3;
use ray::Ray;
use sphere::Sphere;
use material::MaterialType;
use camera::Camera;
use hittable::{Hittable, HittableList};
use color::Color;
use config::RenderConfig;
use utils::*;

type Point3 = Vec3;

fn ray_color(r: &Ray, world: &HittableList, depth: u32, rng: &mut fastrand::Rng) -> Color {
    if depth == 0 {
        return Color::zero();
    }

    // Use smaller epsilon to reduce shadow acne
    if let Some(rec) = world.hit(r, 0.0001, f64::INFINITY) {
        if let Some((scattered, attenuation)) = rec.mat.scatter(r, &rec, rng) {
            return attenuation * ray_color(&scattered, world, depth - 1, rng);
        }
        return Color::zero();
    }

    // Enhanced sky gradient
    let unit_direction = r.dir.unit();
    let t = 0.5 * (unit_direction.y + 1.0);
    let horizon_color = Color::new(1.0, 1.0, 1.0);
    let zenith_color = Color::new(0.5, 0.7, 1.0);
    horizon_color * (1.0 - t) + zenith_color * t
}

fn create_scene() -> HittableList {
    let mut world = HittableList::new();

    // Ground - checkered pattern effect with slight roughness
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        MaterialType::Lambertian { 
            albedo: Color::new(0.8, 0.8, 0.0) 
        },
    )));

    // Center sphere - Lambertian
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        MaterialType::Lambertian { 
            albedo: Color::new(0.1, 0.2, 0.5) 
        },
    )));

    // Left sphere - Dielectric (glass)
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        MaterialType::Dielectric { ir: 1.5 },
    )));

    // Left sphere inner - hollow glass effect
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        -0.4,
        MaterialType::Dielectric { ir: 1.5 },
    )));

    // Right sphere - polished metal
    world.add(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        MaterialType::Metal { 
            albedo: Color::new(0.8, 0.6, 0.2), 
            fuzz: 0.0  // Perfect mirror
        },
    )));

    // Additional smaller spheres for interest
    world.add(Box::new(Sphere::new(
        Point3::new(-0.5, -0.25, -0.5),
        0.25,
        MaterialType::Metal { 
            albedo: Color::new(0.9, 0.9, 0.9), 
            fuzz: 0.1 
        },
    )));

    world.add(Box::new(Sphere::new(
        Point3::new(0.5, -0.25, -0.5),
        0.25,
        MaterialType::Lambertian { 
            albedo: Color::new(0.9, 0.2, 0.2) 
        },
    )));

    world
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = RenderConfig::new();
    let image_height = (config.image_width as f64 / config.aspect_ratio) as usize;

    // Enhanced camera with depth of field
    let lookfrom = Point3::new(3.0, 3.0, 2.0);
    let lookat = Point3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (lookfrom - lookat).length();
    let aperture = 0.1;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0, // vfov
        config.aspect_ratio,
        aperture,
        dist_to_focus,
    );

    let world = create_scene();

    // Render header
    println!("P3\n{} {}\n255", config.image_width, image_height);
    eprintln!("Rendering {}x{} with {} samples per pixel...", 
        config.image_width, image_height, config.samples_per_pixel);

    let start_time = std::time::Instant::now();

    // Parallel rendering with thread-local RNG
    let pixels: Vec<Color> = (0..image_height)
        .into_par_iter()
        .rev()
        .map(|j| {
            if j % 20 == 0 {
                eprintln!("Scanlines remaining: {}", j);
            }
            
            (0..config.image_width)
                .into_par_iter()
                .map(|i| {
                    let mut pixel_color = Color::zero();
                    let mut rng = fastrand::Rng::new();
                    
                    for _ in 0..config.samples_per_pixel {
                        let u = (i as f64 + rng.f64()) / (config.image_width - 1) as f64;
                        let v = (j as f64 + rng.f64()) / (image_height - 1) as f64;
                        let r = cam.get_ray(u, v, &mut rng);
                        pixel_color += ray_color(&r, &world, config.max_depth, &mut rng);
                    }
                    pixel_color
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();

    // Output pixels
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    
    for pixel in pixels {
        pixel.write_color(&mut handle, config.samples_per_pixel)?;
    }

    let elapsed = start_time.elapsed();
    eprintln!("Render completed in {:.2} seconds!", elapsed.as_secs_f64());
    Ok(())
}
