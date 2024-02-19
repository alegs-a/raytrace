mod math;

use std::{fs::File, io::BufWriter, path::Path};
use math::vec3::Vec3;

fn main() -> std::io::Result<()> {
    let path = Path::new("output.ppm");
    let file = File::create(path)?;


    Ok(())
}
