extern crate image;
extern crate rand;

use rand::Rng;

mod camera;
mod object;
mod object_list;
mod ray;
mod sphere;
mod vector3;

use camera::Camera;
use object::Object;
use object_list::ObjectList;
use ray::Ray;
use sphere::Sphere;
use vector3::{unit_vector, Vector3};

fn color(r: &Ray, world: &ObjectList) -> Vector3 {
    match world.hit(r, 0.0, std::f32::MAX) {
        Some(rec) => {
            0.5 * Vector3::new(
                rec.normal.x() + 1.0,
                rec.normal.y() + 1.0,
                rec.normal.z() + 1.0,
            )
        }
        None => {
            let unit_direction = unit_vector(r.direction());
            let t: f32 = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
        }
    }
}

fn main() {
    let width: u32 = 200;
    let height: u32 = 100;
    let antialiasing_sensitivity: u32 = 100;
    let camera: Camera = Camera::new(
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(-2.0, -1.0, -1.0),
        Vector3::new(4.0, 0.0, 0.0),
        Vector3::new(0.0, 2.0, 0.0),
    );

    // TODO: check implementation performance (Box)
    let world: ObjectList = ObjectList::new(vec![
        Box::new(Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0)),
    ]);

    let mut imgbuf = image::ImageBuffer::new(width, height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // x is [0..width]
        // y is [0..height]
        // red goes from left to right
        // green goes from bottom to top
        let mut col: Vector3 = Vector3::new(0.0, 0.0, 0.0);
        let inverted_y = height - y;
        let mut rng = rand::thread_rng();

        // this shoots rays around the object
        // edge using a random offset
        // and computes a color average
        for _ in 0..antialiasing_sensitivity {
            let u = (x as f32 + rng.gen::<f32>()) / width as f32;
            let v = (inverted_y as f32 + rng.gen::<f32>()) / height as f32;
            let r = camera.get_ray(u, v);
            col += color(&r, &world);
        }
        col /= antialiasing_sensitivity as f32;
        let ir = (255.99 * &col[0]) as u8;
        let ig = (255.99 * &col[1]) as u8;
        let ib = (255.99 * &col[2]) as u8;
        *pixel = image::Rgb([ir, ig, ib]);
    }

    imgbuf.save("1.png").unwrap();
}
