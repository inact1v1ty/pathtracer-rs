use crate::aabb::Aabb;
use crate::iter_util::IteratorExt;
use crate::material::MaterialHandle;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::sync::Arc;

#[derive(Clone)]
pub struct HitRecord {
    pub t: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Arc<MaterialHandle>,
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb>;
}

pub type HitableHandle = Box<dyn Hitable + Send + Sync>;

impl<T> Hitable for Vec<T>
where
    T: Hitable,
{
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let (hit, _) = self.iter().fold(
            (None, t_max),
            |(res, closest_so_far): (Option<HitRecord>, f32), h| match h.hit(
                ray,
                t_min,
                closest_so_far,
            ) {
                Some(hit_record) => {
                    let t = hit_record.t;
                    (Some(hit_record), t)
                }
                None => (res, closest_so_far),
            },
        );
        hit
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb> {
        self.iter()
            .map(|h| h.bounding_box(t0, t1))
            .try_reduce(Aabb::merge)
    }
}

#[derive(Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Arc<MaterialHandle>,
}

impl Hitable for Sphere {
    #[allow(clippy::suspicious_operation_groupings)]
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = Vec3::dot(ray.direction, ray.direction);
        let b = Vec3::dot(oc, ray.direction);
        let c = Vec3::dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let temp = (-b - discriminant.sqrt()) / a;
            if temp > t_min && temp < t_max {
                return Some(HitRecord {
                    t: temp,
                    point: ray.point_at(temp),
                    normal: (ray.point_at(temp) - self.center) / self.radius,
                    material: self.material.clone(),
                });
            }
            let temp = (-b + discriminant.sqrt()) / a;
            if temp > t_min && temp < t_max {
                return Some(HitRecord {
                    t: temp,
                    point: ray.point_at(temp),
                    normal: (ray.point_at(temp) - self.center) / self.radius,
                    material: self.material.clone(),
                });
            }
        }

        None
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<Aabb> {
        Some(Aabb {
            min: self.center - Vec3::new(self.radius, self.radius, self.radius),
            max: self.center + Vec3::new(self.radius, self.radius, self.radius),
        })
    }
}
