extern crate crossbeam;
extern crate image;
extern crate num_cpus;
extern crate rand;

use rand::Rng;
use std::time::Instant;

mod aabb;
mod bvh_node;
mod camera;
mod material;
mod moving_sphere;
mod object;
mod object_list;
mod qsort;
mod ray;
mod sphere;
mod texture;
mod vector3;

use camera::Camera;
use material::{Material, MaterialType, Scatterable};
use moving_sphere::MovingSphere;
use object::Hittable;
use object_list::ObjectList;
use ray::Ray;
use sphere::Sphere;
use texture::{CheckerTexture, ConstantTexture};
use vector3::{unit_vector, Vector3};

fn random_scene() -> ObjectList {
    let n: usize = 500;
    let mut object_list: Vec<Box<Hittable + Sync>> = Vec::with_capacity(n + 1);
    let mut rng = rand::thread_rng();

    object_list.push(Box::new(Sphere::new(
        Vector3::new(0.0, -1000.0, 0.0),
        1000.0,
        Material::new(
            MaterialType::Lambertian,
            Box::new(CheckerTexture::new(
                Box::new(ConstantTexture::new(Vector3::new(0.2, 0.3, 0.1))),
                Box::new(ConstantTexture::new(Vector3::new(0.9, 0.9, 0.9))),
            )),
            0.0,
            0.0,
        ),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f32 = rng.gen::<f32>();
            let center: Vector3 = Vector3::new(
                a as f32 + 0.9 * rng.gen::<f32>(),
                0.2,
                b as f32 + 0.9 * rng.gen::<f32>(),
            );
            if (center - Vector3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    object_list.push(Box::new(MovingSphere::new(
                        center,
                        center + Vector3::new(0.0, 0.5 * rng.gen::<f32>(), 0.0),
                        0.0,
                        1.0,
                        0.2,
                        Material::new(
                            MaterialType::Lambertian,
                            Box::new(ConstantTexture::new(Vector3::new(
                                rng.gen::<f32>() * rng.gen::<f32>(),
                                rng.gen::<f32>() * rng.gen::<f32>(),
                                rng.gen::<f32>() * rng.gen::<f32>(),
                            ))),
                            0.0,
                            0.0,
                        ),
                    )));
                } else if choose_mat < 0.95 {
                    object_list.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Material::new(
                            MaterialType::Metal,
                            Box::new(ConstantTexture::new(Vector3::new(
                                0.5 * (1.0 + rng.gen::<f32>()),
                                0.5 * (1.0 + rng.gen::<f32>()),
                                0.5 * (1.0 + rng.gen::<f32>()),
                            ))),
                            0.5 * rng.gen::<f32>(),
                            0.0,
                        ),
                    )));
                } else {
                    object_list.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Material::new(
                            MaterialType::Dielectric,
                            Box::new(ConstantTexture::new(Vector3::new(0.0, 0.0, 0.0))),
                            0.0,
                            1.5,
                        ),
                    )));
                }
            }
        }
    }

    object_list.push(Box::new(Sphere::new(
        Vector3::new(0.0, 1.0, 0.0),
        1.0,
        Material::new(
            MaterialType::Dielectric,
            Box::new(ConstantTexture::new(Vector3::new(0.0, 0.0, 0.0))),
            0.0,
            1.5,
        ),
    )));
    object_list.push(Box::new(Sphere::new(
        Vector3::new(-4.0, 1.0, 0.0),
        1.0,
        Material::new(
            MaterialType::Lambertian,
            Box::new(ConstantTexture::new(Vector3::new(0.4, 0.2, 0.1))),
            0.0,
            0.0,
        ),
    )));
    object_list.push(Box::new(Sphere::new(
        Vector3::new(4.0, 1.0, 0.0),
        1.0,
        Material::new(
            MaterialType::Metal,
            Box::new(ConstantTexture::new(Vector3::new(0.7, 0.6, 0.5))),
            0.0,
            0.0,
        ),
    )));

    ObjectList::new(object_list)
}

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
    let cpu_num = num_cpus::get() - 1; // leave some for the rest of the processes
    let now = Instant::now();
    let width = 640;
    let height = 480;
    // larger makes blur/shadows/antialias smoother
    let smoothness: u32 = 10;
    // use one less thread for exact division
    // the last one will have less pixels to calculate
    let thread_rows = height / cpu_num + 1;
    let lookfrom = Vector3::new(11.0, 4.0, 5.0);
    let lookat = Vector3::new(0.0, 0.0, 0.0);
    let camera: &Camera = &Camera::new(
        lookfrom,
        lookat,
        Vector3::new(0.0, 1.0, 0.0),
        36.0,
        width as f32 / height as f32,
        0.0,
        10.0,
        0.0,
        1.0,
    );

    let world = &random_scene();

    let mut pixels = vec![Vector3::new(0.0, 0.0, 0.0); width * height];
    let rows: Vec<&mut [Vector3]> = pixels.chunks_mut(thread_rows * width).collect();
    let rows_length = rows.len();

    match crossbeam::scope(|spawner| {
        for (i, row) in rows.into_iter().enumerate() {
            spawner.spawn(move |_| {
                // when the height doesn't divide to the thread no
                // the last row doesn't have the same number of
                // pixels so limit max number of rows to remainder
                let mut max_thread_rows = thread_rows;
                if i == rows_length - 1 && height % thread_rows != 0 {
                    max_thread_rows = height % thread_rows;
                }

                for y in 0..max_thread_rows {
                    for x in 0..width {
                        let mut col: Vector3 = Vector3::new(0.0, 0.0, 0.0);
                        // rows and y need to be calculated from bottom up
                        // but buffer needs to be written from top down
                        // also subtract one because the for loop isn't inclusive
                        // on the right hand side
                        let inverted_y = thread_rows - y - 1;
                        // println!("{}", i);

                        let inverted_row = ((cpu_num - i - 1) * thread_rows + inverted_y) as f32;
                        let mut rng = rand::thread_rng();

                        // this shoots rays around the object
                        // edge using a random offset
                        // and computes a color average
                        for _ in 0..smoothness {
                            let u = (x as f32 + rng.gen::<f32>()) / width as f32;
                            let v = (inverted_row + rng.gen::<f32>()) / height as f32;
                            let r = camera.get_ray(u, v);
                            col += color(&r, &world, 0);
                        }
                        col /= smoothness as f32;
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
            "Finished in {}m {}s {}ms",
            (now.elapsed().as_secs() / 60) as u64,
            now.elapsed().as_secs() % 60,
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
