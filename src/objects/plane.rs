use super::super::hittable::{Hittable, HitRecord};
use super::super::material::Material;
use super::super::vector3::Vector3;
use super::super::aabb::Aabb;
use super::super::ray::Ray;

pub struct XYRect {
    pub x0: f32,
    pub x1: f32,
    pub y0: f32,
    pub y1: f32,
    pub k: f32,
    pub material: Material,
}
pub struct XZRect {
    pub x0: f32,
    pub x1: f32,
    pub z0: f32,
    pub z1: f32,
    pub k: f32,
    pub material: Material,
}
pub struct YZRect {
    pub y0: f32,
    pub y1: f32,
    pub z0: f32,
    pub z1: f32,
    pub k: f32,
    pub material: Material,
}

impl Hittable for XYRect {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<(HitRecord, &Material)> {
        let ray_origin = r.origin();
        let ray_direction = r.direction();
        let t: f32 = (self.k - ray_origin.z()) / r.direction().z();
        if t < t_min || t > t_max {
            return None;
        }
        let x: f32 = ray_origin.x() + t * ray_direction.x();
        let y: f32 = ray_origin.y() + t * ray_direction.y();
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }
        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (y - self.y0) / (self.y1 - self.y0);
        let p = r.point_at_parameter(t);
        let normal = Vector3::new(0.0, 0.0, 1.0);
        Some((HitRecord::new(u, v, t, p, normal), &self.material))
    }
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb> {
        Some(Aabb::new(
            Vector3::new(self.x0, self.y0, self.k - 0.0001),
            Vector3::new(self.x1, self.y1, self.k + 0.0001),
        ))
    }
}

impl Hittable for XZRect {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<(HitRecord, &Material)> {
        let ray_origin = r.origin();
        let ray_direction = r.direction();
        let t: f32 = (self.k - ray_origin.y()) / r.direction().y();
        if t < t_min || t > t_max {
            return None;
        }
        let x: f32 = ray_origin.x() + t * ray_direction.x();
        let z: f32 = ray_origin.z() + t * ray_direction.z();
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }
        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        let p = r.point_at_parameter(t);
        let normal = Vector3::new(0.0, 1.0, 0.0);
        Some((HitRecord::new(u, v, t, p, normal), &self.material))
    }
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb> {
        Some(Aabb::new(
            Vector3::new(self.x0, self.k - 0.0001, self.z0),
            Vector3::new(self.x1, self.k + 0.0001, self.z1),
        ))
    }
}

impl Hittable for YZRect {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<(HitRecord, &Material)> {
        let ray_origin = r.origin();
        let ray_direction = r.direction();
        let t: f32 = (self.k - ray_origin.x()) / r.direction().x();
        if t < t_min || t > t_max {
            return None;
        }
        let y: f32 = ray_origin.y() + t * ray_direction.y();
        let z: f32 = ray_origin.z() + t * ray_direction.z();
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }
        let u = (y - self.y0) / (self.y1 - self.y0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        let p = r.point_at_parameter(t);
        let normal = Vector3::new(1.0, 0.0, 0.0);
        Some((HitRecord::new(u, v, t, p, normal), &self.material))
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb> {
        Some(Aabb::new(
            Vector3::new(self.k - 0.0001, self.y0, self.z0),
            Vector3::new(self.k + 0.0001, self.y1, self.z1),
        ))
    }
}
