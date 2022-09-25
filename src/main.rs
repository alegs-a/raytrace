use raytrace::color::Color;
use raytrace::ray::Ray;
use raytrace::vec3::Vec3;

fn main() {
    // Image

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

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
            let pixel_color = ray_color(r);
            Color::write_color(pixel_color);
        }
    }
}

fn ray_color(r: Ray) -> Color {
    let unit_direction: Vec3 = r.dir.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    (Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
    } * (1.0 - t))
        + (Color {
            r: 0.5,
            g: 0.7,
            b: 1.0,
        } * t)
}
