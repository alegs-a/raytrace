//! How we store all of the objects for the scene.

use crate::hittable::{HitRecord, Hittable};

/// A list of hittable objects
#[derive(Default)]
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    /// Create a new `HittableList`
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }
    /// Add a new object to the list of objects.
    pub fn add(&mut self, new: Box<dyn Hittable>) {
        self.objects.push(new);
    }

    /// Clear the list of objects.
    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_record = None;
        let mut closest_so_far = t_max;
        for object in &self.objects {
            if let Some(temp_hit_record) = object.hit(ray, t_min, closest_so_far) {
                hit_record = Some(temp_hit_record);
                closest_so_far = temp_hit_record.t;
            }
        }
        hit_record
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ray::Ray, sphere::Sphere, vec3::Vec3};

    #[test]
    fn test_hit() {
        let hittable_list = HittableList {
            objects: vec![Box::new(Sphere::new(Vec3::new(3.0, 0.0, 0.0), 1.0))],
        };
        let ray = Ray::new(Vec3::zeros(), Vec3::new(1.0, 0.0, 0.0));

        let expected = Some(HitRecord {
            point: Vec3::new(2.0, 0.0, 0.0),
            normal: Vec3::new(-1.0, 0.0, 0.0),
            t: 2.0,
            front_face: true,
        });

        assert_eq!(hittable_list.hit(ray, 0.0, f64::MAX), expected);
    }

    #[test]
    fn test_hit_missing() {
        let hittable_list = HittableList {
            objects: vec![Box::new(Sphere::new(Vec3::new(3.0, 0.0, 0.0), 1.0))],
        };
        let ray = Ray::new(Vec3::zeros(), Vec3::new(0.0, 1.0, 0.0));

        assert_eq!(hittable_list.hit(ray, 0.0, f64::MAX), None);
    }
}
