extern crate image;

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
