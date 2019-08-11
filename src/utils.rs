use super::material::Material;
use super::object::Object;
use super::vector3::Vector3;
use super::aabb::Aabb;
use image::{DynamicImage, GenericImageView};
use std::f32::consts::PI;

pub fn center_at_time(
    time: f32,
    center0: &Vector3,
    center1: &Vector3,
    time0: &f32,
    time1: &f32,
) -> Vector3 {
    ((time - time0) / (time1 - time0)) * (center1 - center0) + *center0
}

pub fn get_sphere_uv(p: &Vector3) -> (f32, f32) {
    let phi: f32 = p.z().atan2(p.x());
    let theta: f32 = p.y().asin();
    (1.0 - (phi + PI) / (2.0 * PI), (theta + PI / 2.0) / PI)
}

pub fn create_cube(pmin: Vector3, pmax: Vector3, material: Material) -> Object {
    let mut list: Vec<Box<Object>> = Vec::with_capacity(6);
    list.push(Box::new(Object::XYRect {
        x0: pmin.x(),
        x1: pmax.x(),
        y0: pmin.y(),
        y1: pmax.y(),
        k: pmax.z(),
        material: material.clone(),
    }));
    list.push(Box::new(Object::FlipNormals(Box::new(Object::XYRect {
        x0: pmin.x(),
        x1: pmax.x(),
        y0: pmin.y(),
        y1: pmax.y(),
        k: pmin.z(),
        material: material.clone(),
    }))));
    list.push(Box::new(Object::XZRect {
        x0: pmin.x(),
        x1: pmax.x(),
        z0: pmin.z(),
        z1: pmax.z(),
        k: pmax.y(),
        material: material.clone(),
    }));
    list.push(Box::new(Object::FlipNormals(Box::new(Object::XZRect {
        x0: pmin.x(),
        x1: pmax.x(),
        z0: pmin.z(),
        z1: pmax.z(),
        k: pmin.y(),
        material: material.clone(),
    }))));
    list.push(Box::new(Object::YZRect {
        y0: pmin.y(),
        y1: pmax.y(),
        z0: pmin.z(),
        z1: pmax.z(),
        k: pmax.x(),
        material: material.clone(),
    }));
    list.push(Box::new(Object::FlipNormals(Box::new(Object::YZRect {
        y0: pmin.y(),
        y1: pmax.y(),
        z0: pmin.z(),
        z1: pmax.z(),
        k: pmin.x(),
        material: material.clone(),
    }))));
    let faces: Box<Object> = Box::new(Object::ObjectList(list));
    Object::Cube {
        pmin,
        pmax,
        material: material,
        faces,
    }
}

pub fn create_rotatey(object: Object, angle: f32) -> Object {
    let radians: f32 = (PI / 180.0) * angle;
    let sin_theta = radians.sin();
    let cos_theta = radians.cos();
    let aabb = object.bounding_box(0.0, 1.0);
    let mut min = Vector3::new(std::f32::MAX, std::f32::MAX, std::f32::MAX);
    let mut max = Vector3::new(-std::f32::MAX, -std::f32::MAX, -std::f32::MAX);
    match aabb {
        Some(bbox) => {
            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let x = i as f32 * bbox.max.x() + (1 - i) as f32 * bbox.min.x();
                        let y = j as f32  * bbox.max.y() + (1 - j) as f32 * bbox.min.y();
                        let z = k as f32 * bbox.max.z() + (1 - k) as f32 * bbox.min.z();
                        let newx: f32 = cos_theta * x + sin_theta * z;
                        let newz: f32 = -sin_theta * x + cos_theta * z;
                        let tester: Vector3 = Vector3::new(newx, y, newz);
                        for c in 0..3 {
                            if tester[c] > max[c] {
                                max[c] = tester[c];
                            }
                            if tester[c] < min[c] {
                                min[c] = tester[c];
                            }
                        }
                    }
                }
            }
        },
        None => {},
    };
    Object::RotateY {
        object: Box::new(object),
        sin_theta,
        cos_theta,
        aabb: Aabb::new(min, max),
    }
}
