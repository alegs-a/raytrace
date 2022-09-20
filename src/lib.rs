/// Holds a 3-dimensional vector and related operations.
pub struct Vec3d {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vec3d {
    /// Initialises a new instance of Vec3d with unit length components (i.e. (1, 1, 1)).
    pub fn new() -> Vec3d {
        let x: f64 = 1.0;
        let y: f64 = 1.0;
        let z: f64 = 1.0;

        Vec3d { x, y, z }
    }
}
