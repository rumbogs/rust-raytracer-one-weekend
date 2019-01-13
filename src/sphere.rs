use super::hitable;
use super::vector3;
use super::ray;

struct Sphere {
  center: vector3::Vector3,
  radius: f32,
};

impl Sphere {
  pub fn new(center: vector3::Vector3, radius: f32) -> Sphere {
    Sphere {
      center,
      radius
    }
  }
}

impl hitable::Hitable for Sphere {
  fn hit(r: &ray::Ray, t_min: f32, t_max: f32, rec: &hitable::hit_record) {
    let oc: vector3::Vector3 = r.origin() - center;
    let a: f32 = vector3::dot(r.direction(), r.direction());
    let b: f32 = vector3::dot(oc, r.direction());
    let c: f32 = vector3::dot(oc, oc) - radius * radius;
    let discriminant: f32 = b * b - a * c;
    if discriminant > 0.0 {
      let mut temp: f32 = (-b - (b * b - a * c).sqrt()) / 2.0;
      if temp < t_max && temp > t_min {
        rec.t = temp;
        rec.p = r.point_at_parameter(rec.t);
        rec.normal = (rec.p - center) / radius;
        return true;
      }
      temp = (-b + (b * b - a * c).sqrt()) / a;
      if (temp < t_max && temp > t_min) {
        rec.t = temp;
        rec.p = r.point_at_parameter(rec.t);
        rec.normal = (rec.p - center) / radius;
        return true;
      }
    }
  }
  false
}