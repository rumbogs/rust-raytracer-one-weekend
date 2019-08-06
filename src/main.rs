extern crate crossbeam;
extern crate image;
extern crate num_cpus;
extern crate rand;

use image::GenericImageView;
use rand::Rng;
use std::time::Instant;

mod aabb;
mod binary_tree;
mod camera;
mod material;
mod object;
mod perlin;
mod ray;
mod texture;
mod utils;
mod vector3;

use binary_tree::create_binary_tree;
use camera::Camera;
use material::Material;
use object::Object;
use perlin::Perlin;
use ray::Ray;
use texture::Texture;
use vector3::{unit_vector, Vector3};

fn random_scene() -> Vec<Box<Object>> {
    let n: usize = 500;
    let mut object_list: Vec<Box<Object>> = Vec::with_capacity(n + 1);

    object_list.push(Box::new(Object::Sphere {
        center: Vector3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Material::Lambertian {
            albedo: Texture::ConstantTexture {
                color: Vector3::new(0.7, 0.6, 0.5),
            },
        },
    }));
    // object_list.push(Box::new(Object::Sphere {
    //     center: Vector3::new(0.0, 1.0, 0.0),
    //     radius: 1.0,
    //     material: Material::Lambertian {
    //         albedo: Texture::NoiseTexture { noise: Perlin::new(), scale: 5.0 },
    //     },
    // }));
    let img = image::open("pug.jpg").unwrap();
    let (nx, ny) = img.dimensions();

    object_list.push(Box::new(Object::Sphere {
        center: Vector3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Lambertian {
            albedo: Texture::ImageTexture { img },
        },
    }));

    object_list
}

fn random_scene2() -> Vec<Box<Object>> {
    let n: usize = 500;
    let mut object_list: Vec<Box<Object>> = Vec::with_capacity(n + 1);
    let mut rng = rand::thread_rng();

    for a in -10..10 {
        for b in -10..10 {
            let choose_mat: f32 = rng.gen::<f32>();
            let center: Vector3 = Vector3::new(
                a as f32 + 0.9 * rng.gen::<f32>(),
                0.2,
                b as f32 + 0.9 * rng.gen::<f32>(),
            );
            if (center - Vector3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    object_list.push(Box::new(Object::MovingSphere {
                        center0: center,
                        center1: center + Vector3::new(0.0, 0.5 * rng.gen::<f32>(), 0.0),
                        time0: 0.0,
                        time1: 1.0,
                        radius: 0.2,
                        material: Material::Lambertian {
                            albedo: Texture::ConstantTexture {
                                color: Vector3::new(
                                    rng.gen::<f32>() * rng.gen::<f32>(),
                                    rng.gen::<f32>() * rng.gen::<f32>(),
                                    rng.gen::<f32>() * rng.gen::<f32>(),
                                ),
                            },
                        },
                    }));
                } else if choose_mat < 0.95 {
                    object_list.push(Box::new(Object::Sphere {
                        center,
                        radius: 0.2,
                        material: Material::Metal {
                            albedo: Texture::ConstantTexture {
                                color: Vector3::new(
                                    0.5 * (1.0 + rng.gen::<f32>()),
                                    0.5 * (1.0 + rng.gen::<f32>()),
                                    0.5 * (1.0 + rng.gen::<f32>()),
                                ),
                            },
                            fuzz: 0.5 * rng.gen::<f32>(),
                        },
                    }));
                } else {
                    object_list.push(Box::new(Object::Sphere {
                        center,
                        radius: 0.2,
                        material: Material::Dielectric { ref_idx: 1.5 },
                    }));
                }
            }
        }
    }
    object_list
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

fn color(r: &Ray, world: &Object, depth: usize) -> Vector3 {
    // some of the rays hit at 0.00000001 instead of 0.0
    // so ignore those to remove noise
    match world.hit(r, 0.001, std::f32::MAX) {
        Some((rec, material)) => {
            if depth < 500 {
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

    // TODO: why is this taking longer :(
    let mut scene = random_scene();
    // let balls = random_scene2();
    // scene.push(Box::new(create_binary_tree(balls, 0.0, 1.0)));
    let world = &Object::ObjectList { list: scene };

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
