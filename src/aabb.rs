use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb {
    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        let (mut t_min, mut t_max) = (t_min, t_max);

        for axis in 0..3 {
            let t0 = ffmin(
                (self.min[axis] - ray.origin[axis]) / ray.direction[axis],
                (self.max[axis] - ray.origin[axis]) / ray.direction[axis],
            );

            let t1 = ffmax(
                (self.min[axis] - ray.origin[axis]) / ray.direction[axis],
                (self.max[axis] - ray.origin[axis]) / ray.direction[axis],
            );

            t_min = ffmax(t0, t_min);
            t_max = ffmin(t1, t_max);

            if t_max <= t_min {
                return false;
            }
        }

        true
    }

    pub fn merge(a: Aabb, b: Aabb) -> Aabb {
        let small = Vec3::new(
            a.min.x.min(b.min.x),
            a.min.y.min(b.min.y),
            a.min.z.min(b.min.z),
        );
        let big = Vec3::new(
            a.max.x.max(b.max.x),
            a.max.y.max(b.max.y),
            a.max.z.max(b.max.z),
        );

        Aabb {
            min: small,
            max: big,
        }
    }
}

#[inline]
fn ffmin(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}

#[inline]
fn ffmax(a: f32, b: f32) -> f32 {
    if a > b {
        a
    } else {
        b
    }
}
