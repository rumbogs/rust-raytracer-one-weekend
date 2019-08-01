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

    pub fn get_slice(&self, start_index: usize, end_index: usize) -> ObjectList {
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
}
