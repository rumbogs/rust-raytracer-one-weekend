use super::aabb::Aabb;
use super::material::Material;
use super::ray::Ray;
use super::vector3::Vector3;

pub struct HitRecord {
    pub t: f32,
    pub p: Vector3,
    pub normal: Vector3,
}

impl HitRecord {
    pub fn new(t: f32, p: Vector3, normal: Vector3) -> HitRecord {
        HitRecord { t, p, normal }
    }
}

pub trait Hittable {
    fn box_clone(&self) -> Box<dyn Hittable + Sync>;
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<(HitRecord, &Material)>;
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb>;
}
