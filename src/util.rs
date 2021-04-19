
use rand::Rng;
use crate::vec3::Vec3;

pub fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    let mut p: Vec3;
    loop {
        p = Vec3::new(rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0));
        if p.squared_length() < 1.0 { break }
    }
    p
}
