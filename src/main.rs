use raytrace::color::Color;
use raytrace::hittable::{HitRecord, Hittable};
use raytrace::hittable_list::HittableList;
use raytrace::ray::Ray;
use raytrace::sphere::Sphere;
use raytrace::vec3::Vec3;

const INFINITY: f64 = std::f64::INFINITY;
const PI: f64 = std::f64::consts::PI;

fn main() {

    // Image

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    // World

    let mut world = HittableList::default();
    world.add(Box::new(Sphere {
        center: Vec3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
    }));
    world.add(Box::new(Sphere {
        center: Vec3 {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
        radius: 100.0,
    }));


    // Camera

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let horizontal = Vec3 {
        x: viewport_width,
        y: 0.0,
        z: 0.0,
    };
    let vertical = Vec3 {
        x: 0.0,
        y: viewport_height,
        z: 0.0,
    };
    let lower_left_corner = origin
        - (horizontal / 2.0)
        - (vertical / 2.0)
        - Vec3 {
            x: 0.0,
            y: 0.0,
            z: focal_length,
        };

    // Render

    println!("P3\n{image_width} {image_height}\n255"); // .ppm header

    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {j}"); // progress indicator
        for i in 0..image_width {
            let u = (i as f64) / ((image_width - 1) as f64);
            let v = (j as f64) / ((image_height - 1) as f64);
            let r = Ray {
                orig: origin,
                dir: lower_left_corner + (horizontal * u) + (vertical * v) - origin,
            };
            let pixel_color = ray_color(&r, &world);
            Color::write_color(pixel_color);
        }
    }
}

/// Calculate the color of a ray, including the case where it intersects geometry
fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    let mut rec = HitRecord::default();
    if world.hit(r, 0.0, INFINITY, &mut rec) {
        return (rec.normal + Color {
            // White (1, 1, 1)
            r: 1.0,
            g: 1.0,
            b: 1.0,
        }) * 0.5;
    }
    let unit_direction: Vec3 = r.dir.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    (Color {
        // White (1, 1, 1)
        r: 1.0,
        g: 1.0,
        b: 1.0,
    } * (1.0 - t))
        + (Color {
            // Bluish (0.5, 0.7, 1)
            r: 0.5,
            g: 0.7,
            b: 1.0,
        } * t)
}

/// Calculate the solutions to `r.at(t)` for all `t` that makes the ray point to the surface of the
/// sphere defined by `center` and `radius`
fn hit_sphere(center: Vec3, radius: f64, r: &Ray) -> f64 {
    let oc = r.orig - center;
    let a = r.dir.length_squared();
    let half_b = oc.dot(&r.dir);
    let c = oc.length_squared() - (radius * radius);
    let discriminant = (half_b * half_b) - (a * c);
    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

fn degrees_to_radians(degrees: f64) -> f64 {
    (degrees * PI) / 180.0
}
