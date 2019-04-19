use super::vector3::Vector3;
use super::object::{HitRecord, Hittable};
use std::cmp::{min, max};

pub struct Aabb {
    min: Vector3,
    max: Vector3,
}

impl Aabb {
    pub fn new(min: Vector3, max: Vector3) -> Self {
        Aabb {
            min,
            max,
        }
    }
}

impl Hittable for Aabb {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<(HitRecord, &Material)> {
        for a in 0..3 {
            let t0: f32 = min((&self.min[a] - r.origin()[a]) / r.direction()[a],
                                (&self.max[a] - r.origin()[a]) / r.direction()[a]);
            let t1: f32 = max((&self.min[a] - r.origin()[a]) / r.direction()[a],
                                (&self.max[a] - r.origin()[a]) / r.direction()[a]);
            let tmin = max(t0, t_min);
            let tmax = min(t1, t_max);
            if (tmax <= tmin) false
        }
        true
    }
}

fn surrounding_box(box0: Aabb, box1: Aabb) -> Aabb {
    let small: Vector3 = Vector3::new(min(box0.min().x, box1.min().x), min(box0.min().y, box1.min().y), min(box0.min().z, box1.min().z));
    let big: Vector3 = Vector3::new(max(box0.max().x, box1.max().x), max(box0.max().y, box1.max().y), max(box0.max().z, box1.max().z));

    Aabb::new(small, big)
}
