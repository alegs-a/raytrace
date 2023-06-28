//! Store and render the scene's geometry.
use crate::camera::Camera;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::io::prelude::*;

use crate::colour::Colour;

use rand::prelude::*;

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
    pub fn render<W: Write>(
        &self,
        writer: &mut std::io::BufWriter<W>,
        samples_per_pixel: i32,
        max_depth: i32
    ) -> std::io::Result<()> {
        // Image
        // let aspect_ratio = self.image_width as f64 / self.image_height as f64;

        // Camera
        let cam = Camera::new(self.image_width, self.image_height);

        // Render!
        self.write_preamble(writer)?;

        let mut rng = rand::thread_rng();

        for j in (0..self.image_height).rev() {
            eprintln!("Lines remaining: {j}");
            for i in 0..self.image_width {
                let mut colour = Colour::black();
                for _s in 0..samples_per_pixel {
                    let u = (i as f64 + rng.gen::<f64>()) / (self.image_width - 1) as f64;
                    let v = (j as f64 + rng.gen::<f64>()) / (self.image_height - 1) as f64;

                    let ray = cam.get_ray(u, v);
                    colour += self.ray_colour(ray, max_depth);
                }
                colour.write(writer, samples_per_pixel)?;
            }
        }
        Ok(())
    }

    /// Return the colour of the geometry a ray hits, or the background colour if it does not hit
    /// any geometry, or black if it exceeds the maximum depth.
    pub fn ray_colour(&self, ray: Ray, depth: i32) -> Colour {
        // If we've exceeded the bounce limit no light is returned
        if depth <= 0 {
            return Colour::black();
        }

        if let Some(hit_record) = self.world.hit(ray, 0.0, f64::INFINITY) {
            let target = hit_record.point + hit_record.normal + Vec3::random_in_unit_sphere();
            return 0.5
                * self.ray_colour(
                    Ray::new(hit_record.point, target - hit_record.point),
                    depth - 1,
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
                colour.write(writer, 1)?; // 1 sample per pixel
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
