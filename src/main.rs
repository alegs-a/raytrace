use raytrace::scene::Scene;
use raytrace::sphere::Sphere;
use raytrace::vec3::Vec3;

use std::fs::File;
use std::path::Path;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Optional file to output to
    #[arg(short, long, default_value = "output.ppm")]
    output: String,
    /// Optional value for samples per pixel
    #[arg(short, long, default_value_t = 100)]
    samples_per_pixel: i32,
    /// Optional maximum number of bounces
    #[arg(short, long, default_value_t = 50)]
    bounce_depth: i32,
}

fn main() {
    let args = Args::parse();
    let output_path = args.output;
    let output_path = Path::new(&output_path);
    let output_file = File::create(output_path).expect("Unable to open file");
    let mut writer = std::io::BufWriter::new(output_file);

<<<<<<< HEAD
    let mut scene = Scene::new(400, 225);
    scene.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    scene.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, 0.0), 100.0)));
    scene
        .render(&mut writer, args.samples_per_pixel, args.bounce_depth)
        .expect("Failed to write to file.");
=======
    let mut scene = Scene::new(400, 200);
    scene.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    scene.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, 0.0), 100.0)));
    scene.render(&mut writer).expect("Failed to write to file.");
>>>>>>> parent of 3f3969a (Finish chapter 7 (antialiasing))
}
