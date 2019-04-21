use super::aabb::{surrounding_box, Aabb};
use super::material::Material;
use super::object::{HitRecord, Hittable};
use super::qsort::quick_sort;
use super::ray::Ray;

use rand::Rng;
use std::io::{self, Write};

fn box_x_compare(a: &Box<dyn Hittable + Sync>, b: &Box<dyn Hittable + Sync>) -> bool {
    let box_left: Aabb;
    let box_right: Aabb;
    match a.bounding_box(0.0, 0.0) {
        Some(bbox) => box_left = bbox,
        None => {
            io::stderr().write(b"no bounding box in BvhNode constructor\n");
        }
    }
    match b.bounding_box(0.0, 0.0) {
        Some(bbox) => box_right = bbox,
        None => {
            io::stderr().write(b"no bounding box in BvhNode constructor\n");
        }
    }

    box_left.min.x() - box_right.min.x() >= 0.0
}

fn box_y_compare(a: &Box<dyn Hittable + Sync>, b: &Box<dyn Hittable + Sync>) -> bool {
    let box_left: Aabb;
    let box_right: Aabb;
    match a.bounding_box(0.0, 0.0) {
        Some(bbox) => box_left = bbox,
        None => {
            io::stderr().write(b"no bounding box in BvhNode constructor\n");
        }
    }
    match b.bounding_box(0.0, 0.0) {
        Some(bbox) => box_right = bbox,
        None => {
            io::stderr().write(b"no bounding box in BvhNode constructor\n");
        }
    }

    box_left.min.y() - box_right.min.y() >= 0.0
}

fn box_z_compare(a: &Box<dyn Hittable + Sync>, b: &Box<dyn Hittable + Sync>) -> bool {
    let box_left: Aabb;
    let box_right: Aabb;
    match a.bounding_box(0.0, 0.0) {
        Some(bbox) => box_left = bbox,
        None => {
            io::stderr().write(b"no bounding box in BvhNode constructor\n");
        }
    }
    match b.bounding_box(0.0, 0.0) {
        Some(bbox) => box_right = bbox,
        None => {
            io::stderr().write(b"no bounding box in BvhNode constructor\n");
        }
    }

    box_left.min.z() - box_right.min.z() >= 0.0
}

pub struct BvhNode {
    bbox: Aabb,
    left: Box<Hittable>,
    right: Box<Hittable>,
}

impl BvhNode {
    pub fn new(
        objects: Vec<Box<dyn Hittable + Sync>>,
        n: usize,
        time0: f32,
        time1: f32,
    ) -> Box<Hittable> {
        let mut rng = rand::thread_rng();
        let left: Box<Hittable>;
        let right: Box<Hittable>;
        let axis: usize = 3 * (rng.gen::<f32>() as usize);
        let bbox: Aabb;

        if axis == 0 {
            quick_sort(&mut objects, &box_x_compare);
        } else if axis == 1 {
            quick_sort(&mut objects, &box_y_compare);
        } else {
            quick_sort(&mut objects, &box_z_compare);
        }

        if n == 1 {
            left = objects[0];
            right = objects[0];
        } else if n == 2 {
            left = objects[0];
            right = objects[1];
        } else {
            left = BvhNode::new(objects, n / 2, time0, time1);
            right = BvhNode::new(objects + n / 2, n - n / 2, time0, time1);
        }

        let box_left: Aabb;
        let box_right: Aabb;
        match left.bounding_box(time0, time1) {
            Some(bbox) => box_left = bbox,
            None => {
                io::stderr().write(b"no bounding box in BvhNode constructor\n");
            }
        }
        match right.bounding_box(time0, time1) {
            Some(bbox) => box_right = bbox,
            None => {
                io::stderr().write(b"no bounding box in BvhNode constructor\n");
            }
        }
        bbox = surrounding_box(box_left, box_right);
        Box::new(BvhNode { left, right, bbox })
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<(HitRecord, &Material)> {
        match self.bbox.hit(r, t_min, t_max) {
            Some((rec, mat)) => match self.left.hit(r, t_min, t_max) {
                Some((left_rec, left_mat)) => match self.right.hit(r, t_min, t_max) {
                    Some((right_rec, right_mat)) => {
                        if left_rec.t < right_rec.t {
                            Some((left_rec, left_mat))
                        } else {
                            Some((right_rec, right_mat))
                        }
                    }
                    None => Some((left_rec, left_mat)),
                },
                None => match self.right.hit(r, t_min, t_max) {
                    Some((right_rec, right_mat)) => Some((right_rec, right_mat)),
                    None => None,
                },
            },
            None => None,
        }
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb> {
        Some(self.bbox)
    }
}
