use rand::Rng;
use std::cmp::Ordering;

use super::aabb::{Aabb, surrounding_box};
use super::material::Material;
use super::object::{HitRecord, Hittable};
use super::object_list::ObjectList;
use super::ray::Ray;

pub enum BinaryTree<'a> {
    Leaf(&'a Box<dyn Hittable + Sync>),
    Node(Box<BvhTree<'a>>, Box<BvhTree<'a>>),
    Null,
}

fn box_x_compare(a: &Box<dyn Hittable + Sync>, b: &Box<dyn Hittable + Sync>) -> Ordering {
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
        },
        _ => Ordering::Less
    }
}

fn box_y_compare(a: &Box<dyn Hittable + Sync>, b: &Box<dyn Hittable + Sync>) -> Ordering {
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
        },
        _ => Ordering::Less
    }
}

fn box_z_compare(a: &Box<dyn Hittable + Sync>, b: &Box<dyn Hittable + Sync>) -> Ordering {
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
        },
        _ => Ordering::Less
    }
}

// fn quickSort(list: &[Box<dyn Hittable + Sync>], compare: fn(&Box<dyn Hittable + Sync>, &Box<dyn Hittable + Sync>) -> Ordering) -> Vec<Box<dyn Hittable + Sync>> {

// }

pub fn createTree<'a>(list: &'a [Box<dyn Hittable + Sync>], t0: f32, t1: f32) -> BinaryTree<'a> {
    let mut rng = rand::thread_rng();
    let axis: usize = rng.gen_range(0, 3);
    // match axis {
    //     0 => list.sort_by(box_x_compare),
    //     1 => list.sort_by(box_y_compare),
    //     _ => list.sort_by(box_z_compare),
    // };
    let list_length = list.len();

    match list_length {
        1 => BinaryTree::Leaf(&list[0]),
        _ => {
            let left = Box::new(BvhTree::new(&list[0..list_length / 2], t0, t1));
            let right = Box::new(BvhTree::new(&list[list_length / 2..list_length], t0, t1));
            BinaryTree::Node(left, right)
        }
    }
}

pub struct BvhTree<'a> {
    binary_tree: BinaryTree<'a>,
}

impl<'a> BvhTree<'a> {
    pub fn new(list: &'a [Box<dyn Hittable + Sync>], t0: f32, t1: f32) -> Self {
        BvhTree {
            binary_tree: createTree(list, t0, t1),
        }
    }
}

impl<'a> Hittable for BvhTree<'a> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<(HitRecord, &Material)> {
        match &self.binary_tree {
            BinaryTree::Leaf(hittable) => hittable.hit(ray, t_min, t_max),
            BinaryTree::Node(left, right) => {
                let left_rec = match left.hit(ray, t_min, t_max) {
                    Some((lr, lm)) => Some((lr, lm)),
                    None => None,
                };
                let right_rec = match right.hit(ray, t_min, t_max) {
                    Some((rr, rm)) => Some((rr, rm)),
                    None => return None,
                };

                return match (left_rec, right_rec) {
                    (Some((lr, lm)), Some((rr, rm))) => {
                        if lr.t < rr.t {
                            Some((lr, &lm))
                        } else {
                            Some((rr, &rm))
                        }
                    }
                    (Some((lr, lm)), None) => Some((lr, &lm)),
                    (None, Some((rr, rm))) => Some((rr, &rm)),
                    (None, None) => None,
                };
            },
            BinaryTree::Null => None,
        }
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb> {
        match &self.binary_tree {
            BinaryTree::Leaf(hittable) => hittable.bounding_box(t0, t1),
            BinaryTree::Node(left, right) => {
                let left_aabb: Option<Aabb> = left.bounding_box(t0, t1);
                let right_aabb: Option<Aabb> = right.bounding_box(t0, t1);
                match (left_aabb, right_aabb) {
                    (Some(lbb), Some(rbb)) => Some(surrounding_box(&lbb, &rbb)),
                    (Some(lbb), None) => Some(lbb),
                    (None, Some(rbb)) => Some(rbb),
                    (None, None) => None,
                }
            },
            BinaryTree::Null => None,
        }
    }
}
