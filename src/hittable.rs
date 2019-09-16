use super::aabb::Aabb;
use super::material::Material;
use super::ray::Ray;
use super::vector3::Vector3;

pub struct HitRecord {
    pub u: f32,
    pub v: f32,
    pub t: f32,
    pub p: Vector3,
    pub normal: Vector3,
}

impl HitRecord {
    pub fn new(u: f32, v: f32, t: f32, p: Vector3, normal: Vector3) -> HitRecord {
        HitRecord { u, v, t, p, normal }
    }
}

pub trait Hittable: Sync {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<(HitRecord, &Material)>;
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb>;
    fn pdf_value(&self, o: Vector3, v: Vector3) -> f32 {
        0.0
    }
    fn random(&self, o: Vector3) -> Vector3 {
        Vector3::new(1.0, 0.0, 0.0)
    }
}
