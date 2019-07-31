use super::perlin::Perlin;
use super::vector3::Vector3;

pub trait Texture {
    fn box_clone(&self) -> Box<dyn Texture + Sync>;
    fn value(&self, u: f32, v: f32, p: &Vector3) -> Vector3;
}

impl Clone for Box<dyn Texture + Sync> {
    fn clone(&self) -> Box<dyn Texture + Sync> {
        self.box_clone()
    }
}

pub struct ConstantTexture {
    color: Vector3,
}

impl ConstantTexture {
    pub fn new(color: Vector3) -> ConstantTexture {
        ConstantTexture { color }
    }
}

impl Texture for ConstantTexture {
    fn box_clone(&self) -> Box<dyn Texture + Sync> {
        Box::new(ConstantTexture::new(self.color.clone()))
    }

    fn value(&self, u: f32, v: f32, p: &Vector3) -> Vector3 {
        self.color
    }
}

pub struct CheckerTexture {
    even: Box<dyn Texture + Sync>,
    odd: Box<dyn Texture + Sync>,
}

impl CheckerTexture {
    pub fn new(even: Box<dyn Texture + Sync>, odd: Box<dyn Texture + Sync>) -> CheckerTexture {
        CheckerTexture { even, odd }
    }
}

impl Texture for CheckerTexture {
    fn box_clone(&self) -> Box<dyn Texture + Sync> {
        Box::new(CheckerTexture::new(self.even.clone(), self.odd.clone()))
    }

    fn value(&self, u: f32, v: f32, p: &Vector3) -> Vector3 {
        let sines: f32 = (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

pub struct NoiseTexture {
    noise: Perlin,
    scale: f32,
}

impl NoiseTexture {
    pub fn new(scale: f32) -> NoiseTexture {
        NoiseTexture {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn box_clone(&self) -> Box<dyn Texture + Sync> {
        Box::new(NoiseTexture::new(self.scale))
    }

    fn value(&self, u: f32, v: f32, p: &Vector3) -> Vector3 {
        // Vector3::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + self.noise.turb(&(self.scale * p), None))
        // Vector3::new(1.0, 1.0, 1.0) * self.noise.turb(&(self.scale * p), None)
        Vector3::new(1.0, 1.0, 1.0)
            * 0.5
            * (1.0 + (self.scale * p.z() + 10.0 * self.noise.turb(&(self.scale * p), None)).sin())
    }
}
