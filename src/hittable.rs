//! Manipulate hittable things
use crate::ray::Ray;
use crate::vec3::Vec3;

/// Determine whether a ray hits `self`.
pub trait Hittable {
    /// Determine whether a ray hits `self`.
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

/// A data-carrier for a hit)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HitRecord {
    /// The point at which the ray hits the object
    pub point: Vec3,
    /// The surface normal at the hit point
    pub normal: Vec3,
    /// The parameter at which the ray hits the object
    pub t: f64,
    /// Whether the ray hit the front of the face
    pub front_face: bool,
}

impl HitRecord {
    /// Create a new HitRecord.
    ///
    /// Automatically determines whether the hit is on the front or back of the face.
    pub fn from_ray(ray: Ray, point: Vec3, outward_normal: Vec3, t: f64) -> HitRecord {
        let front_face: bool = ray.dir.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        HitRecord {
            point,
            normal,
            t,
            front_face,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_ray() {
        let ray = Ray::new(Vec3::zeros(), Vec3::new(1.0, 0.0, 0.0));
        let point = Vec3::new(2.0, 0.0, 0.0);
        let outward_normal = Vec3::new(-1.0, 0.0, 0.0);
        let t = 2.0;

        let expected = HitRecord {
            point,
            normal: outward_normal,
            t,
            front_face: true,
        };
        assert_eq!(HitRecord::from_ray(ray, point, outward_normal, t), expected);
    }

    #[test]
    fn test_from_ray_back_face() {
        let ray = Ray::new(Vec3::zeros(), Vec3::new(1.0, 0.0, 0.0));
        let point = Vec3::new(2.0, 0.0, 0.0);
        let outward_normal = Vec3::new(1.0, 0.0, 0.0);
        let t = 2.0;

        let expected = HitRecord {
            point,
            normal: -outward_normal,
            t,
            front_face: false,
        };
        assert_eq!(HitRecord::from_ray(ray, point, outward_normal, t), expected);
    }
}
