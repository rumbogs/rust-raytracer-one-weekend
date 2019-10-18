use rand::Rng;
use std::cmp::Ordering;

use super::super::aabb::{surrounding_box, Aabb};
use super::super::hittable::{HitRecord, Hittable};
use super::super::material::Material;
use super::super::ray::Ray;

pub enum BinaryTree {
    Leaf(Box<Hittable>),
    Node(Box<BvhTree>, Box<BvhTree>),
}

pub struct BvhTree {
    pub binary_tree: BinaryTree,
    pub aabb: Aabb,
}

fn box_x_compare(a: &Box<Hittable>, b: &Box<Hittable>) -> Ordering {
    let left_aabb = a.bounding_box(0.0, 0.0);
    let right_aabb = b.bounding_box(0.0, 0.0);
    match (left_aabb, right_aabb) {
        (Some(lbb), Some(rbb)) => {
            let delta: f32 = lbb.min.x() - rbb.min.x();
            if delta < 0.0 {
                Ordering::Less
            } else if delta == 0.0 {
                Ordering::Equal
            } else {
                Ordering::Greater
            }
        }
        _ => Ordering::Less,
    }
}

fn box_y_compare(a: &Box<Hittable>, b: &Box<Hittable>) -> Ordering {
    let left_aabb = a.bounding_box(0.0, 0.0);
    let right_aabb = b.bounding_box(0.0, 0.0);
    match (left_aabb, right_aabb) {
        (Some(lbb), Some(rbb)) => {
            let delta: f32 = lbb.min.y() - rbb.min.y();
            if delta < 0.0 {
                Ordering::Less
            } else if delta == 0.0 {
                Ordering::Equal
            } else {
                Ordering::Greater
            }
        }
        _ => Ordering::Less,
    }
}

fn box_z_compare(a: &Box<Hittable>, b: &Box<Hittable>) -> Ordering {
    let left_aabb = a.bounding_box(0.0, 0.0);
    let right_aabb = b.bounding_box(0.0, 0.0);
    match (left_aabb, right_aabb) {
        (Some(lbb), Some(rbb)) => {
            let delta: f32 = lbb.min.z() - rbb.min.z();
            if delta < 0.0 {
                Ordering::Less
            } else if delta == 0.0 {
                Ordering::Equal
            } else {
                Ordering::Greater
            }
        }
        _ => Ordering::Less,
    }
}

impl BvhTree {
    pub fn new(mut list: Vec<Box<Hittable>>, t0: f32, t1: f32) -> Self {
        let mut rng = rand::thread_rng();
        let axis: usize = rng.gen_range(0, 3);
        match axis {
            0 => list.sort_by(box_x_compare),
            1 => list.sort_by(box_y_compare),
            _ => list.sort_by(box_z_compare),
        };
        let list_length = list.len();

        match list_length {
            1 => {
                let hittable = list.pop().unwrap();
                let bbox = hittable.bounding_box(t0, t1);

                return BvhTree {
                    binary_tree: BinaryTree::Leaf(hittable),
                    aabb: match bbox {
                        Some(bb) => bb,
                        None => panic!["No bounding box"],
                    },
                };
            }
            _ => {
                let mut vec1: Vec<Box<Hittable>> = Vec::with_capacity(list_length / 2 + 1);
                let mut vec2: Vec<Box<Hittable>> = Vec::with_capacity(list_length / 2);

                for (i, el) in list.into_iter().enumerate() {
                    if i < list_length / 2 {
                        vec1.push(el);
                    } else {
                        vec2.push(el);
                    }
                }
                let left = BvhTree::new(vec1, t0, t1);
                let right = BvhTree::new(vec2, t0, t1);
                let left_bbox = left.bounding_box(t0, t1);
                let right_bbox = right.bounding_box(t0, t1);
                let aabb = match (left_bbox, right_bbox) {
                    (Some(lbb), Some(rbb)) => surrounding_box(&lbb, &rbb),
                    (Some(lbb), None) => lbb,
                    (None, Some(rbb)) => rbb,
                    (None, None) => panic!["No bounding box"],
                };
                return BvhTree {
                    binary_tree: BinaryTree::Node(Box::new(left), Box::new(right)),
                    aabb,
                };
            }
        }
    }
}

impl Hittable for BvhTree {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<(HitRecord, &Material)> {
        match &self.binary_tree {
            BinaryTree::Leaf(hittable) => hittable.hit(r, t_min, t_max),
            BinaryTree::Node(left, right) => {
                if self.aabb.hit(r, t_min, t_max) {
                    let left_rec = match left.hit(r, t_min, t_max) {
                        Some((lr, lm)) => Some((lr, lm)),
                        None => None,
                    };

                    let right_rec = match right.hit(r, t_min, t_max) {
                        Some((rr, rm)) => Some((rr, rm)),
                        None => None,
                    };

                    match (left_rec, right_rec) {
                        (Some((lr, lm)), Some((rr, rm))) => {
                            if lr.t < rr.t {
                                Some((lr, lm))
                            } else {
                                Some((rr, rm))
                            }
                        }
                        (Some((rec, m)), None) | (None, Some((rec, m))) => Some((rec, m)),
                        (None, None) => None,
                    }
                } else {
                    None
                }
            }
        }
    }
    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<Aabb> {
        Some(self.aabb)
    }
}
