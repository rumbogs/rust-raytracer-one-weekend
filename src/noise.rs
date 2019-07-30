use super::vector3::Vector3;
use rand::{Rng, SeedableRng};

fn perlin_generate() -> [f32; 256] {
    let mut p: [f32; 256] = [0.0; 256];
    let mut rng = rand::thread_rng();
    for i in 0..256 {
        p[i] = rng.gen::<f32>();
    }
    p
}

fn permute(mut p: [usize; 256]) {
    let mut rng = rand::thread_rng();
    for i in 0..256 {
        let target: usize = (rng.gen::<f32>() * (i + 1) as f32) as usize;
        let tmp: usize = p[i];
        p[i] = p[target];
        p[target] = tmp;
    }
}

fn perlin_generate_perm() -> [usize; 256] {
    let mut p: [usize; 256] = [0; 256];
    for i in 0..256 {
        p[i] = i;
    }
    permute(p);
    p
}

pub struct Perlin {
    ranfloat: [f32; 256],
    perm_x: [usize; 256],
    perm_y: [usize; 256],
    perm_z: [usize; 256],
}

impl Perlin {
    pub fn new() -> Perlin {
        Perlin {
            ranfloat: perlin_generate(),
            perm_x: perlin_generate_perm(),
            perm_y: perlin_generate_perm(),
            perm_z: perlin_generate_perm(),
        }
    }

    pub fn noise(&self, p: &Vector3) -> f32 {
        let u: f32 = p.x() - p.x().floor();
        let v: f32 = p.y() - p.y().floor();
        let w: f32 = p.z() - p.z().floor();
        let i: usize = (4.0 * p.x()) as usize & 255;
        let j: usize = (4.0 * p.y()) as usize & 255;
        let k: usize = (4.0 * p.z()) as usize & 255;
        self.ranfloat[self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]]
    }
}
