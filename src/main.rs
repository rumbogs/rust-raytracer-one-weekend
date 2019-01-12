mod vector3;
extern crate image;

// cannot use, open issue here https://github.com/rust-lang/rust/issues/56417
// use vector3::Vector3;

struct Ray {
    A: vector3::Vector3,
    B: vector3::Vector3,
}

impl Ray {
    fn origin(&self) -> vector3::Vector3 {
        self.A
    }
    fn direction(&self) -> vector3::Vector3 {
        self.B
    }
    fn point_at_parameter(&self, t: f32) -> vector3::Vector3 {
        self.A + t * self.B
    }
}

fn main() {
    let nx: u32 = 200;
    let ny: u32 = 100;

    let mut imgbuf = image::ImageBuffer::new(nx, ny);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // x is [0..nx]
        // y is [0..ny]
        // red goes from left to right
        // green goes from bottom to top
        let color = vec![x as f32 / nx as f32, (ny - y) as f32 / ny as f32, 0.2];
        let ir = (255.99 * &color[0]) as u8;
        let ig = (255.99 * &color[1]) as u8;
        let ib = (255.99 * &color[2]) as u8;
        *pixel = image::Rgb([ir, ig, ib]);
    }

    imgbuf.save("1.png").unwrap();
}
