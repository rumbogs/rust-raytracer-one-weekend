use std::ops::{
  Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

#[derive(Copy, Clone)]
pub struct Vector3 {
  pub e: [f32; 3],
}

impl Vector3 {
  pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
    Vector3 { e: [x, y, z] }
  }
  pub fn x(&self) -> f32 {
    self.e[0]
  }
  pub fn y(&self) -> f32 {
    self.e[1]
  }
  pub fn z(&self) -> f32 {
    self.e[2]
  }
  pub fn r(&self) -> f32 {
    self.e[0]
  }
  pub fn g(&self) -> f32 {
    self.e[1]
  }
  pub fn b(&self) -> f32 {
    self.e[2]
  }

  pub fn length(&self) -> f32 {
    (self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]).sqrt()
  }

  pub fn squared_length(&self) -> f32 {
    self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
  }

  fn make_unit_vector(&mut self) {
    let k: f32 = 1.0 / (self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]);
    self.e[0] *= k;
    self.e[1] *= k;
    self.e[2] *= k;
  }
}

impl Add for Vector3 {
  type Output = Vector3;

  fn add(self, other: Vector3) -> Vector3 {
    Vector3::new(
      self.e[0] + other.e[0],
      self.e[1] + other.e[1],
      self.e[2] + other.e[2],
    )
  }
}

impl Sub for Vector3 {
  type Output = Vector3;

  fn sub(self, other: Vector3) -> Vector3 {
    Vector3::new(
      self.e[0] - other.e[0],
      self.e[1] - other.e[1],
      self.e[2] - other.e[2],
    )
  }
}

impl Mul for Vector3 {
  type Output = Vector3;

  fn mul(self, other: Vector3) -> Vector3 {
    Vector3::new(
      self.e[0] * other.e[0],
      self.e[1] * other.e[1],
      self.e[2] * other.e[2],
    )
  }
}

impl Div for Vector3 {
  type Output = Vector3;

  fn div(self, other: Vector3) -> Vector3 {
    Vector3::new(
      self.e[0] / other.e[0],
      self.e[1] / other.e[1],
      self.e[2] / other.e[2],
    )
  }
}

impl Mul<f32> for Vector3 {
  type Output = Vector3;

  fn mul(self, other: f32) -> Vector3 {
    Vector3::new(self.e[0] * other, self.e[1] * other, self.e[2] * other)
  }
}

impl Mul<Vector3> for f32 {
  type Output = Vector3;

  fn mul(self, other: Vector3) -> Vector3 {
    Vector3::new(self * other.e[0], self * other.e[1], self * other.e[2])
  }
}

impl Div<f32> for Vector3 {
  type Output = Vector3;

  fn div(self, other: f32) -> Vector3 {
    Vector3::new(self.e[0] / other, self.e[1] / other, self.e[2] / other)
  }
}

impl Neg for Vector3 {
  type Output = Vector3;

  fn neg(self) -> Vector3 {
    Vector3::new(-self.e[0], -self.e[1], -self.e[2])
  }
}

impl Index<usize> for Vector3 {
  type Output = f32;

  fn index(&self, i: usize) -> &f32 {
    &self.e[i]
  }
}

impl IndexMut<usize> for Vector3 {
  fn index_mut(&mut self, i: usize) -> &mut f32 {
    &mut self.e[i]
  }
}

impl AddAssign for Vector3 {
  fn add_assign(&mut self, other: Vector3) {
    self.e[0] += other.e[0];
    self.e[1] += other.e[1];
    self.e[2] += other.e[2];
  }
}

impl SubAssign for Vector3 {
  fn sub_assign(&mut self, other: Vector3) {
    self.e[0] -= other.e[0];
    self.e[1] -= other.e[1];
    self.e[2] -= other.e[2];
  }
}

impl MulAssign for Vector3 {
  fn mul_assign(&mut self, other: Vector3) {
    self.e[0] *= other.e[0];
    self.e[1] *= other.e[1];
    self.e[2] *= other.e[2];
  }
}

impl MulAssign<f32> for Vector3 {
  fn mul_assign(&mut self, other: f32) {
    self.e[0] *= other;
    self.e[1] *= other;
    self.e[2] *= other;
  }
}

impl DivAssign for Vector3 {
  fn div_assign(&mut self, other: Vector3) {
    self.e[0] /= other.e[0];
    self.e[1] /= other.e[1];
    self.e[2] /= other.e[2];
  }
}

impl DivAssign<f32> for Vector3 {
  fn div_assign(&mut self, other: f32) {
    let k = 1.0 / other;

    self.e[0] *= k;
    self.e[1] *= k;
    self.e[2] *= k;
  }
}

impl std::fmt::Display for Vector3 {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "(x: {}, y: {}, z: {})", self.e[0], self.e[1], self.e[2])
  }
}

pub fn dot(v1: Vector3, v2: Vector3) -> f32 {
  v1.e[0] * v2.e[0] + v1.e[1] * v2.e[1] + v1.e[2] * v2.e[2]
}

pub fn cross(v1: Vector3, v2: Vector3) -> Vector3 {
  Vector3::new(
    v1.e[1] * v2.e[2] - v1.e[2] * v2.e[1],
    -(v1.e[0] * v2.e[2] - v1.e[2] * v2.e[0]),
    v1.e[0] * v2.e[1] - v1.e[1] * v2.e[0],
  )
}

pub fn unit_vector(v: Vector3) -> Vector3 {
  let length = v.length();
  v / length
}
