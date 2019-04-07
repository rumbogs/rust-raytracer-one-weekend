use super::ray::Ray;
use super::vector3::{cross, dot, unit_vector, Vector3};
use rand::Rng;
use std::f32::consts;

fn random_in_unit_disk() -> Vector3 {
    let mut p: Vector3;
    let mut rng = rand::thread_rng();
    loop {
        p = 2.0 * Vector3::new(rng.gen::<f32>(), rng.gen::<f32>(), 0.0)
            - Vector3::new(1.0, 1.0, 0.0);
        if dot(p, p) < 1.0 {
            return p;
        }
    }
}

pub struct Camera {
    origin: Vector3,
    lower_left_corner: Vector3,
    horizontal: Vector3,
    vertical: Vector3,
    u: Vector3,
    v: Vector3,
    w: Vector3,
    lens_radius: f32,
    time0: f32,
    time1: f32,
}

impl Camera {
    /**
     * vup: top to bottom in degrees
     */
    pub fn new(
        lookfrom: Vector3,
        lookat: Vector3,
        vup: Vector3,
        vfov: f32,
        aspect: f32,
        aperture: f32,
        focus_dist: f32,
        time0: f32,
        time1: f32,
    ) -> Camera {
        let theta: f32 = vfov * consts::PI / 180.0;
        let half_height: f32 = (theta / 2.0).tan();
        let half_width: f32 = aspect * half_height;
        let w = unit_vector(lookfrom - lookat);
        let u = unit_vector(cross(vup, w));
        let v = cross(w, u);
        let origin: Vector3 = lookfrom;

        Camera {
            origin,
            lower_left_corner: origin
                - half_width * focus_dist * u
                - half_height * focus_dist * v
                - focus_dist * w,
            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height * focus_dist * v,
            u,
            v,
            w,
            lens_radius: aperture / 2.0,
            time0,
            time1,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let mut rng = rand::thread_rng();
        let rd: Vector3 = self.lens_radius * random_in_unit_disk();
        let offset: Vector3 = self.u * rd.x() + self.v * rd.y();
        let time: f32 = self.time0 + rng.gen::<f32>() * (self.time1 - self.time0);
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
            time,
        )
    }
}
