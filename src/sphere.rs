//! A Sphere is a hittable object that can be rendered in the scene.
use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug)]
/// A sphere.
pub struct Sphere {
    /// The center of the sphere
    center: Vec3,
    /// The radius of the sphere
    radius: f64,
}

impl Sphere {
    /// Create a new `Sphere`
    pub fn new(center: Vec3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.orig - self.center;
        let a = ray.dir.length_squared();
        let half_b = oc.dot(ray.dir);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let point = ray.at(root);
        let outward_normal = (1.0 / self.radius) * (point - self.center);
        Some(HitRecord::from_ray(ray, point, outward_normal, root))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hit() {
        let sphere = Sphere::new(Vec3::new(3.0, 0.0, 0.0), 1.0);
        let ray = Ray::new(Vec3::zeros(), Vec3::new(1.0, 0.0, 0.0));
        let expected = Some(HitRecord {
            point: Vec3::new(2.0, 0.0, 0.0),
            normal: Vec3::new(-1.0, 0.0, 0.0),
            t: 2.0,
            front_face: true,
        });

        assert_eq!(sphere.hit(ray, 0.0, f64::MAX), expected);
    }

    #[test]
    fn test_hit_edge() {
        let sphere = Sphere::new(Vec3::new(3.0, 0.0, 0.0), 1.0);

        let ray = Ray::new(Vec3::new(0.0, 1.0, 0.0), Vec3::new(1.0, 0.0, 0.0));
        let expected = Some(HitRecord {
            point: Vec3::new(3.0, 1.0, 0.0),
            normal: Vec3::new(0.0, -1.0, 0.0),
            t: 3.0,
            front_face: false,
        });

        assert_eq!(sphere.hit(ray, 0.0, f64::MAX), expected);
    }

    #[test]
    fn test_hit_missing() {
        let sphere = Sphere::new(Vec3::new(3.0, 0.0, 0.0), 1.0);
        let ray = Ray::new(Vec3::zeros(), Vec3::new(0.0, 0.0, 1.0));

        assert_eq!(sphere.hit(ray, 0.0, f64::MAX), None);
    }
}
