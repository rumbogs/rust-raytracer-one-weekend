use super::vector3::{cross, unit_vector, Vector3};

pub struct ONB {
    axis: [Vector3; 3],
}

impl ONB {
    pub fn new(n: Vector3) -> Self {
        let w: Vector3 = unit_vector(n);
        let a: Vector3;
        if w.x().abs() > 0.9 {
            a = Vector3::new(0.0, 1.0, 0.0);
        } else {
            a = Vector3::new(1.0, 0.0, 0.0);
        }
        let v = unit_vector(cross(w, a));
        let u = cross(w, v);
        ONB { axis: [u, v, w] }
    }
    pub fn u(&self) -> Vector3 {
        self.axis[0]
    }
    pub fn v(&self) -> Vector3 {
        self.axis[1]
    }
    pub fn w(&self) -> Vector3 {
        self.axis[2]
    }
    pub fn local(&self, a: f32, b: f32, c: f32) -> Vector3 {
        a * self.u() + b * self.v() + c * self.w()
    }
    pub fn local_vec(&self, a: &Vector3) -> Vector3 {
        a.x() * self.u() + a.y() * self.v() + a.z() * self.w()
    }
}
