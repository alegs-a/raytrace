//! Store and manipulate rays.
//!
//! A ray has an origin point and a direction vector.
use crate::colour::Colour;
use crate::vec3::Vec3;

/// A ray in 3D space.
#[derive(Debug, Clone, Copy)]
pub struct Ray {
    /// The origin of the ray.
    pub orig: Vec3,
    /// The direction of the ray.
    pub dir: Vec3,
}

impl Ray {
    /// Create a new `Ray`.
    pub fn new(orig: Vec3, dir: Vec3) -> Ray {
        Ray { orig, dir }
    }
    /// Compute the point obtained from travelling `t` multiples of `dir` from `orig`.
    pub fn at(&self, t: f64) -> Vec3 {
        self.orig + t * self.dir
    }

    /// Return the background colour for a given ray, ignoring the scene's geometry.
    ///
    /// The background produced is a nice sky-blue with a subtle gradient towards white - just like
    /// a real sky!
    pub fn bg_colour(&self) -> Colour {
        let unit_dir = self.dir.unit();
        let t = 0.5 * (unit_dir.y + 1.0);
        (1.0 - t) * Colour::new(1.0, 1.0, 1.0) + t * Colour::new(0.5, 0.7, 1.0)
    }

    /// Determine whether `self` hits the given sphere, and return the multiple of `ray` at which
    /// it first intersects the sphere, or `None` if it does not hit the sphere.
    ///
    /// TODO: Explain how this works, or at least link to the book
    #[deprecated]
    pub fn hit_sphere(&self, center: Vec3, radius: f64) -> Option<f64> {
        let oc = self.orig - center;
        let a = self.dir.length_squared();
        let b = 2.0 * oc.dot(self.dir);
        let c = oc.dot(oc) - radius * radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            None
        } else {
            Some((-b - discriminant.sqrt()) / (2.0 * a))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{hittable::Hittable, sphere::Sphere};

    use super::*;

    #[test]
    fn test_at() {
        let ray = Ray::new(Vec3::zeros(), Vec3::new(1.0, 0.0, 0.0));
        assert_eq!(ray.at(2.0), Vec3::new(2.0, 0.0, 0.0));
    }

    #[test]
    fn test_hit_sphere() {
        let center = Vec3::new(1.0, 0.0, 0.0);
        let radius = 1.0;
        let sphere = Sphere::new(center, radius);
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0));
        assert!(sphere.hit(ray, 0.0, f64::INFINITY).is_some());
        let ray = Ray::new(Vec3::new(1.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0));
        assert!(sphere.hit(ray, 0.0, f64::INFINITY).is_some());
    }

    #[test]
    fn test_miss_sphere() {
        let center = Vec3::new(2.0, 0.0, 0.0);
        let radius = 1.0;
        let sphere = Sphere::new(center, radius);
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
        assert!(sphere.hit(ray, 0.0, f64::INFINITY).is_none());
    }
}
