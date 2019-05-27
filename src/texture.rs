use noise::{NoiseFn, Perlin, Turbulence};
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

fn turb(p: &Vector3, noise: &Perlin, scale: f64, depth: Option<isize>) -> f64 {
    let mut accum: f64 = 0.0;
    let mut temp_p: Vector3 = *p;
    let mut weight: f64 = 1.0;
    let depth = match depth {
        Some(val) => val,
        None => 7
    };
    
    for i in 0..depth {
        accum += weight * noise.get([scale * p.x() as f64, scale * p.y() as f64, scale * p.z() as f64]);
        weight *= 0.5;
        temp_p *= 2.0;
    }
    let absolute_val = accum.abs();
    if absolute_val > 1.0 {
        1.0
    } else {
        absolute_val
    }
}

#[derive(Clone)]
pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new() -> Self {
        NoiseTexture {
            noise: Perlin::new(),
            scale: 4.0,
        }
    }
}

impl Texture for NoiseTexture {
    fn box_clone(&self) -> Box<dyn Texture + Sync> {
        Box::new(self.clone())
    }

    fn value(&self, u: f32, v: f32, p: &Vector3) -> Vector3 {
        let point_value = [self.scale * p.x() as f64, self.scale * p.y() as f64, self.scale * p.z() as f64];
        let noise_value = self.noise.get(point_value);
        // let turbulence = Turbulence::new(&self.noise);
        // let turbulence_value = turbulence.get(point_value);
        // let turbulence_value = turb(p, &self.noise, self.scale, None);
        Vector3::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + (10.0 * turbulence_value as f32) + self.scale as f32 * p.z())
    }
}
