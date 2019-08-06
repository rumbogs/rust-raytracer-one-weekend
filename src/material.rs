use super::object::HitRecord;
use super::random_in_unit_sphere;
use super::ray::Ray;
use super::texture::Texture;
use super::vector3::{dot, unit_vector, Vector3};

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

pub enum Material {
    Lambertian { albedo: Texture },
    Metal { albedo: Texture, fuzz: f32 },
    Dielectric { ref_idx: f32 },
}

impl Material {
    pub fn scatter(&self, r_in: &Ray, rec: HitRecord) -> Option<(Vector3, Ray)> {
        match self {
            Material::Lambertian { albedo } => {
                let target: Vector3 = rec.p + rec.normal + random_in_unit_sphere();
                Some((
                    albedo.value(0.0, 0.0, &rec.p),
                    Ray::new(rec.p, target - rec.p, r_in.time),
                ))
            }
            Material::Metal { albedo, fuzz } => {
                let mut fuzz = *fuzz;
                if fuzz > 1.0 {
                    fuzz = 1.0;
                }
                let reflected = reflect(unit_vector(r_in.direction()), rec.normal);
                let scattered =
                    Ray::new(rec.p, reflected + fuzz * random_in_unit_sphere(), 0.0);
                if dot(scattered.direction(), rec.normal) > 0.0 {
                    Some((albedo.value(0.0, 0.0, &rec.p), scattered))
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

                Some((attenuation, scattered))
            }
        }
    }
}
