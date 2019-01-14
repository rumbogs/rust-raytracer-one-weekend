use super::vector3::Vector3;
use super::ray::Ray;

pub struct Camera {
  origin: Vector3,
  lower_left_corner: Vector3,
  horizontal: Vector3,
  vertical: Vector3
}

impl Camera {
  pub fn new(origin: Vector3,
  lower_left_corner: Vector3,
  horizontal: Vector3,
  vertical: Vector3) -> Camera {
    Camera { origin, lower_left_corner, horizontal, vertical }
  }

  pub fn get_ray(&self, u: f32, v: f32) -> Ray {
    Ray::new(self.origin, self.lower_left_corner + u * self.horizontal + v * self.vertical)
  }
}