use super::object::{HitRecord, Object};
use super::ray::Ray;
use super::vector3::Vector3;

pub struct ObjectList {
  list: Vec<Box<dyn Object>>,
}

impl ObjectList {
  pub fn new(list: Vec<Box<dyn Object>>) -> ObjectList {
    ObjectList { list }
  }
}

impl Object for ObjectList {
  fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
    let mut hit_anything: bool = false;
    let mut closest_so_far: f32 = t_max;
    // needs refactor to avoid useless initialization but don't know how yet
    let mut rec: HitRecord = HitRecord::new(
      0.0,
      Vector3::new(0.0, 0.0, 0.0),
      Vector3::new(0.0, 0.0, 0.0),
    );

    for element in self.list.iter() {
      match element.hit(r, t_min, closest_so_far) {
        Some(temp_rec) => {
          hit_anything = true;
          closest_so_far = temp_rec.t;
          rec = temp_rec;
        }
        None => {}
      }
    }

    if hit_anything {
      Some(rec)
    } else {
      None
    }
  }
}
