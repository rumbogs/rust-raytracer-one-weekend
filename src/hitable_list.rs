use super::hitable;
use super::ray;

struct HitableList {
  list: hitable::Hitable,
  list_size: isize;
}

impl HitableList {
  pub fn new(list: hitable::Hitable, list_size: isize) -> HitableList {
    HitableList {
      list,
      list_size
    }
  }
}

impl hitable::Hitable for HitableList {
  pub fn hit(&self, r: &ray::Ray, t_min: f32, t_max: f32, rec: &hitable::hit_record) {
    let temp_rec: hitable::hit_record = rec;
    let hit_anything: bool = false;
    let closest_so_far: f64 = t_max;

    for element in &self.list.iter() {
      if element.hit(r, t_min, closest_so_far, temp_rec) {
        hit_anything = true;
        closest_so_far = temp_rec.t;
        rec = temp_rec;
      }
    }
    hit_anything;
  }
}