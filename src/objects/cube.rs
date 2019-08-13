use super::super::aabb::Aabb;
use super::super::hittable::{HitRecord, Hittable};
use super::super::material::Material;
use super::super::modifiers::flip_normals::FlipNormals;
use super::super::ray::Ray;
use super::super::vector3::Vector3;
use super::object_list::ObjectList;
use super::plane::{XYRect, XZRect, YZRect};

pub struct Cube {
    pub pmin: Vector3,
    pub pmax: Vector3,
    pub material: Material,
    pub faces: ObjectList,
}

impl Cube {
    pub fn new(pmin: Vector3, pmax: Vector3, material: Material) -> Self {
        let mut list: Vec<Box<Hittable>> = Vec::with_capacity(6);
        list.push(Box::new(XYRect {
            x0: pmin.x(),
            x1: pmax.x(),
            y0: pmin.y(),
            y1: pmax.y(),
            k: pmax.z(),
            material: material.clone(),
        }));
        list.push(Box::new(FlipNormals::new(Box::new(XYRect {
            x0: pmin.x(),
            x1: pmax.x(),
            y0: pmin.y(),
            y1: pmax.y(),
            k: pmin.z(),
            material: material.clone(),
        }))));
        list.push(Box::new(XZRect {
            x0: pmin.x(),
            x1: pmax.x(),
            z0: pmin.z(),
            z1: pmax.z(),
            k: pmax.y(),
            material: material.clone(),
        }));
        list.push(Box::new(FlipNormals::new(Box::new(XZRect {
            x0: pmin.x(),
            x1: pmax.x(),
            z0: pmin.z(),
            z1: pmax.z(),
            k: pmin.y(),
            material: material.clone(),
        }))));
        list.push(Box::new(YZRect {
            y0: pmin.y(),
            y1: pmax.y(),
            z0: pmin.z(),
            z1: pmax.z(),
            k: pmax.x(),
            material: material.clone(),
        }));
        list.push(Box::new(FlipNormals::new(Box::new(YZRect {
            y0: pmin.y(),
            y1: pmax.y(),
            z0: pmin.z(),
            z1: pmax.z(),
            k: pmin.x(),
            material: material.clone(),
        }))));
        Cube {
            pmin,
            pmax,
            material: material,
            faces: ObjectList::new(list),
        }
    }
}

impl Hittable for Cube {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<(HitRecord, &Material)> {
        self.faces.hit(r, t_min, t_max)
    }
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb> {
        Some(Aabb::new(self.pmin, self.pmax))
    }
}
