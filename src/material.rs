use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{Color, Vector3};
use rand::Rng;

#[derive(Clone, Copy)]
pub enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color },
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
            },
            Material::Metal { albedo } => {
                let reflected = ray.direction.normalize().reflect(rec.normal);
                let scattered = Ray::from(rec.p, reflected);

                if scattered.direction.dot(rec.normal) > 0. {
                    Some((albedo, scattered))
                }
                else {
                    None
                }
            }
        }
    }
}
