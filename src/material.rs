use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{Color, Vector3};
use crate::Num;
use rand::Rng;

#[derive(Clone, Copy)]
pub enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz: Num },
    Dielectric { ir: Num },
}

impl Material {
    pub fn scatter<R: Rng>(&self, ray: Ray, rec: HitRecord, rng: &mut R) -> Option<(Color, Ray)> {
        match *self {
            Material::Lambertian { albedo } => {
                let mut scatter_dir = rec.normal + Vector3::random_unit_vector(rng);
                if scatter_dir.near_zero() {
                    scatter_dir = rec.normal;
                }

                let scattered = Ray::from(rec.p, scatter_dir);
                Some((albedo, scattered))
            }
            Material::Metal { albedo, fuzz } => {
                let reflected = ray.direction.normalize().reflect(rec.normal);
                let scattered = Ray::from(
                    rec.p,
                    reflected + fuzz * Vector3::random_in_unit_sphere(rng),
                );

                if scattered.direction.dot(rec.normal) > 0. {
                    Some((albedo, scattered))
                } else {
                    None
                }
            }
            Material::Dielectric { ir } => {
                let attenuation = Color::from_elem(1.);
                let refraction_ratio = if rec.front_face { 1. / ir } else { ir };

                let unit_direction = ray.direction.normalize();

                let cos_theta = (-unit_direction).dot(rec.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

                // Schlick's approximation
                let reflectance = {
                    let r0 = (1. - refraction_ratio) / (1. + refraction_ratio);
                    let r0 = r0 * r0;
                    r0 + (1. - r0) * ((1. - cos_theta).powi(5))
                };
                let direction =
                    if refraction_ratio * sin_theta > 1. || reflectance > rng.gen::<Num>() {
                        unit_direction.reflect(rec.normal)
                    } else {
                        unit_direction.refract(rec.normal, refraction_ratio)
                    };

                let scattered = Ray::from(rec.p, direction);
                Some((attenuation, scattered))
            }
        }
    }
}
