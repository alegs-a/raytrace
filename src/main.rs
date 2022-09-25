use raytrace::vec3::Vec3;
use raytrace::color::Color;

fn main() {
    // Image

    let image_width = 256;
    let image_height = 256;

    // Render

    println!("P3\n{image_width} {image_height}\n255");

    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {j}");
        for i in 0..image_width {
            // let r = (i as f64) / ((image_width - 1) as f64);
            // let g = (j as f64) / ((image_height - 1) as f64);
            // let b = 0.25;

            // let r = (255.999 * r) as u8;
            // let g = (255.999 * g) as u8;
            // let b = (255.999 * b) as u8;
            
            let color = Color {
                r: ((i as f64) / (image_width - 1) as f64),
                g: ((j as f64) / (image_height - 1) as f64),
                b: 0.25,
            };
            Color::write_color(color);
        }
    }
}
