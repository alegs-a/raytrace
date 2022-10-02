use raytrace::camera::Camera;
use raytrace::color::Color;
use raytrace::hittable::{HitRecord, Hittable};
use raytrace::hittable_list::HittableList;
use raytrace::material::Material;
use raytrace::math::random_f64;
use raytrace::ray::Ray;
use raytrace::sphere::Sphere;
use raytrace::vec3::Vec3;

use std::f64::INFINITY;

fn main() {
    // Image

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World

    let mut world = HittableList::default();
    let material_ground = Box::new(Material::Lambertian {
        albedo: Color::new(0.8, 0.8, 0.0),
    });
    let material_center = Box::new(Material::Lambertian {
        albedo: Color::new(0.7, 0.3, 0.3),
    });
    let material_left = Box::new(Material::Metal {
        albedo: Color::new(0.8, 0.8, 0.8),
        fuzz: 0.3,
    });
    let material_right = Box::new(Material::Metal {
        albedo: Color::new(0.8, 0.6, 0.2),
        fuzz: 0.0,
    });

    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    // Camera

    let cam = Camera::new();

    // Render

    println!("P3\n{image_width} {image_height}\n255"); // .ppm header

    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {j}"); // progress indicator
        for i in 0..image_width {
            let mut pixel_color = Color::black();
            for _s in 0..samples_per_pixel {
                let u = (i as f64 + random_f64()) / ((image_width - 1) as f64);
                let v = (j as f64 + random_f64()) / ((image_height - 1) as f64);
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, max_depth);
            }
            Color::write_color(pixel_color, samples_per_pixel);
        }
    }
}

/// Calculate the color of a ray, including the case where it intersects geometry
fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    let mut rec = HitRecord::default();

    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return Color::black();
    }

    if world.hit(r, 0.001, INFINITY, &mut rec) {
        let mut scattered = Ray::default();
        let mut attenuation = Color::default();
        if rec
            .material
            .scatter(r, &rec, &mut attenuation, &mut scattered)
        {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        return Color::black();
    }

    let unit_direction: Vec3 = r.dir.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    (Color::white() * (1.0 - t)) + (Color::new(0.5, 0.7, 1.0) * t)
}
