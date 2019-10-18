extern crate crossbeam;
extern crate image;
extern crate num_cpus;
extern crate rand;

use image::GenericImageView;
use rand::Rng;
use std::time::Instant;

mod aabb;
mod camera;
mod hittable;
mod material;
mod modifiers;
mod objects;
mod onb;
mod pdf;
mod perlin;
mod ray;
mod texture;
mod utils;
mod vector3;

use camera::Camera;
use hittable::Hittable;
use material::Material;
use modifiers::flip_normals::FlipNormals;
use modifiers::rotate::RotateY;
use modifiers::translate::Translate;
use objects::bvh_tree::BvhTree;
use objects::constant_medium::ConstantMedium;
use objects::cube::Cube;
use objects::moving_sphere::MovingSphere;
use objects::object_list::ObjectList;
use objects::plane::{XYRect, XZRect, YZRect};
use objects::sphere::Sphere;
use pdf::{HittablePDF, MixturePDF, PDF};
use perlin::Perlin;
use ray::Ray;
use texture::Texture;
use utils::{clamp, de_nan, random_cosine_direction, random_on_unit_sphere, random_to_sphere};
use vector3::Vector3;

fn random_scene() -> Vec<Box<Hittable>> {
    let n: usize = 500;
    let mut object_list: Vec<Box<Hittable>> = Vec::with_capacity(n + 1);
    let light_material = Material::DiffuseLight {
        emit: Texture::ConstantTexture {
            color: Vector3::new(4.0, 4.0, 4.0),
        },
    };

    object_list.push(Box::new(Sphere {
        center: Vector3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Material::Lambertian {
            albedo: Texture::ConstantTexture {
                color: Vector3::new(0.7, 0.7, 0.7),
            },
        },
    }));
    // object_list.push(Box::new(Sphere {
    //     center: Vector3::new(0.0, 1.0, 0.0),
    //     radius: 1.0,
    //     material: Material::Lambertian {
    //         albedo: Texture::NoiseTexture { noise: Perlin::new(), scale: 5.0 },
    //     },
    // }));
    object_list.push(Box::new(Sphere {
        center: Vector3::new(3.0, 1.0, 3.0),
        radius: 1.0,
        material: Material::DiffuseLight {
            emit: Texture::ConstantTexture {
                color: Vector3::new(4.0, 4.0, 4.0),
            },
        },
    }));
    let img = image::open("pug.jpg").unwrap();
    let (nx, ny) = img.dimensions();

    object_list.push(Box::new(Sphere {
        center: Vector3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Lambertian {
            albedo: Texture::ImageTexture { img },
        },
    }));

    let light = Material::DiffuseLight {
        emit: Texture::ConstantTexture {
            color: Vector3::new(4.0, 4.0, 4.0),
        },
    };

    object_list.push(Box::new(XYRect {
        x0: -2.0,
        x1: 2.0,
        y0: 1.0,
        y1: 3.0,
        k: -3.0,
        material: light,
    }));

    object_list
}

