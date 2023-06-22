//! Store and render the scene's geometry.
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::io::prelude::*;

use super::colour::Colour;

/// A scene for rendering.
///
/// Will eventually include geometry etc. once implemented.
pub struct Scene {
    /// width of rendered image
    image_width: i32,
    /// height of rendered image
    image_height: i32,
    /// The scene's geometry
    world: HittableList,
}

impl Scene {
    /// Create a new Scene with the given dimensions and an empty world
    pub fn new(image_width: i32, image_height: i32) -> Scene {
        Scene {
            image_width,
            image_height,
            world: HittableList::new(),
        }
    }

    /// Add an object to the world.
    pub fn add(&mut self, new: Box<dyn Hittable>) {
        self.world.add(new);
    }

    /// Render the scene.
    ///
    /// The output is written to `writer`.
    pub fn render<W: Write>(&self, writer: &mut std::io::BufWriter<W>) -> std::io::Result<()> {
        // Image
        let aspect_ratio = self.image_width as f64 / self.image_height as f64;

        // Camera
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Vec3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - 0.5 * horizontal - 0.5 * vertical - Vec3::new(0.0, 0.0, focal_length);

        // Render!
        self.write_preamble(writer)?;

        for j in (0..self.image_height).rev() {
            for i in 0..self.image_width {
                let u = i as f64 / (self.image_width - 1) as f64;
                let v = j as f64 / (self.image_height - 1) as f64;

                let ray = Ray::new(
                    origin,
                    lower_left_corner + (u * horizontal) + (v * vertical) - origin,
                );
                let colour = self.ray_colour(ray);
                colour.write(writer)?;
            }
        }
        Ok(())
    }

    /// Return the colour of the geometry a ray hits, or the background colour if it does not hit
    /// any geometry.
    pub fn ray_colour(&self, ray: Ray) -> Colour {
        if let Some(hit_record) = self.world.hit(ray, 0.0, f64::INFINITY) {
            return 0.5
                * Colour::new(
                    hit_record.normal.x + 1.0,
                    hit_record.normal.y + 1.0,
                    hit_record.normal.z + 1.0,
                );
        }
        ray.bg_colour()
    }

    /// Generate a simple test image.
    #[allow(dead_code)]
    pub fn test_output<W: Write>(&self, writer: &mut std::io::BufWriter<W>) -> std::io::Result<()> {
        eprintln!("Generating test image");
        self.write_preamble(writer)?;
        for y in 0..self.image_height {
            for x in 0..self.image_width {
                let r = y as f64 / self.image_height as f64;
                let g = x as f64 / self.image_width as f64;
                let b = 0f64;
                let colour = Colour::new(r, g, b);
                colour.write(writer)?;
            }
        }
        writer.flush()?;
        eprintln!("Done!");
        Ok(())
    }

    fn write_preamble<W: Write>(&self, writer: &mut std::io::BufWriter<W>) -> std::io::Result<()> {
        writeln!(writer, "P3")?;
        writeln!(writer, "{} {}", self.image_width, self.image_height)?;
        writeln!(writer, "255")?;

        Ok(())
    }
}