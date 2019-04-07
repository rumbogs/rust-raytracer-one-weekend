use super::vector3::Vector3;

pub struct Ray {
    pub a: Vector3,
    pub b: Vector3,
    pub time: f32,
}

impl Ray {
    pub fn new(a: Vector3, b: Vector3, time: f32) -> Ray {
        Ray { a, b, time }
    }
    pub fn origin(&self) -> Vector3 {
        Vector3::new(self.a.e[0], self.a.e[1], self.a.e[2])
    }
    pub fn direction(&self) -> Vector3 {
        Vector3::new(self.b.e[0], self.b.e[1], self.b.e[2])
    }
    pub fn point_at_parameter(&self, t: f32) -> Vector3 {
        self.a + t * self.b
    }
}
