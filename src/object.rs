use super::aabb::{surrounding_box, Aabb};
use super::binary_tree::BinaryTree;
use super::material::Material;
use super::ray::Ray;
use super::utils::{center_at_time, get_sphere_uv};
use super::vector3::{dot, Vector3};

pub struct HitRecord {
    pub u: f32,
    pub v: f32,
    pub t: f32,
    pub p: Vector3,
    pub normal: Vector3,
}

impl HitRecord {
    pub fn new(u: f32, v: f32, t: f32, p: Vector3, normal: Vector3) -> HitRecord {
        HitRecord { u, v, t, p, normal }
    }
}

pub enum Object {
    XYRect {
        x0: f32,
        x1: f32,
        y0: f32,
        y1: f32,
        k: f32,
        material: Material,
        flip_normals: bool,
    },
    XZRect {
        x0: f32,
        x1: f32,
        z0: f32,
        z1: f32,
        k: f32,
        material: Material,
        flip_normals: bool,
    },
    YZRect {
        y0: f32,
        y1: f32,
        z0: f32,
        z1: f32,
        k: f32,
        material: Material,
        flip_normals: bool,
    },
    Sphere {
        center: Vector3,
        radius: f32,
        material: Material,
        flip_normals: bool,
    },
    MovingSphere {
        center0: Vector3,
        center1: Vector3,
        time0: f32,
        time1: f32,
        radius: f32,
        material: Material,
        flip_normals: bool,
    },
    ObjectList {
        list: Vec<Box<Object>>,
    },
    BvhTree {
        binary_tree: BinaryTree,
        aabb: Aabb,
    },
}

