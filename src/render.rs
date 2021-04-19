use crate::hit::HitableHandle;
use crate::ray::Ray;
use crate::tracer::Tracer;
use crate::util::local_rng;
use crate::vec3::Vec3;

use rand::Rng;

impl Tracer {
    #[allow(clippy::let_and_return)]
    pub(crate) fn trace(&self, x: u32, y: u32, steps: u32) -> Vec3 {
        let mut rng = local_rng();

        let mut color = Vec3::zero();

        for _ in 0..steps {
            let u = (x as f32 + rng.gen_range(0.0..1.0)) / self.width as f32;
            let v = (y as f32 + rng.gen_range(0.0..1.0)) / self.height as f32;

            let ray = self.camera.get_ray(u, v);

            color += Tracer::color(ray, &self.world, 0);
        }

        color /= steps as f32;
        color = Vec3::new(color.x.sqrt(), color.y.sqrt(), color.z.sqrt());

        color
    }

    fn color(ray: Ray, hitable: &HitableHandle, depth: u32) -> Vec3 {
        match hitable.hit(&ray, 0.001, f32::MAX) {
            Some(hit) => {
                if depth >= 50 {
                    Vec3::zero()
                } else if let Some((scattered, attenuation)) = hit.material.scatter(&ray, &hit) {
                    attenuation * Tracer::color(scattered, hitable, depth + 1)
                } else {
                    Vec3::zero()
                }
            }
            None => {
                let t = 0.5 * (ray.direction.unit().y + 1.0);
                (1.0 - t) * Vec3::one() + t * Vec3::new(0.5, 0.7, 1.0)
            }
        }
    }
}
