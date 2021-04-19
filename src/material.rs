use crate::hit::HitRecord;
use crate::ray::Ray;
use crate::util;
use crate::vec3::Vec3;

use rand::Rng;

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vec3)>;
}

pub type MaterialHandle = Box<dyn Material + Send + Sync>;

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vec3)> {
        let target = hit.point + hit.normal + util::random_in_unit_sphere();
        Some((
            Ray {
                origin: hit.point,
                direction: target - hit.point,
            },
            self.albedo,
        ))
    }
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vec3)> {
        let fuzz = if self.fuzz < 1.0 { self.fuzz } else { 1.0 };
        let reflected = reflect(ray.direction.unit(), hit.normal);
        if Vec3::dot(reflected, hit.normal) > 0.0 {
            Some((
                Ray {
                    origin: hit.point,
                    direction: reflected + util::random_in_unit_sphere() * fuzz,
                },
                self.albedo,
            ))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    pub refraction_idx: f32,
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vec3)> {
        assert_ne!(self.refraction_idx, 0.0);
        let mut rng = rand::thread_rng();

        let reflected = reflect(ray.direction, hit.normal);

        let (outward_normal, ni_over_nt, cosine) = if Vec3::dot(ray.direction, hit.normal) > 0.0 {
            (
                -hit.normal,
                self.refraction_idx,
                self.refraction_idx * Vec3::dot(ray.direction, hit.normal) / ray.direction.length(),
            )
        } else {
            (
                hit.normal,
                1.0 / self.refraction_idx,
                -Vec3::dot(ray.direction, hit.normal) / ray.direction.length(),
            )
        };

        let direction = refract(ray.direction, outward_normal, ni_over_nt)
            .filter(|_| rng.gen_range(0.0..1.0) > schlick(cosine, self.refraction_idx))
            .unwrap_or(reflected);

        Some((
            Ray {
                origin: hit.point,
                direction,
            },
            Vec3::one(),
        ))
    }
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * Vec3::dot(v, n) * n
}

pub fn refract(v: Vec3, n: Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = v.unit();
    let dt = Vec3::dot(uv, n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);

    if discriminant > 0.0 {
        Some(ni_over_nt * (uv - n * dt) - n * discriminant.sqrt())
    } else {
        None
    }
}

pub fn schlick(cosine: f32, refraction_idx: f32) -> f32 {
    let mut r0 = (1.0 - refraction_idx) / (1.0 + refraction_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
