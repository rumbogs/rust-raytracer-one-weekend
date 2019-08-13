use std::f32::consts::PI;

use super::super::aabb::Aabb;
use super::super::hittable::{HitRecord, Hittable};
use super::super::material::Material;
use super::super::ray::Ray;
use super::super::vector3::{dot, Vector3};

pub struct Sphere {
    pub center: Vector3,
    pub radius: f32,
    pub material: Material,
}

impl Sphere {
    pub fn get_sphere_uv(&self, p: &Vector3) -> (f32, f32) {
        let phi: f32 = p.z().atan2(p.x());
        let theta: f32 = p.y().asin();
        (1.0 - (phi + PI) / (2.0 * PI), (theta + PI / 2.0) / PI)
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
                let (u, v) = self.get_sphere_uv(&((p - self.center) / self.radius));
                return Some((HitRecord::new(u, v, t, p, normal), &self.material));
            }
            temp = (-b + (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let t = temp;
                let p = r.point_at_parameter(t);
                let normal = (p - self.center) / self.radius;
                let (u, v) = self.get_sphere_uv(&((p - self.center) / self.radius));
                return Some((HitRecord::new(u, v, t, p, normal), &self.material));
            }
        }
        None
    }
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb> {
        Some(Aabb::new(
            self.center - Vector3::new(self.radius, self.radius, self.radius),
            self.center + Vector3::new(self.radius, self.radius, self.radius),
        ))
    }
}
