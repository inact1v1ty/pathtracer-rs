
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::tracer::Tracer;
use crate::hit::HitableHolder;
use crate::camera::Camera;

use rand::Rng;

impl Tracer {
    #[allow(clippy::let_and_return)]
    pub(crate) fn trace(&self, x: u32, y: u32, steps: u32) -> Vec3 {
        let camera = Camera::new(self.width, self.height, self.block_size);

        let mut rng = rand::thread_rng();

        let mut color = Vec3::zero();

        for _ in 0..steps {
            let u = (x as f32 + rng.gen_range(0.0..1.0)) / self.width as f32;
            let v = (y as f32 + rng.gen_range(0.0..1.0)) / self.height as f32;

            let ray = camera.get_ray(u, v);

            color += Tracer::color(ray, &self.world);
        }

        color /= steps as f32;
        color = Vec3::new(color.x.sqrt(), color.y.sqrt(), color.z.sqrt());

        color
    }

    fn color(ray: Ray, hitable: &HitableHolder) -> Vec3 {
        match hitable.hit(0.001, f32::MAX, &ray) {
            Some(hit) => {
                let target = hit.point + hit.normal + Tracer::random_in_unit_sphere();
                0.5 * Tracer::color(Ray { origin: hit.point, direction: target - hit.point }, hitable)
            }
            None => {
                let t = 0.5 * (ray.direction.unit().y + 1.0);
                (1.0 - t) * Vec3::one() + t * Vec3::new(0.5, 0.7, 1.0)
            }
        }
    }

    fn random_in_unit_sphere() -> Vec3 {
        let mut rng = rand::thread_rng();
        let mut p: Vec3;
        loop {
            p = Vec3::new(rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0));
            if p.squared_length() < 1.0 { break }
        }
        p
    }
}
