use super::super::hittable::{Hittable, HitRecord};
use super::super::material::Material;
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
    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<(HitRecord, &Material)> {
        let ray_origin = r.origin();
                let ray_direction = r.direction();
                let t: f32 = (k - ray_origin.z()) / r.direction().z();
                if t < t_min || t > t_max {
                    return None;
                }
                let x: f32 = ray_origin.x() + t * ray_direction.x();
                let y: f32 = ray_origin.y() + t * ray_direction.y();
                if x < *x0 || x > *x1 || y < *y0 || y > *y1 {
                    return None;
                }
                let u = (x - x0) / (x1 - x0);
                let v = (y - y0) / (y1 - y0);
                let p = r.point_at_parameter(t);
                let normal = Vector3::new(0.0, 0.0, 1.0);
                Some((HitRecord::new(u, v, t, p, normal), &material))
            // Object::XZRect {
            //     x0,
            //     x1,
            //     z0,
            //     z1,
            //     k,
            //     material,
            // } => {
            //     let ray_origin = r.origin();
            //     let ray_direction = r.direction();
            //     let t: f32 = (k - ray_origin.y()) / r.direction().y();
            //     if t < t_min || t > t_max {
            //         return None;
            //     }
            //     let x: f32 = ray_origin.x() + t * ray_direction.x();
            //     let z: f32 = ray_origin.z() + t * ray_direction.z();
            //     if x < *x0 || x > *x1 || z < *z0 || z > *z1 {
            //         return None;
            //     }
            //     let u = (x - x0) / (x1 - x0);
            //     let v = (z - z0) / (z1 - z0);
            //     let p = r.point_at_parameter(t);
            //     let normal = Vector3::new(0.0, 1.0, 0.0);
            //     Some((HitRecord::new(u, v, t, p, normal), &material))
            // }
            // Object::YZRect {
            //     y0,
            //     y1,
            //     z0,
            //     z1,
            //     k,
            //     material,
            // } => {
            //     let ray_origin = r.origin();
            //     let ray_direction = r.direction();
            //     let t: f32 = (k - ray_origin.x()) / r.direction().x();
            //     if t < t_min || t > t_max {
            //         return None;
            //     }
            //     let y: f32 = ray_origin.y() + t * ray_direction.y();
            //     let z: f32 = ray_origin.z() + t * ray_direction.z();
            //     if y < *y0 || y > *y1 || z < *z0 || z > *z1 {
            //         return None;
            //     }
            //     let u = (y - y0) / (y1 - y0);
            //     let v = (z - z0) / (z1 - z0);
            //     let p = r.point_at_parameter(t);
            //     let normal = Vector3::new(1.0, 0.0, 0.0);
            //     Some((HitRecord::new(u, v, t, p, normal), &material))
            // }
    }
    pub fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb> {
        Object::XYRect {
                x0,
                x1,
                y0,
                y1,
                k,
                material: _,
            } => Some(Aabb::new(
                Vector3::new(*x0, *y0, k - 0.0001),
                Vector3::new(*x1, *y1, k + 0.0001),
            )),
            Object::XZRect {
                x0,
                x1,
                z0,
                z1,
                k,
                material: _,
            } => Some(Aabb::new(
                Vector3::new(*x0, k - 0.0001, *z0),
                Vector3::new(*x1, k + 0.0001, *z1),
            )),
            Object::YZRect {
                y0,
                y1,
                z0,
                z1,
                k,
                material,
            } => Some(Aabb::new(
                Vector3::new(k - 0.0001, *y0, *z0),
                Vector3::new(k + 0.0001, *y1, *z1),
            )),
    }
}