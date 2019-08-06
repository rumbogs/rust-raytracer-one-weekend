use rand::Rng;
use std::cmp::Ordering;

use super::aabb::surrounding_box;
use super::object::Object;

pub enum BinaryTree {
    Leaf(Box<Object>),
    Node(Box<Object>, Box<Object>),
}

fn box_x_compare(a: &Box<Object>, b: &Box<Object>) -> Ordering {
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

fn box_y_compare(a: &Box<Object>, b: &Box<Object>) -> Ordering {
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

fn box_z_compare(a: &Box<Object>, b: &Box<Object>) -> Ordering {
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

pub fn create_binary_tree(mut list: Vec<Box<Object>>, t0: f32, t1: f32) -> Object {
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

            return Object::BvhTree {
                binary_tree: BinaryTree::Leaf(hittable),
                aabb: match bbox {
                    Some(bb) => bb,
                    None => panic!["No bounding box"],
                },
            };
        }
        _ => {
            let mut vec1: Vec<Box<Object>> = Vec::with_capacity(list_length / 2 + 1);
            let mut vec2: Vec<Box<Object>> = Vec::with_capacity(list_length / 2);

            for (i, el) in list.into_iter().enumerate() {
                if i < list_length / 2 {
                    vec1.push(el);
                } else {
                    vec2.push(el);
                }
            }
            let left = create_binary_tree(vec1, t0, t1);
            let right = create_binary_tree(vec2, t0, t1);
            let left_bbox = left.bounding_box(t0, t1);
            let right_bbox = right.bounding_box(t0, t1);
            let aabb = match (left_bbox, right_bbox) {
                (Some(lbb), Some(rbb)) => surrounding_box(&lbb, &rbb),
                (Some(lbb), None) => lbb,
                (None, Some(rbb)) => rbb,
                (None, None) => panic!["No bounding box"],
            };
            return Object::BvhTree {
                binary_tree: BinaryTree::Node(Box::new(left), Box::new(right)),
                aabb,
            };
        }
    }
}
