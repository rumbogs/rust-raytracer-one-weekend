use super::vector3;
use super::ray;

pub struct hit_record {
  t: f32,
  p: vector3::Vector3,
  normal: vector3::Vector3
}

trait Hitable {
    pub fn hit(r: ray::Ray, t_min: f32, t_max: f32, rec: hit_record) -> bool;
}
