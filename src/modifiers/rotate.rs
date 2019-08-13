use std::f32::consts::PI;

use super::super::aabb::Aabb;
use super::super::hittable::{HitRecord, Hittable};
use super::super::material::Material;
use super::super::ray::Ray;
use super::super::vector3::Vector3;

pub struct RotateY {
    pub object: Box<Hittable>,
    pub sin_theta: f32,
    pub cos_theta: f32,
    pub aabb: Aabb,
}

impl RotateY {
    pub fn new(object: Box<Hittable>, angle: f32) -> Self {
        let radians: f32 = (PI / 180.0) * angle;
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let aabb = object.bounding_box(0.0, 1.0);
        let mut min = Vector3::new(std::f32::MAX, std::f32::MAX, std::f32::MAX);
        let mut max = Vector3::new(-std::f32::MAX, -std::f32::MAX, -std::f32::MAX);
        match aabb {
            Some(bbox) => {
                for i in 0..2 {
                    for j in 0..2 {
                        for k in 0..2 {
                            let x = i as f32 * bbox.max.x() + (1 - i) as f32 * bbox.min.x();
                            let y = j as f32 * bbox.max.y() + (1 - j) as f32 * bbox.min.y();
                            let z = k as f32 * bbox.max.z() + (1 - k) as f32 * bbox.min.z();
                            let newx: f32 = cos_theta * x + sin_theta * z;
                            let newz: f32 = -sin_theta * x + cos_theta * z;
                            let tester: Vector3 = Vector3::new(newx, y, newz);
                            for c in 0..3 {
                                if tester[c] > max[c] {
                                    max[c] = tester[c];
                                }
                                if tester[c] < min[c] {
                                    min[c] = tester[c];
                                }
                            }
                        }
                    }
                }
            }
            None => {}
        };
        RotateY {
            object,
            sin_theta,
            cos_theta,
            aabb: Aabb::new(min, max),
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<(HitRecord, &Material)> {
        let mut origin = r.origin();
        let mut direction = r.direction();
        origin[0] = self.cos_theta * r.origin()[0] - self.sin_theta * r.origin()[2];
        origin[2] = self.sin_theta * r.origin()[0] + self.cos_theta * r.origin()[2];
        direction[0] = self.cos_theta * r.direction()[0] - self.sin_theta * r.direction()[2];
        direction[2] = self.sin_theta * r.direction()[0] + self.cos_theta * r.direction()[2];
        let rotated_r = Ray::new(origin, direction, r.time);
        match self.object.hit(&rotated_r, t_min, t_max) {
            Some((rec, mat)) => {
                let mut p = rec.p;
                let mut normal = rec.normal;
                p[0] = self.cos_theta * rec.p[0] + self.sin_theta * rec.p[2];
                p[2] = -self.sin_theta * rec.p[0] + self.cos_theta * rec.p[2];
                normal[0] = self.cos_theta * rec.normal[0] + self.sin_theta * rec.normal[2];
                normal[2] = -self.sin_theta * rec.normal[0] + self.cos_theta * rec.normal[2];
                Some((HitRecord::new(rec.u, rec.v, rec.t, p, normal), mat))
            }
            None => None,
        }
    }
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb> {
        Some(self.aabb)
    }
}
