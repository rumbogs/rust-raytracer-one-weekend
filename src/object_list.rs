use super::material::Material;
use super::object::{HitRecord, Hittable};
use super::ray::Ray;
// use super::sphere::{Plane, Sphere};

pub struct ObjectList {
  pub list: Vec<Box<dyn Hittable>>,
}

impl ObjectList {
  pub fn new(list: Vec<Box<dyn Hittable>>) -> Self {
    ObjectList { list }
  }
}

impl Hittable for ObjectList {
  fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<(HitRecord, &Material)> {
    let mut closest_so_far: f32 = t_max;
    let mut hit_objects: Vec<(HitRecord, &Material)> = vec![];

    for element in self.list.iter() {
      match element.hit(r, t_min, closest_so_far) {
        Some((rec, mat)) => {
          closest_so_far = rec.t;
          hit_objects.push((rec, mat))
        }
        None => {}
      }
    }

    if !hit_objects.is_empty() {
      match hit_objects.pop() {
        Some(entry) => Some(entry),
        None => None,
      }
    } else {
      None
    }
  }
}
