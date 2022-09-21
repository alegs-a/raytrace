use raytrace::ray::Ray;
use raytrace::vec3d::*;
use raytrace::image::*;

fn main() {
    // let test = Image::test(1920, 1080);
    // test.print("print_output.ppm");

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3d { x: 0.0, y: 0.0, z: 0.0 };
    let horizontal = Vec3d { x: viewport_width, y: 0.0, z: 0.0 };
    let vertical = Vec3d { x: 0.0, y: viewport_height, z: 0.0 };
    let lower_left_corner = origin - horizontal.scalarp(0.5) - vertical.scalarp(0.5) + Vec3d { x: 0.0, y: 0.0, z: focal_length };

    // Render
    ppm_prelude(image_width, image_height, 255);
    for y in (0..image_height).rev() {
        // eprintln!("Lines remaining: {}", y);
        for x in 0..image_width {
            let u = x as f64 / (image_width - 1) as f64;
            let v = y as f64 / (image_height - 1) as f64;
            let r = Ray {
                orig: origin,
                dir: lower_left_corner + horizontal.scalarp(u) + vertical.scalarp(v) - origin,
            };
            eprintln!("Pixel: {x}, {y}");
            let pixel_colour = ray_colour(r);
            eprintln!("Colour: {pixel_colour}\n");
            println!("{}", pixel_colour.write());
        }
    }
    eprintln!("Done. Created a {image_width}x{image_height} pixel image.");

}


/// Calulates the background colour for a given ray, if it does not intersect any other geometry.
fn ray_colour(ray: Ray) -> Colour {
    let unit_direction = ray.dir.unit_vector();
    eprintln!("Ray: {ray}\nNormalised direction: {unit_direction}");
    let t = 0.5 * (unit_direction.y + 1.0);
    Colour { r: 1.0, g: 1.0, b: 1.0} * (1.0 - t) + Colour { r: 0.0, g: 0.7, b: 1.0 } * t
}

/// Prints the header for a .ppm file to stdout.
///
///
fn ppm_prelude(width: i32, height: i32, max_value: i32) {
    println!("P3");
    println!("{width} {height}");
    println!("{max_value}");
}
