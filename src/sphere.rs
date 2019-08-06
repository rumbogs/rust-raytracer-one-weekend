use super::aabb::Aabb;
use super::material::Material;
use super::object::{HitRecord, Hittable};
use super::ray::Ray;
use super::vector3::{dot, Vector3};

pub struct Sphere {
    center: Vector3,
    radius: f32,
    material: Material,
}

impl Sphere {
    pub fn new(center: Vector3, radius: f32, material: Material) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<(HitRecord, &Material)> {
        let oc: Vector3 = r.origin() - self.center;
        let a: f32 = dot(r.direction(), r.direction());
        let b: f32 = dot(oc, r.direction());
        let c: f32 = dot(oc, oc) - self.radius * self.radius;
        let discriminant: f32 = b * b - a * c;
        if discriminant > 0.0 {
            let mut temp: f32 = (-b - (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let t = temp;
                let p = r.point_at_parameter(t);
                let normal = (p - self.center) / self.radius;
                return Some((HitRecord::new(t, p, normal), &self.material));
            }
            temp = (-b + (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let t = temp;
                let p = r.point_at_parameter(t);
                let normal = (p - self.center) / self.radius;
                return Some((HitRecord::new(t, p, normal), &self.material));
            }
        }
        None
    }
    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<Aabb> {
        Some(Aabb::new(
            self.center - Vector3::new(self.radius, self.radius, self.radius),
            self.center + Vector3::new(self.radius, self.radius, self.radius),
        ))
    }
}
