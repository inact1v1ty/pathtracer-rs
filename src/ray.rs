use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn point_at(self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}
