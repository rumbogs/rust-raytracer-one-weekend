use std::f32::consts;
use rand::Rng;

use super::hittable::Hittable;
use super::onb::ONB;
use super::random_cosine_direction;
use super::vector3::{dot, unit_vector, Vector3};

pub trait PDF {
    fn value(&self, direction: Vector3) -> f32;
    fn generate(&self) -> Vector3;
}

pub struct CosinePDF {
    uvw: ONB,
}

impl CosinePDF {
    pub fn new(w: Vector3) -> Self {
        CosinePDF { uvw: ONB::new(w) }
    }
}

impl PDF for CosinePDF {
    fn value(&self, direction: Vector3) -> f32 {
        let cosine: f32 = dot(unit_vector(direction), self.uvw.w());
        if cosine > 0.0 {
            cosine / consts::PI
        } else {
            0.0
        }
    }
    fn generate(&self) -> Vector3 {
        self.uvw.local_vec(&random_cosine_direction())
    }
}

pub struct HittablePDF {
    pub o: Vector3,
    pub hittable: Box<Hittable>,
}

impl PDF for HittablePDF {
    fn value(&self, direction: Vector3) -> f32 {
        self.hittable.pdf_value(self.o, direction)
    }
    fn generate(&self) -> Vector3 {
        self.hittable.random(self.o)
    }
}

pub struct MixturePDF {
    pdf: [Box<PDF>; 2],
}

impl MixturePDF {
    pub fn new(p0: Box<PDF>, p1: Box<PDF>) -> Self {
        MixturePDF {
            pdf: [p0, p1]
        }
    }
}

impl PDF for MixturePDF {
    fn value(&self, direction: Vector3) -> f32 {
        0.5 * self.pdf[0].value(direction) + 0.5 * self.pdf[1].value(direction)
    }
    fn generate(&self) -> Vector3 {
        let mut rng = rand::thread_rng();
        if rng.gen::<f32>() < 0.5 {
            self.pdf[0].generate()
        } else {
            self.pdf[1].generate()
        }
    }
}
