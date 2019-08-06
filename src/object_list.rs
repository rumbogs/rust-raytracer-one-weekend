use super::material::Material;
use super::object::{HitRecord, Hittable};
use super::ray::Ray;
use super::aabb::{Aabb, surrounding_box};

pub struct ObjectList {
    // this needs to be dynamic to allow for different
    // structs that implement the Hittable trait
    // otherwise it would allow for only one kind
    // e.g. Sphere
    pub list: Vec<Box<Hittable>>,
}

impl ObjectList {
    pub fn new(list: Vec<Box<Hittable>>) -> Self {
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

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb> {
        if self.list.is_empty() {
            return None;
        }

        let mut hit_bbox: Aabb;

        match self.list[0].bounding_box(t0, t1) {
            Some(first_bbox) => {
                hit_bbox = first_bbox;
                for element in self.list.iter() {
                    match element.bounding_box(t0, t1) {
                        Some(bbox) => {
                            hit_bbox = surrounding_box(&hit_bbox, &bbox);
                        }
                        None => return Some(hit_bbox),
                    }
                }
            }
            None => return None,
        }

        Some(hit_bbox)
    }
}
