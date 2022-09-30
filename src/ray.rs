use super::vec3::Vec3;

/// A vector with an origin
///
/// A ray is an object that defines an origin point and then a direction from that origin. Both are
/// internally represented as a Vec3, in its capacity as simply describing a point in 3D space.
pub struct Ray {
    /// The vector describing the origin of the ray
    pub orig: Vec3,
    /// The vector describing the direction of the ray
    pub dir: Vec3,
}

impl Ray {
    pub fn new(orig: Vec3, dir: Vec3) -> Ray {
        Ray { orig, dir }
    }

    /// The point that is origin + (t * dir)
    pub fn at(&self, t: f64) -> Vec3 {
        self.orig + (self.dir * t)
    }
}
