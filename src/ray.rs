use crate::vec3d::Vec3d;

/// A ray.
/// Is mostly a wrapper to provide the .at() method, to avoid writing the origin and direction
/// vectors in code.
#[derive(Copy, Clone)]
pub struct Ray {
    /// The position vector describing the origin point of the ray
    pub orig: Vec3d,
    /// The vector describing the direction of the ray.
    pub dir: Vec3d,
}

impl Ray {
    pub fn at(&self, t: f64) -> Vec3d {
        self.orig + self.dir.scalarp(t)
    }
}

impl std::fmt::Display for Ray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Origin: {}, {}, {}\nDirection: {}, {}, {}",
            self.orig.x, self.orig.y, self.orig.z, self.dir.x, self.dir.y, self.dir.z
        )
    }
}
