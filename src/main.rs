mod vec3;
mod ray;
mod sphere;
mod material;
mod camera;
mod hittable;
mod color;
mod config;

use std::io::{self};
use rayon::prelude::*;
use rand::prelude::*;
use rand::thread_rng;

use vec3::Vec3;
use ray::Ray;
use sphere::Sphere;
use material::MaterialType;
use camera::Camera;
use hittable::{Hittable, HittableList};
use color::Color;
use config::RenderConfig;

type Point3 = Vec3;

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

    let unit_direction = r.dir.unit();
    let t = 0.5 * (unit_direction.y + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

fn create_scene() -> HittableList {
    let mut world = HittableList::new();

    // Ground
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        MaterialType::Lambertian { 
            albedo: Color::new(0.8, 0.8, 0.0) 
        },
    )));

    // Center sphere
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        MaterialType::Lambertian { 
            albedo: Color::new(0.1, 0.2, 0.5) 
        },
    )));

    // Left sphere (metal)
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        MaterialType::Metal { 
            albedo: Color::new(0.8, 0.8, 0.9), 
            fuzz: 0.0 
        },
    )));

    // Right sphere (metal)
    world.add(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        MaterialType::Metal { 
            albedo: Color::new(0.8, 0.6, 0.2), 
            fuzz: 1.0 
        },
    )));

    world
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = RenderConfig::default();
    let image_height = (config.image_width as f64 / config.aspect_ratio) as usize;

    // Camera
    let cam = Camera::new();

    // World
    let world = create_scene();

    // Render
    println!("P3\n{} {}\n255", config.image_width, image_height);
    eprintln!("Starting render...");

    let pixels: Vec<Color> = (0..image_height)
        .into_par_iter()
        .rev()
        .map(|j| {
            if j % 50 == 0 {
                eprintln!("Scanlines remaining: {}", j);
            }
            
            (0..config.image_width)
                .into_par_iter()
                .map(|i| {
                    let mut pixel_color = Color::zero();
                    let mut rng = thread_rng();
                    
                    for _ in 0..config.samples_per_pixel {
                        let u = (i as f64 + rng.gen_range(0.0..1.0)) / (config.image_width - 1) as f64;
                        let v = (j as f64 + rng.gen_range(0.0..1.0)) / (image_height - 1) as f64;
                        let r = cam.get_ray(u, v);
                        pixel_color += ray_color(&r, &world, config.max_depth);
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

    eprintln!("Done!");
    Ok(())
}
