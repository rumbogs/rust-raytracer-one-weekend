use rand::Rng;

use super::super::aabb::Aabb;
use super::super::hittable::{HitRecord, Hittable};
use super::super::material::Material;
use super::super::ray::Ray;
use super::super::texture::Texture;
use super::super::vector3::Vector3;

pub struct ConstantMedium {
    boundary: Box<Hittable>,
    density: f32,
    phase_function: Material,
}

impl ConstantMedium {
    pub fn new(boundary: Box<Hittable>, density: f32, texture: Texture) -> Self {
        ConstantMedium {
            boundary,
            density,
            phase_function: Material::Isotropic { texture },
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<(HitRecord, &Material)> {
        let mut rng = rand::thread_rng();
        match self.boundary.hit(r, -std::f32::MAX, std::f32::MAX) {
            Some((mut rec1, _mat1)) => match self.boundary.hit(r, rec1.t + 0.0001, std::f32::MAX) {
                Some((mut rec2, _mat2)) => {
                    if rec1.t < t_min {
                        rec1.t = t_min
                    }
                    if rec2.t > t_max {
                        rec2.t = t_max
                    }
                    if rec1.t >= rec2.t {
                        return None;
                    }
                    if rec1.t < 0.0 {
                        rec1.t = 0.0;
                    }
                    let distance_inside_boundary: f32 = (rec2.t - rec1.t) * r.direction().length();
                    let hit_distance: f32 = -(1.0 / self.density) * rng.gen::<f32>().ln();
                    if hit_distance < distance_inside_boundary {
                        let t: f32 = rec1.t + hit_distance / r.direction().length();
                        let p = r.point_at_parameter(t);
                        let normal = Vector3::new(1.0, 0.0, 0.0); // random
                        return Some((
                            HitRecord::new(0.0, 0.0, t, p, normal),
                            &self.phase_function,
                        ));
                    }
                }
                None => {}
            },
            None => {}
        }
        None
    }
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb> {
        self.boundary.bounding_box(t0, t1)
    }
}