fn cornell_box(aspect: f32) -> (Vec<Box<Hittable>>, Camera) {
    let n: usize = 500;
    let mut object_list: Vec<Box<Hittable>> = Vec::with_capacity(n + 1);
    let red: Material = Material::Lambertian {
        albedo: Texture::ConstantTexture {
            color: Vector3::new(0.65, 0.05, 0.05),
        },
    };
    let white: Material = Material::Lambertian {
        albedo: Texture::ConstantTexture {
            color: Vector3::new(0.73, 0.73, 0.73),
        },
    };
    let green: Material = Material::Lambertian {
        albedo: Texture::ConstantTexture {
            color: Vector3::new(0.12, 0.45, 0.15),
        },
    };
    let light: Material = Material::DiffuseLight {
        emit: Texture::ConstantTexture {
            color: Vector3::new(15.0, 15.0, 15.0),
        },
    };
    let aluminium: Material = Material::Metal {
        albedo: Texture::ConstantTexture {
            color: Vector3::new(0.8, 0.85, 0.88),
        },
        fuzz: 0.0,
    };

    object_list.push(Box::new(FlipNormals::new(Box::new(YZRect {
        y0: 0.0,
        y1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 555.0,
        material: green,
    }))));
    object_list.push(Box::new(YZRect {
        y0: 0.0,
        y1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 0.0,
        material: red,
    }));
    object_list.push(Box::new(FlipNormals::new(Box::new(XZRect {
        x0: 213.0,
        x1: 343.0,
        z0: 227.0,
        z1: 332.0,
        k: 554.0,
        material: light,
    }))));
    object_list.push(Box::new(FlipNormals::new(Box::new(XZRect {
        x0: 0.0,
        x1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 555.0,
        material: white.clone(),
    }))));
    object_list.push(Box::new(XZRect {
        x0: 0.0,
        x1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 0.0,
        material: white.clone(),
    }));
    object_list.push(Box::new(FlipNormals::new(Box::new(XYRect {
        x0: 0.0,
        x1: 555.0,
        y0: 0.0,
        y1: 555.0,
        k: 555.0,
        material: white.clone(),
    }))));
    // object_list.push(Box::new(Translate {
    //     object: Box::new(RotateY::new(
    //         Box::new(Cube::new(
    //             Vector3::new(0.0, 0.0, 0.0),
    //             Vector3::new(165.0, 165.0, 165.0),
    //             white,
    //         )),
    //         -18.0,
    //     )),
    //     offset: Vector3::new(130.0, 0.0, 65.0),
    // }));
    object_list.push(Box::new(Sphere {
        center: Vector3::new(190.0, 90.0, 190.0),
        radius: 90.0,
        material: Material::Dielectric { ref_idx: 1.5 },
    }));
    object_list.push(Box::new(Translate {
        object: Box::new(RotateY::new(
            Box::new(Cube::new(
                Vector3::new(0.0, 0.0, 0.0),
                Vector3::new(165.0, 330.0, 165.0),
                white,
            )),
            15.0,
        )),
        offset: Vector3::new(265.0, 0.0, 295.0),
    }));

    let cornell_camera: Camera = Camera::new(
        Vector3::new(278.0, 270.0, -800.0),
        Vector3::new(278.0, 270.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        40.0,
        aspect,
        0.0,
        10.0,
        0.0,
        1.0,
    );

    (object_list, cornell_camera)
}

fn cornell_smoke() -> Vec<Box<Hittable>> {
    let n: usize = 500;
    let mut object_list: Vec<Box<Hittable>> = Vec::with_capacity(n + 1);
    let red: Material = Material::Lambertian {
        albedo: Texture::ConstantTexture {
            color: Vector3::new(0.65, 0.05, 0.05),
        },
    };
    let white: Material = Material::Lambertian {
        albedo: Texture::ConstantTexture {
            color: Vector3::new(0.73, 0.73, 0.73),
        },
    };
    let green: Material = Material::Lambertian {
        albedo: Texture::ConstantTexture {
            color: Vector3::new(0.12, 0.45, 0.15),
        },
    };
    let light: Material = Material::DiffuseLight {
        emit: Texture::ConstantTexture {
            color: Vector3::new(7.0, 7.0, 7.0),
        },
    };

    object_list.push(Box::new(FlipNormals::new(Box::new(YZRect {
        y0: 0.0,
        y1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 555.0,
        material: green,
    }))));
    object_list.push(Box::new(YZRect {
        y0: 0.0,
        y1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 0.0,
        material: red,
    }));
    object_list.push(Box::new(XZRect {
        x0: 113.0,
        x1: 443.0,
        z0: 127.0,
        z1: 432.0,
        k: 554.0,
        material: light,
    }));
    object_list.push(Box::new(FlipNormals::new(Box::new(XZRect {
        x0: 0.0,
        x1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 555.0,
        material: white.clone(),
    }))));
    object_list.push(Box::new(XZRect {
        x0: 0.0,
        x1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 0.0,
        material: white.clone(),
    }));
    object_list.push(Box::new(FlipNormals::new(Box::new(XYRect {
        x0: 0.0,
        x1: 555.0,
        y0: 0.0,
        y1: 555.0,
        k: 555.0,
        material: white.clone(),
    }))));
    let box1: Translate = Translate {
        object: Box::new(RotateY::new(
            Box::new(Cube::new(
                Vector3::new(0.0, 0.0, 0.0),
                Vector3::new(165.0, 165.0, 165.0),
                white.clone(),
            )),
            -18.0,
        )),
        offset: Vector3::new(130.0, 0.0, 65.0),
    };
    let box2: Translate = Translate {
        object: Box::new(RotateY::new(
            Box::new(Cube::new(
                Vector3::new(0.0, 0.0, 0.0),
                Vector3::new(165.0, 330.0, 165.0),
                white.clone(),
            )),
            15.0,
        )),
        offset: Vector3::new(265.0, 0.0, 295.0),
    };
    object_list.push(Box::new(ConstantMedium::new(
        Box::new(box1),
        0.01,
        Texture::ConstantTexture {
            color: Vector3::new(1.0, 1.0, 1.0),
        },
    )));
    object_list.push(Box::new(ConstantMedium::new(
        Box::new(box2),
        0.01,
        Texture::ConstantTexture {
            color: Vector3::new(0.0, 0.0, 0.0),
        },
    )));
    object_list
}

fn random_scene2() -> Vec<Box<Hittable>> {
    let n: usize = 500;
    let mut object_list: Vec<Box<Hittable>> = Vec::with_capacity(n + 1);
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
                    object_list.push(Box::new(MovingSphere {
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
                    object_list.push(Box::new(Sphere {
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
                    object_list.push(Box::new(Sphere {
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

fn final_scene() -> Vec<Box<Hittable>> {
    let mut list: Vec<Box<Hittable>> = Vec::with_capacity(30);
    let mut boxlist: Vec<Box<Hittable>> = Vec::with_capacity(10000);
    let mut boxlist2: Vec<Box<Hittable>> = Vec::with_capacity(10000);
    let mut rng = rand::thread_rng();
    let nb: usize = 20;

    let white: Material = Material::Lambertian {
        albedo: Texture::ConstantTexture {
            color: Vector3::new(0.73, 0.73, 0.73),
        },
    };
    let ground: Material = Material::Lambertian {
        albedo: Texture::ConstantTexture {
            color: Vector3::new(0.48, 0.83, 0.53),
        },
    };
    for i in 0..nb {
        for j in 0..nb {
            let w: f32 = 100.0;
            let x0: f32 = -1000.0 + i as f32 * w;
            let z0: f32 = -1000.0 + j as f32 * w;
            let y0: f32 = 0.0;
            let x1: f32 = x0 + w;
            let y1: f32 = 100.0 * (rng.gen::<f32>() + 0.01);
            let z1: f32 = z0 + w;
            boxlist.push(Box::new(Cube::new(
                Vector3::new(x0, y0, z0),
                Vector3::new(x1, y1, z1),
                ground.clone(),
            )))
        }
    }
    list.push(Box::new(BvhTree::new(boxlist, 0.0, 1.0)));

    let light: Material = Material::DiffuseLight {
        emit: Texture::ConstantTexture {
            color: Vector3::new(7.0, 7.0, 7.0),
        },
    };
    list.push(Box::new(XZRect {
        x0: 123.0,
        x1: 423.0,
        z0: 147.0,
        z1: 412.0,
        k: 554.0,
        material: light,
    }));

    let center: Vector3 = Vector3::new(400.0, 400.0, 200.0);
    list.push(Box::new(MovingSphere {
        center0: center,
        center1: center + Vector3::new(30.0, 0.0, 0.0),
        time0: 0.0,
        time1: 1.0,
        radius: 50.0,
        material: Material::Lambertian {
            albedo: Texture::ConstantTexture {
                color: Vector3::new(0.7, 0.3, 0.1),
            },
        },
    }));

    list.push(Box::new(Sphere {
        center: Vector3::new(260.0, 150.0, 45.0),
        radius: 50.0,
        material: Material::Dielectric { ref_idx: 1.5 },
    }));
    list.push(Box::new(Sphere {
        center: Vector3::new(0.0, 150.0, 145.0),
        radius: 50.0,
        material: Material::Metal {
            albedo: Texture::ConstantTexture {
                color: Vector3::new(0.8, 0.8, 0.9),
            },
            fuzz: 10.0,
        },
    }));
    list.push(Box::new(Sphere {
        center: Vector3::new(360.0, 150.0, 145.0),
        radius: 70.0,
        material: Material::Dielectric { ref_idx: 1.5 },
    }));

    list.push(Box::new(ConstantMedium::new(
        Box::new(Sphere {
            center: Vector3::new(360.0, 150.0, 145.0),
            radius: 70.0,
            material: Material::Dielectric { ref_idx: 1.5 },
        }),
        0.2,
        Texture::ConstantTexture {
            color: Vector3::new(0.2, 0.4, 0.9),
        },
    )));
    list.push(Box::new(ConstantMedium::new(
        Box::new(Sphere {
            center: Vector3::new(0.0, 0.0, 0.0),
            radius: 5000.0,
            material: Material::Dielectric { ref_idx: 1.5 },
        }),
        0.0001,
        Texture::ConstantTexture {
            color: Vector3::new(1.0, 1.0, 1.0),
        },
    )));

    let img = image::open("pug.jpg").unwrap();

    list.push(Box::new(Sphere {
        center: Vector3::new(400.0, 200.0, 400.0),
        radius: 100.0,
        material: Material::Lambertian {
            albedo: Texture::ImageTexture { img },
        },
    }));

    list.push(Box::new(Sphere {
        center: Vector3::new(220.0, 280.0, 300.0),
        radius: 80.0,
        material: Material::Lambertian {
            albedo: Texture::NoiseTexture {
                noise: Perlin::new(),
                scale: 0.1,
            },
        },
    }));

    let ns: usize = 1000;
    for _i in 0..ns {
        boxlist2.push(Box::new(Sphere {
            center: Vector3::new(
                165.0 * rng.gen::<f32>(),
                165.0 * rng.gen::<f32>(),
                165.0 * rng.gen::<f32>(),
            ),
            radius: 10.0,
            material: white.clone(),
        }))
    }

    list.push(Box::new(Translate {
        object: Box::new(RotateY::new(
            Box::new(BvhTree::new(boxlist2, 0.0, 1.0)),
            15.0,
        )),
        offset: Vector3::new(-100.0, 270.0, 395.0),
    }));

    list
}

fn color(r: &Ray, world: &ObjectList, depth: usize) -> Vector3 {
    // some of the rays hit at 0.00000001 instead of 0.0
    // so ignore those to remove noise
    match world.hit(r, 0.001, std::f32::MAX) {
        Some((rec, material)) => {
            let emitted: Vector3 = material.emitted(r, &rec, rec.u, rec.v, &rec.p);
            if depth < 50 {
                match material.scatter(r, &rec) {
                    Some(scatter_record) => {
                        //TODO: find a way to pass this in as an argument
                        let light_shape: XZRect = XZRect {
                            x0: 213.0,
                            x1: 343.0,
                            z0: 227.0,
                            z1: 332.0,
                            k: 554.0,
                            material: Material::DiffuseLight {
                                emit: Texture::ConstantTexture {
                                    color: Vector3::new(15.0, 15.0, 15.0),
                                },
                            },
                        };
                        let sphere_shape: Sphere = Sphere {
                            center: Vector3::new(190.0, 90.0, 190.0),
                            radius: 90.0,
                            material: Material::Lambertian {
                                albedo: Texture::ConstantTexture {
                                    color: Vector3::new(0.7, 0.7, 0.7),
                                },
                            },
                        };
                        let object_list: ObjectList =
                            ObjectList::new(vec![Box::new(light_shape), Box::new(sphere_shape)]);

                        if scatter_record.is_specular {
                            return scatter_record.attenuation
                                * color(&scatter_record.specular_ray.unwrap(), world, depth + 1);
                        }
                        let plight: HittablePDF = HittablePDF {
                            o: rec.p,
                            hittable: Box::new(object_list),
                        };
                        let p: MixturePDF =
                            MixturePDF::new(Box::new(plight), scatter_record.pdf.unwrap());
                        let scattered = Ray::new(rec.p, p.generate(), r.time);
                        let pdf_val = p.value(scattered.direction());
                        emitted
                            + scatter_record.attenuation
                                * material.scattering_pdf(r, &rec, &scattered)
                                * color(&scattered, world, depth + 1)
                                / pdf_val
                    }
                    None => emitted,
                }
            } else {
                Vector3::new(0.0, 0.0, 0.0)
            }
        }
        None => {
            // let unit_direction = unit_vector(r.direction());
            // let t: f32 = 0.5 * (unit_direction.y() + 1.0);
            // (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
            Vector3::new(0.0, 0.0, 0.0)
        }
    }
}

fn main() {
    let cpu_num = num_cpus::get() - 1; // leave some for the rest of the processes
    let now = Instant::now();
    let width = 500;
    let height = 500;
    // larger makes blur/shadows/antialias smoother
    let smoothness: u32 = 100;
    // use one less thread for exact division
    // the last one will have less pixels to calculate
    let thread_rows = height / cpu_num + 1;
    let camera: &Camera = &Camera::new(
        Vector3::new(11.0, 4.0, 5.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        36.0,
        width as f32 / height as f32,
        0.0,
        10.0,
        0.0,
        1.0,
    );
    let final_camera: &Camera = &Camera::new(
        Vector3::new(478.0, 278.0, -600.0),
        Vector3::new(278.0, 278.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        40.0,
        width as f32 / height as f32,
        0.0,
        10.0,
        0.0,
        1.0,
    );

    let (scene, cornell_camera) = cornell_box(width as f32 / height as f32);
    let camera = &cornell_camera;
    // let balls = random_scene2();
    // scene.push(Box::new(create_binary_tree(balls, 0.0, 1.0)));
    let world = &ObjectList::new(scene);

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

                        let inverted_row = ((cpu_num - i - 1) * thread_rows + inverted_y) as f32;
                        let mut rng = rand::thread_rng();

                        // this shoots rays around the object
                        // edge using a random offset
                        // and computes a color average
                        for _ in 0..smoothness {
                            let u = (x as f32 + rng.gen::<f32>()) / width as f32;
                            let v = (inverted_row + rng.gen::<f32>()) / height as f32;
                            let r = camera.get_ray(u, v);
                            col += de_nan(color(&r, &world, 0));
                        }
                        col /= smoothness as f32;
                        // remove the gamma of 2 from the color (raise to power of 1/2)
                        col = Vector3::new(col[0].sqrt(), col[1].sqrt(), col[2].sqrt());
                        let ir = clamp(255.99 * &col[0], 0.0, 255.99);
                        let ig = clamp(255.99 * &col[1], 0.0, 255.99);
                        let ib = clamp(255.99 * &col[2], 0.0, 255.99);

                        // save buffer values
                        let buffer_pos = y * width + x;
                        row[buffer_pos][0] = ir;
                        row[buffer_pos][1] = ig;
                        row[buffer_pos][2] = ib;
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
