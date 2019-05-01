use super::aabb::{surrounding_box, Aabb};
use super::material::Material;
use super::object::{HitRecord, Hittable};
use super::ray::Ray;
use super::vector3::{dot, Vector3};

pub struct MovingSphere {
    center0: Vector3,
    center1: Vector3,
    time0: f32,
    time1: f32,
    radius: f32,
    material: Material,
}

impl MovingSphere {
    pub fn new(
        center0: Vector3,
        center1: Vector3,
        time0: f32,
        time1: f32,
        radius: f32,
        material: Material,
    ) -> Self {
        MovingSphere {
            center0,
            center1,
            time0,
            time1,
            radius,
            material,
        }
    }

    pub fn center(&self, time: f32) -> Vector3 {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<(HitRecord, &Material)> {
        let oc: Vector3 = r.origin() - self.center(r.time);
        let a: f32 = dot(r.direction(), r.direction());
        let b: f32 = dot(oc, r.direction());
        let c: f32 = dot(oc, oc) - self.radius * self.radius;
        let discriminant: f32 = b * b - a * c;
        if discriminant > 0.0 {
            let mut temp: f32 = (-b - (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let t = temp;
                let p = r.point_at_parameter(t);
                let normal = (p - self.center(r.time)) / self.radius;
                return Some((HitRecord::new(t, p, normal), &self.material));
            }
            temp = (-b + (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let t = temp;
                let p = r.point_at_parameter(t);
                let normal = (p - self.center(r.time)) / self.radius;
                return Some((HitRecord::new(t, p, normal), &self.material));
            }
        }
        None
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb> {
        Some(surrounding_box(
            Aabb::new(
                self.center0 - Vector3::new(self.radius, self.radius, self.radius),
                self.center0 + Vector3::new(self.radius, self.radius, self.radius),
            ),
            Aabb::new(
                self.center1 - Vector3::new(self.radius, self.radius, self.radius),
                self.center1 + Vector3::new(self.radius, self.radius, self.radius),
            ),
        ))
    }
}
