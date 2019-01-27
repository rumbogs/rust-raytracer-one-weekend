extern crate crossbeam;
extern crate image;
extern crate num_cpus;
extern crate rand;

use rand::Rng;
use std::time::Instant;

mod camera;
mod material;
mod object;
mod object_list;
mod ray;
mod sphere;
mod vector3;

use camera::Camera;
use material::{Material, MaterialType, Scatterable};
use object::Hittable;
use object_list::ObjectList;
use ray::Ray;
use sphere::Sphere;
use vector3::{unit_vector, Vector3};

fn random_in_unit_sphere() -> Vector3 {
    let mut p: Vector3;
    loop {
        let mut rng = rand::thread_rng();
        p = 2.0 * Vector3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>())
            - Vector3::new(1.0, 1.0, 1.0); // -1 -> 1
        if p.squared_length() < 1.0 {
            break;
        }
    }
    p
}

fn color(r: &Ray, world: &ObjectList, depth: usize) -> Vector3 {
    // some of the rays hit at 0.00000001 instead of 0.0
    // so ignore those to remove noise
    match world.hit(r, 0.001, std::f32::MAX) {
        Some((rec, material)) => {
            if depth < 50 {
                match material.scatter(r, rec) {
                    Some((attenuation, scattered)) => {
                        attenuation * color(&scattered, world, depth + 1)
                    }
                    None => Vector3::new(0.0, 0.0, 0.0),
                }
            } else {
                Vector3::new(0.0, 0.0, 0.0)
            }
        }
        None => {
            let unit_direction = unit_vector(r.direction());
            let t: f32 = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
        }
    }
}

fn main() {
    let cpu_num = num_cpus::get();
    let now = Instant::now();
    let width = 200;
    let height = 100;
    let thread_rows = height / cpu_num;
    let antialiasing_sensitivity: u32 = 100;
    let camera: &Camera = &Camera::new(
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(-2.0, -1.0, -1.0),
        Vector3::new(4.0, 0.0, 0.0),
        Vector3::new(0.0, 2.0, 0.0),
    );

    // TODO: check implementation performance (Box)
    let world = &ObjectList::new(vec![
        Box::new(Sphere::new(
            Vector3::new(0.0, 0.0, -1.0),
            0.5,
            Material::new(
                MaterialType::Lambertian,
                Vector3::new(0.8, 0.3, 0.3),
                0.0,
                0.0,
            ),
        )),
        Box::new(Sphere::new(
            Vector3::new(0.0, -100.5, -1.0),
            100.0,
            Material::new(
                MaterialType::Lambertian,
                Vector3::new(0.8, 0.8, 0.0),
                0.0,
                0.0,
            ),
        )),
        Box::new(Sphere::new(
            Vector3::new(1.0, 0.0, -1.0),
            0.5,
            Material::new(MaterialType::Metal, Vector3::new(0.8, 0.6, 0.2), 0.3, 0.0),
        )),
        Box::new(Sphere::new(
            Vector3::new(-1.0, 0.0, -1.0),
            0.5,
            Material::new(
                MaterialType::Dielectric,
                Vector3::new(1.0, 1.0, 1.0),
                0.0,
                1.5,
            ),
        )),
    ]);

    let mut pixels = vec![Vector3::new(0.0, 0.0, 0.0); width * height];
    let rows: Vec<&mut [Vector3]> = pixels.chunks_mut(thread_rows * width).collect();

    match crossbeam::scope(|spawner| {
        for (i, row) in rows.into_iter().enumerate() {
            spawner.spawn(move |_| {
                for y in 0..thread_rows {
                    for x in 0..width {
                        let mut col: Vector3 = Vector3::new(0.0, 0.0, 0.0);
                        // rows and y need to be calculated from bottom up
                        // but buffer needs to written from top down
                        // also subtract one because the for loop isn't inclusive
                        // on the right hand side
                        let inverted_y = thread_rows - y - 1;
                        let inverted_row = ((cpu_num - i - 1) * thread_rows + inverted_y) as f32;
                        let mut rng = rand::thread_rng();

                        // this shoots rays around the object
                        // edge using a random offset
                        // and computes a color average
                        for _ in 0..antialiasing_sensitivity {
                            let u = (x as f32 + rng.gen::<f32>()) / width as f32;
                            let v = (inverted_row + rng.gen::<f32>()) / height as f32;
                            let r = camera.get_ray(u, v);
                            col += color(&r, &world, 0);
                        }
                        col /= antialiasing_sensitivity as f32;
                        // remove the gamma of 2 from the color (raise to power of 1/2)
                        col = Vector3::new(col[0].sqrt(), col[1].sqrt(), col[2].sqrt());
                        let ir = (255.99 * &col[0]) as u8;
                        let ig = (255.99 * &col[1]) as u8;
                        let ib = (255.99 * &col[2]) as u8;
                        // save buffer values
                        let buffer_pos = y * width + x;
                        row[buffer_pos][0] = ir as f32;
                        row[buffer_pos][1] = ig as f32;
                        row[buffer_pos][2] = ib as f32;
                    }
                }
            });
        }
    }) {
        Ok(()) => println!(
            "Finished in {}s {}ms",
            now.elapsed().as_secs(),
            (now.elapsed().subsec_nanos() / 1_000_000) as u64
        ),
        Err(err) => println!("{:?}", err),
    };

    let mut buffer = vec![];

    for pixel in pixels {
        buffer.push(pixel[0] as u8);
        buffer.push(pixel[1] as u8);
        buffer.push(pixel[2] as u8);
    }

    image::save_buffer("1.png", &buffer, width as u32, height as u32, image::RGB(8)).unwrap();
}
