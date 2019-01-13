extern crate image;

mod hitable;
mod ray;
mod vector3;

// cannot use, open issue here https://github.com/rust-lang/rust/issues/56417
// use vector3::Vector3;

fn color(r: &ray::Ray, world: hitable::Hitable) -> vector3::Vector3 {
    let rec: hitable::hit_record;
    if world.hit(r, 0.0, MAXFLOAT, rec) {
        return 0.5
            * vector3::Vector3::new(
                rec.normal.x() + 1.0,
                rec.normal.y() + 1.0,
                rec.normal.z() + 1.0,
            );
    } else {
        let unit_direction = vector3::unit_vector(r.direction());
        let t: f32 = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * vector3::Vector3::new(1.0, 1.0, 1.0) + t * vector3::Vector3::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    let nx: u32 = 200;
    let ny: u32 = 100;

    let lower_left_corner: vector3::Vector3 = vector3::Vector3 {
        e: [-2.0, -1.0, -1.0],
    };
    let horizontal: vector3::Vector3 = vector3::Vector3::new(4.0, 0.0, 0.0);
    let vertical: vector3::Vector3 = vector3::Vector3::new(0.0, 2.0, 0.0);
    let origin: vector3::Vector3 = vector3::Vector3::new(0.0, 0.0, 0.0);

    let mut imgbuf = image::ImageBuffer::new(nx, ny);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // x is [0..nx]
        // y is [0..ny]
        // red goes from left to right
        // green goes from bottom to top
        let inverted_y = ny - y;
        let u = x as f32 / nx as f32;
        let v = inverted_y as f32 / ny as f32;
        let r = ray::Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
        let col = color(&r);
        let ir = (255.99 * &col[0]) as u8;
        let ig = (255.99 * &col[1]) as u8;
        let ib = (255.99 * &col[2]) as u8;
        *pixel = image::Rgb([ir, ig, ib]);
    }

    imgbuf.save("1.png").unwrap();
}
