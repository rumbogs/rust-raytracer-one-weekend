use super::Aabb;
use super::Hittable::{Hittable, HitRecord};

use rand::Rng;
use std::io::{self, Write};

fn box_x_compare(a, b) -> usize {
    let box_left: Aabb;
    let box_right: Aabb;
    match a.bounding_box(time0, time1) {
        Some(bbox) => { box_left = bbox },
        None => io::stderr().write(b"no bounding box in BvhNode constructor\n")?;
    }
    match b.bounding_box(time0, time1) {
        Some(bbox) => { box_right = bbox },
        None => io::stderr().write(b"no bounding box in BvhNode constructor\n")?;
    }

    if box_left.min.x - box_right.min.x < 0.0 {
        -1
    } else {
        1
    }
}

fn box_y_compare(a, b) -> usize {
    let box_left: Aabb;
    let box_right: Aabb;
    match a.bounding_box(time0, time1) {
        Some(bbox) => { box_left = bbox },
        None => io::stderr().write(b"no bounding box in BvhNode constructor\n")?;
    }
    match b.bounding_box(time0, time1) {
        Some(bbox) => { box_right = bbox },
        None => io::stderr().write(b"no bounding box in BvhNode constructor\n")?;
    }

    if box_left.min.y - box_right.min.y < 0.0 {
        -1
    } else {
        1
    }
}

fn box_z_compare(a, b) -> usize {
    let box_left: Aabb;
    let box_right: Aabb;
    match a.bounding_box(time0, time1) {
        Some(bbox) => { box_left = bbox },
        None => io::stderr().write(b"no bounding box in BvhNode constructor\n")?;
    }
    match b.bounding_box(time0, time1) {
        Some(bbox) => { box_right = bbox },
        None => io::stderr().write(b"no bounding box in BvhNode constructor\n")?;
    }

    if box_left.min.z - box_right.min.z < 0.0 {
        -1
    } else {
        1
    }
}

pub struct BvhNode {
    bbox: Aabb,
}

impl BvhNode {
    pub fn new(hittable: Hittable, n: usize, time0: f32, time1: f32) -> Self {
        let axis: usize = rng.gen::<f32>() as usize;
        if axis == 0 {
            sort(hittable, n, box_x_compare);
        } else if axis == 1 {
            sort(hittable, n, box_y_compare);
        } else {
            sort(hittable, n, box_z_compare);
        }
        if n == 1 {
            self.left = hittable[0];
            self.right = hittable[0];
        } else if n == 2 {
            self.left = hittable[0];
            self.right = hittable[1];
        } else {
            self.left = BvhNode::new(hittable, n / 2, time0, time1);
            self.right = BvhNode::new(hittable + n / 2, n - n / 2, time0, time1);
        }
        let box_left: Aabb;
        let box_right: Aabb;
        match self.left.bounding_box(time0, time1) {
            Some(bbox) => { box_left = bbox },
            None => io::stderr().write(b"no bounding box in BvhNode constructor\n")?;
        }
        match self.right.bounding_box(time0, time1) {
            Some(bbox) => { box_right = bbox },
            None => io::stderr().write(b"no bounding box in BvhNode constructor\n")?;
        }
        self.box = surrounding_box(box_left, box_right);
        self
    }

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<(HitRecord, &Material)> {
        match self.bbox.hit(r, t_min, t_max) {
            Some((rec, mat)) => {
                match self.left.hit(r, t_min, t_max) {
                    Some((left_rec, left_mat)) => {
                        match self.right.hit(r, t_min, t_max)) {
                            Some((right_rec, right_mat)) => {
                                if left_rec.t < right_rec.t {
                                    Some((left_rec, left_mat))
                                } else {
                                    Some((right_rec, right_mat))
                                }
                            },
                            None => Some((left_rec, left_mat))
                        }
                    },
                    None => {
                        match.self.right.hit(r, t_min, t_max)) {
                            Some((right_rec, right_mat)) => Some((right_rec, right_mat)),
                            None => None
                        }
                    }
                };
            },
            None => None
        }
    }
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb> {
        self.bbox
    }
}
