use super::super::hittable::{Hittable, HitRecord};
use super::super::material::Material;
use super::super::aabb::Aabb;
use super::super::ray::Ray;

pub struct FlipNormals {
    object: Box<Hittable>
}

impl FlipNormals {
    pub fn new(object: Box<Hittable>) -> Self {
        FlipNormals { object }
    }
}

impl Hittable for FlipNormals {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<(HitRecord, &Material)> {
        match self.object.hit(r, t_min, t_max) {
            Some((rec, mat)) => {
                Some((HitRecord::new(rec.u, rec.v, rec.t, rec.p, -rec.normal), mat))
            }
            None => None,
        }
    }
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb> {
        self.object.bounding_box(t0, t1)
    }
}