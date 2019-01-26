use super::object::HitRecord;
use super::random_in_unit_sphere;
use super::ray::Ray;
use super::vector3::{dot, unit_vector, Vector3};

fn reflect(v: Vector3, n: Vector3) -> Vector3 {
  v - 2.0 * dot(v, n) * n
}

fn refract(v: Vector3, n: Vector3, ni_over_nt: f32) -> Option<Vector3> {
  let uv: Vector3 = unit_vector(v);
  let dt: f32 = dot(uv, n);
  let discriminant: f32 = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
  if discriminant > 0.0 {
    Some(ni_over_nt * (uv - n * dt) - n * discriminant.sqrt())
  } else {
    None
  }
}

pub enum MaterialType {
  Lambertian,
  Metal,
  Dielectric,
}

pub struct Material {
  kind: MaterialType,
  albedo: Vector3,
  fuzz: f32,
  ref_idx: f32,
}

impl Material {
  pub fn new(kind: MaterialType, albedo: Vector3, f: f32, ref_idx: f32) -> Self {
    let mut fuzz = f;
    if f >= 1.0 {
      fuzz = 1.0;
    }
    Material {
      kind,
      albedo,
      fuzz,
      ref_idx,
    }
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
        let scattered = Ray::new(rec.p, reflected + self.fuzz * random_in_unit_sphere());
        if dot(scattered.direction(), rec.normal) > 0.0 {
          Some((self.albedo, scattered))
        } else {
          None
        }
      }
      MaterialType::Dielectric => {
        let outward_normal: Vector3;
        let reflected: Vector3 = reflect(r_in.direction(), rec.normal);
        let ni_over_nt: f32;
        // kill the blue color which is a color bug
        let attenuation = Vector3::new(1.0, 1.0, 0.0);
        let scattered: Ray;

        if dot(r_in.direction(), rec.normal) > 0.0 {
          outward_normal = -rec.normal;
          ni_over_nt = self.ref_idx;
        } else {
          outward_normal = rec.normal;
          ni_over_nt = 1.0 / self.ref_idx;
        }

        match refract(r_in.direction(), outward_normal, ni_over_nt) {
          Some(refracted) => {
            scattered = Ray::new(rec.p, reflected);
            Some((attenuation, scattered))
          }
          None => {
            scattered = Ray::new(rec.p, reflected);
            None
          }
        }
      }
    }
  }
}
