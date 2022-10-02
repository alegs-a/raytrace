use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Clone, Copy)]
pub enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz: f64 },
}

impl Default for Material {
    fn default() -> Self {
        Material::Lambertian {
            albedo: Color::black(),
        }
    }
}

impl Material {
    pub fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        match self {
            Material::Lambertian { albedo } => {
                Self::scatter_lambertian(albedo, hit_record, attenuation, scattered)
            }
            Material::Metal { albedo, fuzz } => {
                Self::scatter_metal(albedo, ray, hit_record, attenuation, scattered, fuzz)
            }
        }
    }

    // Deleted the parameter `ray: &Ray` from this function cos Clippy told me to
    fn scatter_lambertian(
        albedo: &Color,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = *albedo;
        true
    }

    fn scatter_metal(
        albedo: &Color,
        ray: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        fuzz: &f64,
    ) -> bool {
        let reflected = ray.dir.unit_vector().reflect(&rec.normal);
        *scattered = Ray::new(rec.p, reflected + Vec3::random_unit_vector() * *fuzz);
        *attenuation = *albedo;
        scattered.dir.dot(&rec.normal) > 0.0
    }
}
