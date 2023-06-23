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
    #[arg(short, long)]
    samples_per_pixel: Option<i32>,
}

fn main() {
    let args = Args::parse();
    let output_path = args.output;
    let output_path = Path::new(&output_path);
    let output_file = File::create(output_path).expect("Unable to open file");
    let mut writer = std::io::BufWriter::new(output_file);

    let samples_per_pixel = args.samples_per_pixel.unwrap_or(100);

    let mut scene = Scene::new(400, 200);
    scene.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    scene.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, 0.0), 100.0)));
    scene.render(&mut writer, samples_per_pixel).expect("Failed to write to file.");
}
