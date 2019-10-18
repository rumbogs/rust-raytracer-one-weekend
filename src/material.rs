use super::hittable::HitRecord;
use super::onb::ONB;
use super::pdf::{CosinePDF, PDF};
use super::ray::Ray;
use super::texture::Texture;
use super::vector3::{dot, unit_vector, Vector3};
use super::{random_cosine_direction, random_on_unit_sphere};
use std::f32::consts;

use rand::Rng;

fn reflect(v: Vector3, n: Vector3) -> Vector3 {
    v - 2.0 * dot(v, n) * n
}

fn refract(v: Vector3, n: Vector3, ni_over_nt: f32) -> Option<Vector3> {
    let uv: Vector3 = unit_vector(v);
    let dt: f32 = dot(uv, n);
    let discriminant: f32 = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some(ni_over_nt * (uv - n * dt) - n * discriminant.sqrt())
    } else {
        None
    }
}

// approximation for fresnel reflections
fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0: f32 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}

pub struct ScatterRecord {
    pub specular_ray: Option<Ray>,
    pub is_specular: bool,
    pub attenuation: Vector3,
    pub pdf: Option<Box<PDF>>,
}

#[derive(Clone)]
pub enum Material {
    Lambertian { albedo: Texture },
    Metal { albedo: Texture, fuzz: f32 },
    Dielectric { ref_idx: f32 },
    DiffuseLight { emit: Texture },
    Isotropic { texture: Texture },
}

impl Material {
    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        match self {
            Material::Lambertian { albedo } => {
                let uvw: ONB = ONB::new(rec.normal);
                let direction: Vector3 = uvw.local_vec(&random_cosine_direction());
                let scattered: Ray = Ray::new(rec.p, unit_vector(direction), r_in.time);
                let scattered_direction = scattered.direction();
                Some(ScatterRecord {
                    specular_ray: None,
                    is_specular: false,
                    attenuation: albedo.value(rec.u, rec.v, &rec.p),
                    pdf: Some(Box::new(CosinePDF::new(rec.normal))),
                })
            }
            Material::Metal { albedo, fuzz } => {
                let mut fuzz = *fuzz;
                if fuzz > 1.0 {
                    fuzz = 1.0;
                }
                let reflected = reflect(unit_vector(r_in.direction()), rec.normal);
                let scattered = Ray::new(rec.p, reflected + fuzz * random_on_unit_sphere(), 0.0);
                if dot(scattered.direction(), rec.normal) > 0.0 {
                    Some(ScatterRecord {
                        attenuation: albedo.value(rec.u, rec.v, &rec.p),
                        specular_ray: Some(scattered),
                        is_specular: true,
                        pdf: None,
                    })
                } else {
                    None
                }
            }
            Material::Dielectric { ref_idx } => {
                let outward_normal: Vector3;
                let reflected: Vector3 = reflect(r_in.direction(), rec.normal);
                let ni_over_nt: f32;
                let attenuation = Vector3::new(1.0, 1.0, 1.0);
                let scattered: Ray;
                let mut saved_refracted: Vector3 = Vector3::new(0.0, 0.0, 0.0);
                let reflect_prob: f32;
                let ray_angle = dot(r_in.direction(), rec.normal);
                let random = rand::thread_rng().gen::<f32>();
                let mut cosine: f32 = ray_angle / r_in.direction().length();

                if ray_angle > 0.0 {
                    outward_normal = -rec.normal;
                    ni_over_nt = *ref_idx;
                    cosine = ref_idx * cosine;
                } else {
                    outward_normal = rec.normal;
                    ni_over_nt = 1.0 / ref_idx;
                    cosine = -cosine;
                }

                match refract(r_in.direction(), outward_normal, ni_over_nt) {
                    Some(refracted) => {
                        reflect_prob = schlick(cosine, *ref_idx);
                        saved_refracted = refracted;
                    }
                    None => {
                        reflect_prob = 1.0;
                    }
                }

                if random < reflect_prob {
                    scattered = Ray::new(rec.p, reflected, 0.0);
                } else {
                    scattered = Ray::new(rec.p, saved_refracted, 0.0);
                }

                Some(ScatterRecord {
                    attenuation,
                    specular_ray: Some(scattered),
                    is_specular: true,
                    pdf: None,
                })
            }
            Material::Isotropic { texture } => {
                let target: Vector3 = rec.p + rec.normal + random_on_unit_sphere();
                Some(ScatterRecord {
                    attenuation: texture.value(rec.u, rec.v, &rec.p),
                    specular_ray: Some(Ray::new(rec.p, random_on_unit_sphere(), r_in.time)),
                    is_specular: true,
                    pdf: None,
                })
            }
            _ => None,
        }
    }

    pub fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f32 {
        match self {
            Material::Lambertian { albedo } => {
                let mut cosine: f32 = dot(rec.normal, unit_vector(scattered.direction()));
                if cosine < 0.0 {
                    cosine = 0.0;
                }
                cosine / consts::PI
            }
            // Material::Metal { albedo, fuzz } => {
            // }
            // Material::Dielectric { ref_idx } => {
            // }
            // Material::Isotropic { texture } => {
            // }
            _ => 0.0,
        }
    }

    pub fn emitted(&self, r_in: &Ray, rec: &HitRecord, u: f32, v: f32, p: &Vector3) -> Vector3 {
        match self {
            Material::DiffuseLight { emit } => {
                if dot(rec.normal, r_in.direction()) < 0.0 {
                    emit.value(u, v, p)
                } else {
                    Vector3::new(0.0, 0.0, 0.0)
                }
            }
            _ => Vector3::new(0.0, 0.0, 0.0),
        }
    }
}
