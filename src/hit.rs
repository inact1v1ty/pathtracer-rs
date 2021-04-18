use crate::vec3::Vec3;
use crate::ray::Ray;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct HitRecord {
    pub t: f32,
    pub point: Vec3,
    pub normal: Vec3,
}

pub trait Hitable {
    fn hit(&self, t_min: f32, t_max: f32, ray: &Ray) -> Option<HitRecord>;
}

pub type HitableHolder = Box<dyn Hitable + Sync + Send>;

impl<T> Hitable for Vec<T>
where
    T: Hitable {

    fn hit(&self, t_min: f32, t_max: f32, ray: &Ray) -> Option<HitRecord> {
        let (hit, _) = self.iter()
            .fold((None, t_max), |(res, closest_so_far): (Option<HitRecord>, f32), h| {
                match h.hit(t_min, closest_so_far, ray) {
                    Some(hit_record) => (Some(hit_record), hit_record.t),
                    None => (res, closest_so_far)
                }
            });
        
        hit
    }
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Hitable for Sphere {
    #[allow(clippy::suspicious_operation_groupings)]
    fn hit(&self, t_min: f32, t_max: f32, ray: &Ray) -> Option<HitRecord> {
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
                    normal: (ray.point_at(temp) - self.center) / self.radius
                })
            }
            
            let temp = (-b + discriminant.sqrt()) / a;
            if temp > t_min && temp < t_max {
                return Some(HitRecord {
                    t: temp,
                    point: ray.point_at(temp),
                    normal: (ray.point_at(temp) - self.center) / self.radius
                })
            }
        }

        None
    }
}
