use super::ray::Ray;
use super::vector3::Vector3;

#[derive(Copy, Clone)]
pub struct Aabb {
    pub min: Vector3,
    pub max: Vector3,
}

impl Aabb {
    pub fn new(min: Vector3, max: Vector3) -> Self {
        Aabb { min, max }
    }

    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> bool {
        for a in 0..3 {
            let tmin;
            let tmax;
            let t0: f32 = ((self.min[a] - r.origin()[a]) / r.direction()[a])
                .min((self.max[a] - r.origin()[a]) / r.direction()[a]);
            let t1: f32 = ((self.min[a] - r.origin()[a]) / r.direction()[a])
                .max((self.max[a] - r.origin()[a]) / r.direction()[a]);
            tmin = t0.max(t_min);
            tmax = t1.min(t_max);
            if tmax <= tmin {
                return false;
            }
        }
        return true;
    }
}

pub fn surrounding_box(box0: &Aabb, box1: &Aabb) -> Aabb {
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
