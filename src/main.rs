use raytrace::Vec3d;

fn main() {
    let vector = Vec3d { x: 1.0, y: 2.0, z: 3.0 };
    let vector2 = Vec3d::new();

    println!("A vector with components {}, {}, {}", vector.x, vector.y, vector.z);
    println!("A vector with components {}, {}, {}", vector2.x, vector2.y, vector2.z);
}
