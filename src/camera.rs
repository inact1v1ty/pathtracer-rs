use crate::ray::Ray;
use crate::vec3::Vec3;

use rand::Rng;

#[derive(Debug, Clone)]
pub struct Camera {
    pub lower_left: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub origin: Vec3,
    pub lens_radius: f32,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        vup: Vec3,
        v_fov: f32,
        aspect_ratio: f32,
        focus_dist: f32,
        aperture: f32,
    ) -> Self {
        let lens_radius = aperture / 2.0;
        let theta = v_fov.to_radians();
        let half_height = (theta / 2.0).tan();
        let half_width = half_height * aspect_ratio;
        let origin = look_from;
        let w = (look_from - look_at).unit();
        let u = Vec3::cross(vup, w).unit();
        let v = Vec3::cross(w, u);
        Camera {
            lower_left: origin
                - half_width * focus_dist * u
                - half_height * focus_dist * v
                - focus_dist * w,
            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height * focus_dist * v,
            origin,
            u,
            v,
            w,
            lens_radius,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        Ray {
            origin: self.origin + offset,
            direction: self.lower_left + s * self.horizontal + t * self.vertical
                - self.origin
                - offset,
        }
    }
}

pub fn random_in_unit_disk() -> Vec3 {
    let mut rng = rand::thread_rng();
    let mut p: Vec3;
    loop {
        p = Vec3::new(rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0), 0.0);
        if Vec3::dot(p, p) < 1.0 {
            break;
        }
    }
    p
}
