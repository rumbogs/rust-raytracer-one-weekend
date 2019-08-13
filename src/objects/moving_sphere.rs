use std::f32::consts::PI;

use super::super::aabb::{surrounding_box, Aabb};
use super::super::hittable::{HitRecord, Hittable};
use super::super::material::Material;
use super::super::ray::Ray;
use super::super::vector3::{dot, Vector3};

pub struct MovingSphere {
    pub center0: Vector3,
    pub center1: Vector3,
    pub time0: f32,
    pub time1: f32,
    pub radius: f32,
    pub material: Material,
}

impl MovingSphere {
    pub fn center_at_time(&self, time: f32) -> Vector3 {
        ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
            + self.center0
    }
    pub fn get_sphere_uv(&self, p: &Vector3) -> (f32, f32) {
        let phi: f32 = p.z().atan2(p.x());
        let theta: f32 = p.y().asin();
        (1.0 - (phi + PI) / (2.0 * PI), (theta + PI / 2.0) / PI)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<(HitRecord, &Material)> {
        let center = self.center_at_time(r.time);
        let oc: Vector3 = r.origin() - center;
        let a: f32 = dot(r.direction(), r.direction());
        let b: f32 = dot(oc, r.direction());
        let c: f32 = dot(oc, oc) - self.radius * self.radius;
        let discriminant: f32 = b * b - a * c;
        if discriminant > 0.0 {
            let mut temp: f32 = (-b - (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let t = temp;
                let p = r.point_at_parameter(t);
                let normal = (p - center) / self.radius;
                let (u, v) = self.get_sphere_uv(&((p - center) / self.radius));
                return Some((HitRecord::new(u, v, t, p, normal), &self.material));
            }
            temp = (-b + (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let t = temp;
                let p = r.point_at_parameter(t);
                let normal = (p - center) / self.radius;
                let (u, v) = self.get_sphere_uv(&((p - center) / self.radius));
                return Some((HitRecord::new(u, v, t, p, normal), &self.material));
            }
        }
        None
    }
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb> {
        Some(surrounding_box(
            &Aabb::new(
                self.center0 - Vector3::new(self.radius, self.radius, self.radius),
                self.center0 + Vector3::new(self.radius, self.radius, self.radius),
            ),
            &Aabb::new(
                self.center1 - Vector3::new(self.radius, self.radius, self.radius),
                self.center1 + Vector3::new(self.radius, self.radius, self.radius),
            ),
        ))
    }
}
