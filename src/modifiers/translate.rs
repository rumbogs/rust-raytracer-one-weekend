use super::super::aabb::Aabb;
use super::super::hittable::{HitRecord, Hittable};
use super::super::material::Material;
use super::super::ray::Ray;
use super::super::vector3::Vector3;

pub struct Translate {
    pub object: Box<Hittable>,
    pub offset: Vector3,
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<(HitRecord, &Material)> {
        let moved_ray: Ray = Ray::new(r.origin() - self.offset, r.direction(), r.time);
        match self.object.hit(&moved_ray, t_min, t_max) {
            Some((rec, mat)) => Some((
                HitRecord::new(rec.u, rec.v, rec.t, rec.p + self.offset, rec.normal),
                mat,
            )),
            None => None,
        }
    }
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb> {
        match self.object.bounding_box(t0, t1) {
            Some(aabb) => Some(Aabb::new(aabb.min + self.offset, aabb.max + self.offset)),
            None => None,
        }
    }
}
