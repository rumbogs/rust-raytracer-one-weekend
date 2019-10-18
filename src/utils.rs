use rand::Rng;
use std::f32::consts;

use super::vector3::{unit_vector, Vector3};

pub fn random_on_unit_sphere() -> Vector3 {
    let mut p: Vector3;
    loop {
        let mut rng = rand::thread_rng();
        p = 2.0 * Vector3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>())
            - Vector3::new(1.0, 1.0, 1.0); // -1 -> 1
        if p.squared_length() < 1.0 {
            break;
        }
    }
    unit_vector(p)
}

pub fn random_cosine_direction() -> Vector3 {
    let mut rng = rand::thread_rng();
    let r1 = rng.gen::<f32>();
    let r2 = rng.gen::<f32>();
    let z = (1.0 - r2).sqrt();
    let phi = 2.0 * consts::PI * r1;
    let x = phi.cos() * 2.0 * r2.sqrt();
    let y = phi.sin() * 2.0 * r2.sqrt();
    Vector3::new(x, y, z)
}

pub fn random_to_sphere(radius: f32, distance_squared: f32) -> Vector3 {
    let mut rng = rand::thread_rng();
    let r1 = rng.gen::<f32>();
    let r2 = rng.gen::<f32>();
    let z = 1.0 + r2 * ((1.0 - radius.powf(2.0) / distance_squared).sqrt() - 1.0);
    let phi = 2.0 * consts::PI * r1;
    let x = phi.cos() * (1.0 - z.powf(2.0)).sqrt();
    let y = phi.sin() * (1.0 - z.powf(2.0)).sqrt();
    Vector3::new(x, y, z)
}

pub fn de_nan(c: Vector3) -> Vector3 {
    let mut temp = c;
    if temp[0].is_nan() {
        temp[0] = 0.0;
    }
    if temp[1].is_nan() {
        temp[1] = 0.0;
    }
    if temp[2].is_nan() {
        temp[2] = 0.0;
    }
    temp
}

pub fn clamp(input: f32, min: f32, max: f32) -> f32 {
    if input < min {
        min
    } else if input > max {
        max
    } else {
        input
    }
}
