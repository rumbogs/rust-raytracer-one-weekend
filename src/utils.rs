use super::vector3::Vector3;
use super::object::Object;
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
