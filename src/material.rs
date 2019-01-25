use super::object::HitRecord;
use super::random_in_unit_sphere;
use super::ray::Ray;
use super::vector3::{dot, unit_vector, Vector3};

fn reflect(v: Vector3, n: Vector3) -> Vector3 {
  v - 2.0 * dot(v, n) * n
}

pub enum MaterialType {
  Lambertian,
  Metal,
}

pub struct Material {
  kind: MaterialType,
  albedo: Vector3,
}

impl Material {
  pub fn new(kind: MaterialType, albedo: Vector3) -> Self {
    Material { kind, albedo }
  }
}

pub trait Scatterable {
  fn scatter(&self, r_in: &Ray, rec: HitRecord) -> Option<(Vector3, Ray)>;
}

impl Scatterable for Material {
  fn scatter(&self, r_in: &Ray, rec: HitRecord) -> Option<(Vector3, Ray)> {
    match self.kind {
      MaterialType::Lambertian => {
        let target: Vector3 = rec.p + rec.normal + random_in_unit_sphere();
        Some((self.albedo, Ray::new(rec.p, target - rec.p)))
      }
      MaterialType::Metal => {
        let reflected = reflect(unit_vector(r_in.direction()), rec.normal);
        let scattered = Ray::new(rec.p, reflected);
        if dot(scattered.direction(), rec.normal) > 0.0 {
          Some((self.albedo, scattered))
        } else {
          None
        }
      }
    }
  }
}