impl Object {
    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<(HitRecord, &Material)> {
        match self {
            Object::XYRect {
                x0,
                x1,
                y0,
                y1,
                k,
                material,
                flip_normals,
            } => {
                let ray_origin = r.origin();
                let ray_direction = r.direction();
                let t: f32 = (k - ray_origin.z()) / r.direction().z();
                if t < t_min || t > t_max {
                    return None;
                }
                let x: f32 = ray_origin.x() + t * ray_direction.x();
                let y: f32 = ray_origin.y() + t * ray_direction.y();
                if x < *x0 || x > *x1 || y < *y0 || y > *y1 {
                    return None;
                }
                let u = (x - x0) / (x1 - x0);
                let v = (y - y0) / (y1 - y0);
                let p = r.point_at_parameter(t);
                let mut normal = Vector3::new(0.0, 0.0, 1.0);
                if *flip_normals {
                    normal = -normal;
                }
                Some((HitRecord::new(u, v, t, p, normal), &material))
            }
            Object::XZRect {
                x0,
                x1,
                z0,
                z1,
                k,
                material,
                flip_normals,
            } => {
                let ray_origin = r.origin();
                let ray_direction = r.direction();
                let t: f32 = (k - ray_origin.y()) / r.direction().y();
                if t < t_min || t > t_max {
                    return None;
                }
                let x: f32 = ray_origin.x() + t * ray_direction.x();
                let z: f32 = ray_origin.z() + t * ray_direction.z();
                if x < *x0 || x > *x1 || z < *z0 || z > *z1 {
                    return None;
                }
                let u = (x - x0) / (x1 - x0);
                let v = (z - z0) / (z1 - z0);
                let p = r.point_at_parameter(t);
                let mut normal = Vector3::new(0.0, 1.0, 0.0);
                if *flip_normals {
                    normal = -normal;
                }
                Some((HitRecord::new(u, v, t, p, normal), &material))
            }
            Object::YZRect {
                y0,
                y1,
                z0,
                z1,
                k,
                material,
                flip_normals,
            } => {
                let ray_origin = r.origin();
                let ray_direction = r.direction();
                let t: f32 = (k - ray_origin.x()) / r.direction().x();
                if t < t_min || t > t_max {
                    return None;
                }
                let y: f32 = ray_origin.y() + t * ray_direction.y();
                let z: f32 = ray_origin.z() + t * ray_direction.z();
                if y < *y0 || y > *y1 || z < *z0 || z > *z1 {
                    return None;
                }
                let u = (y - y0) / (y1 - y0);
                let v = (z - z0) / (z1 - z0);
                let p = r.point_at_parameter(t);
                let mut normal = Vector3::new(1.0, 0.0, 0.0);
                if *flip_normals {
                    normal = -normal;
                }
                Some((HitRecord::new(u, v, t, p, normal), &material))
            }
            Object::Sphere {
                center,
                radius,
                material,
                flip_normals,
            } => {
                let oc: Vector3 = r.origin() - *center;
                let a: f32 = dot(r.direction(), r.direction());
                let b: f32 = dot(oc, r.direction());
                let c: f32 = dot(oc, oc) - radius * radius;
                let discriminant: f32 = b * b - a * c;
                if discriminant > 0.0 {
                    let mut temp: f32 = (-b - (b * b - a * c).sqrt()) / a;
                    if temp < t_max && temp > t_min {
                        let t = temp;
                        let p = r.point_at_parameter(t);
                        let mut normal = (p - *center) / *radius;
                        if *flip_normals {
                            normal = -normal;
                        }
                        let (u, v) = get_sphere_uv(&((p - *center) / *radius));
                        return Some((HitRecord::new(u, v, t, p, normal), &material));
                    }
                    temp = (-b + (b * b - a * c).sqrt()) / a;
                    if temp < t_max && temp > t_min {
                        let t = temp;
                        let p = r.point_at_parameter(t);
                        let mut normal = (p - *center) / *radius;
                        if *flip_normals {
                            normal = -normal;
                        }
                        let (u, v) = get_sphere_uv(&((p - *center) / *radius));
                        return Some((HitRecord::new(u, v, t, p, normal), &material));
                    }
                }
                None
            }
            Object::MovingSphere {
                center0,
                center1,
                time0,
                time1,
                radius,
                material,
                flip_normals,
            } => {
                let center = center_at_time(r.time, center0, center1, time0, time1);
                let oc: Vector3 = r.origin() - center;
                let a: f32 = dot(r.direction(), r.direction());
                let b: f32 = dot(oc, r.direction());
                let c: f32 = dot(oc, oc) - radius * radius;
                let discriminant: f32 = b * b - a * c;
                if discriminant > 0.0 {
                    let mut temp: f32 = (-b - (b * b - a * c).sqrt()) / a;
                    if temp < t_max && temp > t_min {
                        let t = temp;
                        let p = r.point_at_parameter(t);
                        let mut normal = (p - center) / *radius;
                        if *flip_normals {
                            normal = -normal;
                        }
                        let (u, v) = get_sphere_uv(&((p - center) / *radius));
                        return Some((HitRecord::new(u, v, t, p, normal), &material));
                    }
                    temp = (-b + (b * b - a * c).sqrt()) / a;
                    if temp < t_max && temp > t_min {
                        let t = temp;
                        let p = r.point_at_parameter(t);
                        let mut normal = (p - center) / *radius;
                        if *flip_normals {
                            normal = -normal;
                        }
                        let (u, v) = get_sphere_uv(&((p - center) / *radius));
                        return Some((HitRecord::new(u, v, t, p, normal), &material));
                    }
                }
                None
            }
            Object::ObjectList { list } => {
                let mut closest_so_far: f32 = t_max;
                let mut hit_objects: Vec<(HitRecord, &Material)> = vec![];

                for element in list.iter() {
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
            Object::BvhTree { binary_tree, aabb } => match &binary_tree {
                BinaryTree::Leaf(hittable) => hittable.hit(r, t_min, t_max),
                BinaryTree::Node(left, right) => {
                    if aabb.hit(r, t_min, t_max) {
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
            },
        }
    }

    pub fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb> {
        match self {
            Object::XYRect {
                x0,
                x1,
                y0,
                y1,
                k,
                material,
                flip_normals,
            } => Some(Aabb::new(
                Vector3::new(*x0, *y0, k - 0.0001),
                Vector3::new(*x1, *y1, k + 0.0001),
            )),
            Object::XZRect {
                x0,
                x1,
                z0,
                z1,
                k,
                material,
                flip_normals,
            } => Some(Aabb::new(
                Vector3::new(*x0, k - 0.0001, *z0),
                Vector3::new(*x1, k + 0.0001, *z1),
            )),
            Object::YZRect {
                y0,
                y1,
                z0,
                z1,
                k,
                material,
                flip_normals,
            } => Some(Aabb::new(
                Vector3::new(k - 0.0001, *y0, *z0),
                Vector3::new(k + 0.0001, *y1, *z1),
            )),
            Object::Sphere {
                center,
                radius,
                material: _,
                flip_normals,
            } => Some(Aabb::new(
                *center - Vector3::new(*radius, *radius, *radius),
                *center + Vector3::new(*radius, *radius, *radius),
            )),
            Object::MovingSphere {
                center0,
                center1,
                time0: _,
                time1: _,
                radius,
                material: _,
                flip_normals,
            } => Some(surrounding_box(
                &Aabb::new(
                    *center0 - Vector3::new(*radius, *radius, *radius),
                    *center0 + Vector3::new(*radius, *radius, *radius),
                ),
                &Aabb::new(
                    *center1 - Vector3::new(*radius, *radius, *radius),
                    *center1 + Vector3::new(*radius, *radius, *radius),
                ),
            )),
            Object::ObjectList { list } => {
                if list.is_empty() {
                    return None;
                }

                let mut hit_bbox: Aabb;

                match list[0].bounding_box(t0, t1) {
                    Some(first_bbox) => {
                        hit_bbox = first_bbox;
                        for element in list.iter() {
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
            Object::BvhTree {
                binary_tree: _,
                aabb,
            } => Some(*aabb),
        }
    }
}
