extern crate crossbeam;
extern crate image;
// extern crate indicatif;
extern crate num_cpus;
extern crate rand;

use image::png::PNGEncoder;
use image::ColorType;
// use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;
use std::cmp;
use std::fs::File;
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

fn save_image(buffer: &[u8], width: u32, height: u32) -> Result<(), std::io::Error> {
    image::save_buffer("1.png", buffer, width, height, image::RGB(8))
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
    let cpu_num = num_cpus::get();
    let now = Instant::now();
    let width: u32 = 200;
    let height: u32 = 100;
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

    // let mut imgbuf = image::ImageBuffer::new(width, height);
    // let progress_bar = ProgressBar::new((width * height) as u64);
    // progress_bar.set_style(
    //     ProgressStyle::default_bar()
    //         .template("[{elapsed_precise}] {bar:40.cyan/blue} {percent:>7}% {msg}")
    //         .progress_chars("##-"),
    // );

    let thread_rows: u32 = height / cpu_num as u32 + 1;
    let mut buffer = vec![0; (width * height * 3) as usize];
    let rows: Vec<&mut [u8]> = buffer
        .chunks_mut((thread_rows * width * 3) as usize)
        .collect();
    //TODO: correct division between rows of pixels
    crossbeam::scope(|spawner| {
        for (i, row) in rows.into_iter().enumerate() {
            spawner.spawn(move |_| {
                for y in 0..height {
                    for x in 0..width {
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
                            col += color(&r, &world, 0);
                        }
                        col /= antialiasing_sensitivity as f32;
                        // remove the gamma of 2 from the color (raise to power of 1/2)
                        col = Vector3::new(col[0].sqrt(), col[1].sqrt(), col[2].sqrt());
                        let ir = (255.99 * &col[0]) as u8;
                        let ig = (255.99 * &col[1]) as u8;
                        let ib = (255.99 * &col[2]) as u8;
                        // save buffer values
                        row[(3 * (y * width + x)) as usize] = ir;
                        row[(3 * (y * width + x) + 1) as usize] = ig;
                        row[(3 * (y * width + x) + 2) as usize] = ib;
                        // progress_bar.inc(1);
                        print!("\r{}s", now.elapsed().as_secs());
                    }
                }
            });
        }
    })
    .unwrap();

    match save_image(&buffer, width, height) {
        Ok(()) => println!("OK"),
        Err(err) => println!("{}", err),
    };

    // progress_bar.finish();
    println!(
        "Finished in {}s {}ms",
        now.elapsed().as_secs(),
        (now.elapsed().subsec_nanos() / 1_000_000) as u64
    );
    // imgbuf.save("1.png").unwrap();
}
