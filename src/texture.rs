use super::perlin::Perlin;
use super::vector3::Vector3;

pub enum Texture {
    ConstantTexture{ color: Vector3 },
    CheckerTexture{ even: Box<Texture>, odd: Box<Texture> },
    NoiseTexture{ noise: Perlin, scale: f32 },
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
            },
            Texture::NoiseTexture { noise, scale } => {
                // Vector3::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + self.noise.turb(&(self.scale * p), None))
                // Vector3::new(1.0, 1.0, 1.0) * self.noise.turb(&(self.scale * p), None)
                Vector3::new(1.0, 1.0, 1.0)
                    * 0.5
                    * (1.0 + (scale * p.z() + 10.0 * noise.turb(&(*scale * p), None)).sin())
            }
        }
        
    }
}
