//! Handle the camera and associated functions/

use crate::{vec3::Vec3, ray::Ray};

/// A camera
pub struct Camera {
    /// The point which rays are cast from.
    origin: Vec3,
    /// The lower left corner of the viewport
    lower_left_corner: Vec3,
    /// The horizontal dimension of the viewport
    horizontal: Vec3,
    /// The vertical dimension of the viewport
    vertical: Vec3,
}

impl Camera {
    /// Create a new camera with the given resolution.
    pub fn new(image_width: i32, image_height: i32) -> Camera {
        let aspect_ratio = image_width as f64 / image_height as f64;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Vec3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - 0.5 * horizontal - 0.5 * vertical - Vec3::new(0.0, 0.0, focal_length);

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    /// Get the ray that is `u` across and `v` down in the viewport
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin)
    }
}
