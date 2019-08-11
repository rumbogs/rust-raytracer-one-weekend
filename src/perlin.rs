use super::vector3::{dot, unit_vector, Vector3};
use rand::Rng;

fn perlin_interp(c: [[[Vector3; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);
    let mut accum: f32 = 0.0;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let i_f32: f32 = i as f32;
                let j_f32: f32 = j as f32;
                let k_f32: f32 = k as f32;
                let weight_v: Vector3 = Vector3::new(u - i_f32, v - j_f32, w - k_f32);
                accum += (i_f32 * uu + (1.0 - i_f32) * (1.0 - uu))
                    * (j_f32 * vv + (1.0 - j_f32) * (1.0 - vv))
                    * (k_f32 * ww + (1.0 - k_f32) * (1.0 - ww))
                    * dot(c[i][j][k], weight_v);
            }
        }
    }
    accum
}

fn perlin_generate() -> [Vector3; 256] {
    let mut p = [Vector3::new(0.0, 0.0, 0.0); 256];
    let mut rng = rand::thread_rng();
    for i in 0..256 {
        p[i] = unit_vector(Vector3::new(
            -1.0 + 2.0 * rng.gen::<f32>(),
            -1.0 + 2.0 * rng.gen::<f32>(),
            -1.0 + 2.0 * rng.gen::<f32>(),
        ));
    }
    p
}

fn permute(p: &mut [usize; 256]) {
    let mut rng = rand::thread_rng();
    for i in (0..256).rev() {
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
    permute(&mut p);
    p
}

#[derive(Clone)]
pub struct Perlin {
    ranfloat: [Vector3; 256],
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

    pub fn turb(&self, p: &Vector3, depth: Option<usize>) -> f32 {
        let mut accum: f32 = 0.0;
        let mut temp_p: Vector3 = Vector3::new(p.x(), p.y(), p.z());
        let mut weight: f32 = 1.0;
        let depth = match depth {
            Some(v) => v,
            None => 7,
        };
        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }
        accum.abs()
    }

    pub fn noise(&self, p: &Vector3) -> f32 {
        let u: f32 = p.x() - p.x().floor();
        let v: f32 = p.y() - p.y().floor();
        let w: f32 = p.z() - p.z().floor();
        let i: isize = p.x().floor() as isize;
        let j: isize = p.y().floor() as isize;
        let k: isize = p.z().floor() as isize;
        let mut c: [[[Vector3; 2]; 2]; 2] = [[[Vector3::new(1.0, 1.0, 1.0); 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranfloat[self.perm_x[((i + di as isize) & 255) as usize]
                        ^ self.perm_y[((j + dj as isize) & 255) as usize]
                        ^ self.perm_z[((k + dk as isize) & 255) as usize]]
                }
            }
        }
        perlin_interp(c, u, v, w)
    }
}
