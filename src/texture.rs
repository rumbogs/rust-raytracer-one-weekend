use super::vector3::Vector3;

pub trait Texture {
    fn box_clone(&self) -> Box<dyn Texture + Sync>;
    fn value(&self, u: f32, v: f32, p: &Vector3) -> Vector3;
    fn getColor(&self) -> Vector3;
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

    fn getColor(&self) -> Vector3 {
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

    fn getColor(&self) -> Vector3 {
        self.odd.getColor()
    }
}
