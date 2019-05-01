use super::aabb::{surrounding_box, Aabb};
use super::material::Material;
use super::object::{HitRecord, Hittable};
use super::object_list::ObjectList;
use super::qsort::quick_sort;
use super::ray::Ray;
use super::vector3::Vector3;

use rand::Rng;
use std::io::{self, Write};

fn box_x_compare(a: &Box<Hittable + Sync>, b: &Box<Hittable + Sync>) -> bool {
    let mut box_left: Aabb = Aabb::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0));
    let mut box_right: Aabb = Aabb::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0));
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

fn box_y_compare(a: &Box<Hittable + Sync>, b: &Box<Hittable + Sync>) -> bool {
    let mut box_left: Aabb = Aabb::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0));
    let mut box_right: Aabb = Aabb::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0));
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

fn box_z_compare(a: &Box<Hittable + Sync>, b: &Box<Hittable + Sync>) -> bool {
    let mut box_left: Aabb = Aabb::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0));
    let mut box_right: Aabb = Aabb::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0));
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
    bvhs: Option<(Box<BvhNode>, Box<BvhNode>)>,
    roots: Option<(Box<Hittable + Sync>, Box<Hittable + Sync>)>,
}

impl BvhNode {
    pub fn new(objects: &[Box<Hittable + Sync>], time0: f32, time1: f32) -> Box<BvhNode> {
        // let mut rng = rand::thread_rng();
        let bvhs: Option<(Box<BvhNode>, Box<BvhNode>)> = None;
        let roots: Option<(Box<Hittable + Sync>, Box<Hittable + Sync>)> = None;

        // let axis: usize = 3 * (rng.gen::<f32>() as usize);
        let bbox: Aabb;
        let n = objects.len();

        // if axis == 0 {
        //     quick_sort(&mut objects.list, &box_x_compare);
        // } else if axis == 1 {
        //     quick_sort(&mut objects.list, &box_y_compare);
        // } else {
        //     quick_sort(&mut objects.list, &box_z_compare);
        // }

        if n == 1 {
            println!("n = 1");
            roots = Some((objects[0], objects[0]));
        } else if n == 2 {
            println!("n = 2");
            roots = Some((objects[0], objects[1]));
        } else {
            bvhs = Some((
                BvhNode::new(&objects[0..n / 2], time0, time1),
                BvhNode::new(&objects[n / 2..n], time0, time1),
            ));
        }

        let mut box_left: Aabb =
            Aabb::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0));
        let mut box_right: Aabb =
            Aabb::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0));
        match roots {
            Some((left, right)) => {
                match left.bounding_box(time0, time1) {
                    Some(bbox) => box_left = bbox,
                    None => {
                        io::stderr().write(b"no bounding box in BvhNode constructor\n");
                    }
                };
                match right.bounding_box(time0, time1) {
                    Some(bbox) => box_right = bbox,
                    None => {
                        io::stderr().write(b"no bounding box in BvhNode constructor\n");
                    }
                };
            }
            None => {
                match bvhs {
                    Some((leftBvh, rightBvh)) => {
                        match leftBvh.bounding_box(time0, time1) {
                            Some(bbox) => box_left = bbox,
                            None => {
                                io::stderr().write(b"no bounding box in BvhNode constructor\n");
                            }
                        };
                        match rightBvh.bounding_box(time0, time1) {
                            Some(bbox) => box_right = bbox,
                            None => {
                                io::stderr().write(b"no bounding box in BvhNode constructor\n");
                            }
                        };
                    }
                    None => { /* shouldn't happen */ }
                };
            }
        }
        bbox = surrounding_box(box_left, box_right);
        Box::new(BvhNode { roots, bvhs, bbox })
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<(HitRecord, &Material)> {
        match self.bbox.hit(r, t_min, t_max) {
            Some((rec, mat)) => match self.roots {
                Some((left, right)) => match left.hit(r, t_min, t_max) {
                    Some((left_rec, left_mat)) => match right.hit(r, t_min, t_max) {
                        Some((right_rec, right_mat)) => {
                            if left_rec.t < right_rec.t {
                                Some((left_rec, left_mat))
                            } else {
                                Some((right_rec, right_mat))
                            }
                        }
                        None => Some((left_rec, left_mat)),
                    },
                    None => match right.hit(r, t_min, t_max) {
                        Some((right_rec, right_mat)) => Some((right_rec, right_mat)),
                        None => None,
                    },
                },
                None => match self.bvhs {
                    Some((left, right)) => match left.hit(r, t_min, t_max) {
                        Some((left_rec, left_mat)) => match right.hit(r, t_min, t_max) {
                            Some((right_rec, right_mat)) => {
                                if left_rec.t < right_rec.t {
                                    Some((left_rec, left_mat))
                                } else {
                                    Some((right_rec, right_mat))
                                }
                            }
                            None => Some((left_rec, left_mat)),
                        },
                        None => match right.hit(r, t_min, t_max) {
                            Some((right_rec, right_mat)) => Some((right_rec, right_mat)),
                            None => None,
                        },
                    },
                    None => None,
                },
            },
            None => None,
        }
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb> {
        Some(self.bbox.clone())
    }
}
