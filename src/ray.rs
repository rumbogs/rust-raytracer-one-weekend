use super::vector3;

pub struct Ray {
  pub a: vector3::Vector3,
  pub b: vector3::Vector3,
}

impl Ray {
  pub fn new(a: vector3::Vector3, b: vector3::Vector3) -> Ray {
    Ray { a, b }
  }
  pub fn origin(&self) -> vector3::Vector3 {
    vector3::Vector3::new(self.a.e[0], self.a.e[1], self.a.e[2])
  }
  pub fn direction(&self) -> vector3::Vector3 {
    vector3::Vector3::new(self.b.e[0], self.b.e[1], self.b.e[2])
  }
  pub fn point_at_parameter(&self, t: f32) -> vector3::Vector3 {
    self.a + t * self.b
  }
}
