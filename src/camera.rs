use crate::vec3::Vec3;
use crate::ray::Ray;

const WORLD_SCALE: f32 = 0.2;

pub struct Camera {
    pub lower_left: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub origin: Vec3,
}

impl Camera {
    pub fn new(width: u32, height: u32, block_size: u32) -> Self {
        let camera_size = Vec3::new(width as f32 / block_size as f32, height as f32 / block_size as f32, 0.0) * WORLD_SCALE;
        Camera {
            lower_left: -0.5 * camera_size + Vec3::new(0.0, 0.0, -1.0),
            horizontal: Vec3::new(camera_size.x, 0.0, 0.0),
            vertical: Vec3::new(0.0, camera_size.y, 0.0),
            origin: Vec3::zero()
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left + u * self.horizontal + v * self.vertical
        }
    }
}
