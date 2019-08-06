use super::perlin::Perlin;
use super::vector3::Vector3;
use image::{DynamicImage, GenericImageView};

pub enum Texture {
    ConstantTexture {
        color: Vector3,
    },
    CheckerTexture {
        even: Box<Texture>,
        odd: Box<Texture>,
    },
    NoiseTexture {
        noise: Perlin,
        scale: f32,
    },
    ImageTexture {
        img: DynamicImage,
    },
}

impl Texture {
    pub fn value(&self, u: f32, v: f32, p: &Vector3) -> Vector3 {
        match self {
            Texture::ConstantTexture { color } => *color,
            Texture::CheckerTexture { even, odd } => {
                let sines: f32 = (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();
                if sines < 0.0 {
                    odd.value(u, v, p)
                } else {
                    even.value(u, v, p)
                }
            }
            Texture::NoiseTexture { noise, scale } => {
                // Vector3::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + self.noise.turb(&(self.scale * p), None))
                // Vector3::new(1.0, 1.0, 1.0) * self.noise.turb(&(self.scale * p), None)
                Vector3::new(1.0, 1.0, 1.0)
                    * 0.5
                    * (1.0 + (scale * p.z() + 10.0 * noise.turb(&(*scale * p), None)).sin())
            }
            Texture::ImageTexture { img } => {
                let (nx, ny) = img.dimensions();
                let nx_isize = nx as isize;
                let ny_isize = ny as isize;
                let mut i: isize = (u * nx as f32) as isize;
                let mut j: isize = ((1.0 - v) * ny as f32 - 0.001) as isize;
                if i < 0 {
                    i = 0;
                }
                if j < 0 {
                    j = 0;
                }
                if i > nx_isize - 1 {
                    i = nx_isize - 1;
                }
                if j > ny_isize - 1 {
                    j = ny_isize - 1;
                }
                let pixel = img.get_pixel(i as u32, j as u32);
                return Vector3::new(
                    pixel[0] as f32 / 255.0,
                    pixel[1] as f32 / 255.0,
                    pixel[2] as f32 / 255.0,
                );
            }
        }
    }
}
