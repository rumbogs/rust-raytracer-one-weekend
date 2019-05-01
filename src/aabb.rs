use super::material::{Material, MaterialType};
use super::object::{HitRecord, Hittable};
use super::ray::Ray;
use super::texture::ConstantTexture;
use super::vector3::Vector3;

#[derive(Clone)]
pub struct Aabb {
    pub min: Vector3,
    pub max: Vector3,
    material: Material,
}

impl Aabb {
    pub fn new(min: Vector3, max: Vector3) -> Self {
        Aabb {
            min,
            max,
            // default material to be returned by hit (keep return signature)
            material: Material::new(
                MaterialType::Lambertian,
                Box::new(ConstantTexture::new(Vector3::new(0.4, 0.2, 0.1))),
                0.0,
                0.0,
            ),
        }
    }
}

impl Hittable for Aabb {
    fn box_clone(&self) -> Box<dyn Hittable + Sync> {
        Box::new(Aabb {
            min: self.min.clone(),
            max: self.max.clone(),
            material: self.material.clone(),
        })
    }
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<(HitRecord, &Material)> {
        let mut tmin;
        let mut tmax;
        for a in 0..3 {
            let t0: f32 = ((&self.min[a] - r.origin()[a]) / r.direction()[a])
                .min((&self.max[a] - r.origin()[a]) / r.direction()[a]);
            let t1: f32 = ((&self.min[a] - r.origin()[a]) / r.direction()[a])
                .max((&self.max[a] - r.origin()[a]) / r.direction()[a]);
            tmin = t0.max(t_min);
            tmax = t1.min(t_max);
            if tmax <= tmin {
                return None;
            }
        }
        Some((
            HitRecord::new(
                0.0,
                Vector3::new(0.0, 0.0, 0.0),
                Vector3::new(0.0, 0.0, 0.0),
            ),
            &self.material,
        ))
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb> {
        None
    }
}

pub fn surrounding_box(box0: Aabb, box1: Aabb) -> Aabb {
    let small: Vector3 = Vector3::new(
        box0.min.x().min(box1.min.x()),
        box0.min.y().min(box1.min.y()),
        box0.min.z().min(box1.min.z()),
    );
    let big: Vector3 = Vector3::new(
        box0.max.x().max(box1.max.x()),
        box0.max.y().max(box1.max.y()),
        box0.max.z().max(box1.max.z()),
    );

    Aabb::new(small, big)
}
