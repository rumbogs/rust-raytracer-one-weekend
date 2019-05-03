use rand::Rng;

use super::vector3::{unit_vector, Vector3};

fn perlin_interp(c: [[[Vector3; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);
    let mut accum: f32 = 0.0;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let weight_v: Vector3 = Vector3::new(u - i, v - j, w - k);
                accum += (i as f32 * uu + (1 - i) as f32 * (1.0 - uu))
                    * (j as f32 * vv + (1 - j) as f32 * (1.0 - vv))
                    * (k as f32 * ww + (1 - k) as f32 * (1.0 - ww))
                    * c[i][j][k];
            }
        }
    }
    accum
}

fn perlin_generate() -> [Vector3; 256] {
    let mut p: [Vector3; 256] = [Vector3::new(0.0, 0.0, 0.0); 256];
    let mut rng = rand::thread_rng();

    for i in 0..256 {
        p[i] = unit_vector(Vector3::new(
            -1.0 * 2.0 * rng.gen::<f32>(),
            -1.0 + 2.0 * rng.gen::<f32>(),
            -1.0 + 2.0 * rng.gen::<f32>(),
        ));
    }
    p
}

fn permute(mut p: [usize; 256], n: usize) {
    let mut rng = rand::thread_rng();

    for i in (0..n - 1).rev() {
        let random = rng.gen::<f32>() as usize;
        let target = random * (i + 1);
        let tmp = p[i];
        p[i] = p[target];
        p[target] = tmp;
    }
}

fn perlin_generate_perm() -> [usize; 256] {
    let mut p: [usize; 256] = [0; 256];
    for i in 0..256 {
        p[i] = i;
    }
    permute(p, 256);
    p
}

#[derive(Clone)]
pub struct Perlin {
    ranvec: [Vector3; 256],
    perm_x: [usize; 256],
    perm_y: [usize; 256],
    perm_z: [usize; 256],
}

impl Perlin {
    pub fn new() -> Self {
        Perlin {
            ranvec: perlin_generate(),
            perm_x: perlin_generate_perm(),
            perm_y: perlin_generate_perm(),
            perm_z: perlin_generate_perm(),
        }
    }

    pub fn noise(&self, p: Vector3) -> f32 {
        let mut u: f32 = p.x() - p.x().floor();
        let mut v: f32 = p.y() - p.y().floor();
        let mut w: f32 = p.z() - p.z().floor();
        let i: isize = p.x().floor() as isize;
        let j: isize = p.y().floor() as isize;
        let k: isize = p.z().floor() as isize;
        let mut c: [[[Vector3; 2]; 2]; 2] = [
            [
                [Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0)],
                [Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0)],
            ],
            [
                [Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0)],
                [Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0)],
            ],
        ];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranvec[self.perm_x[((i + di as isize) & 255) as usize]
                        ^ self.perm_y[((j + dj as isize) & 255) as usize]
                        ^ self.perm_z[((k + dk as isize) & 255) as usize]];
                }
            }
        }
        perlin_interp(c, u, v, w)
    }
}
