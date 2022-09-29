use raytrace::camera::Camera;
use raytrace::color::Color;
use raytrace::hittable::{HitRecord, Hittable};
use raytrace::hittable_list::HittableList;
use raytrace::ray::Ray;
use raytrace::sphere::Sphere;
use raytrace::vec3::Vec3;

use rand::Rng;

// use std::f64::consts::PI;
use std::f64::INFINITY;

fn main() {
    // Image

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;

    // World

    let mut world = HittableList::default();
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

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
                pixel_color += ray_color(&r, &world);
            }
            Color::write_color(pixel_color, samples_per_pixel);
        }
    }
}

/// Calculate the color of a ray, including the case where it intersects geometry
fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    let mut rec = HitRecord::default();
    if world.hit(r, 0.0, INFINITY, &mut rec) {
        return (rec.normal + Color::white()) * 0.5;
    }
    let unit_direction: Vec3 = r.dir.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    (Color::white() * (1.0 - t)) + (Color::new(0.5, 0.7, 1.0) * t)
}

fn random_f64() -> f64 {
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(0.0..1.0);
    num
}

#[allow(dead_code)]
fn random_f64_wide(lower: f64, upper: f64) -> f64 {
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(lower..upper);
    num
}

// /// Calculate the solutions to `r.at(t)` for all `t` that makes the ray point to the surface of the
// /// sphere defined by `center` and `radius`
// fn hit_sphere(center: Vec3, radius: f64, r: &Ray) -> f64 {
//     let oc = r.orig - center;
//     let a = r.dir.length_squared();
//     let half_b = oc.dot(&r.dir);
//     let c = oc.length_squared() - (radius * radius);
//     let discriminant = (half_b * half_b) - (a * c);
//     if discriminant < 0.0 {
//         -1.0
//     } else {
//         (-half_b - discriminant.sqrt()) / a
//     }
// }

// fn degrees_to_radians(degrees: f64) -> f64 {
//     (degrees * PI) / 180.0
// }
