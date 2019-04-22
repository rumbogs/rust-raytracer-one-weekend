use super::aabb::{surrounding_box, Aabb};
use super::material::Material;
use super::object::{HitRecord, Hittable};
use super::ray::Ray;

pub struct ObjectList {
    // this needs to be dynamic to allow for different
    // structs that implement the Hittable trait
    // otherwise it would allow for only one kind
    // e.g. Sphere
    pub list: Vec<Box<dyn Hittable + Sync>>,
}

impl Clone for Box<dyn Hittable + Sync> {
    fn clone(&self) -> Box<dyn Hittable + Sync> {
        self.box_clone()
    }
}

impl ObjectList {
    pub fn new(list: Vec<Box<dyn Hittable + Sync>>) -> Self {
        ObjectList { list }
    }

    pub fn getSlice(&self, start_index: usize, end_index: usize) -> ObjectList {
        let mut list = vec![];
        for i in start_index..end_index {
            list.push(self.list[i].clone());
        }
        ObjectList { list }
    }
}

impl Hittable for ObjectList {
    fn box_clone(&self) -> Box<dyn Hittable + Sync> {
        let mut list = vec![];
        for element in self.list.iter() {
            list.push(element.clone());
        }
        Box::new(ObjectList { list })
    }

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
                            hit_bbox = surrounding_box(hit_bbox, bbox);
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
