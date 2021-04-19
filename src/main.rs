#![deny(clippy::all)]
//#![forbid(unsafe_code)]
#![allow(clippy::many_single_char_names)]
#![allow(dead_code)]
#![allow(clippy::identity_op)]

use image::RgbaImage;
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use rand::Rng;
use std::convert::TryInto;
use std::sync::Arc;
use std::thread;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

mod aabb;
mod bvh;
mod camera;
mod hit;
mod iter_util;
mod material;
mod ray;
mod render;
mod tracer;
mod util;
mod vec3;

use bvh::BvhNode;
use camera::Camera;
use hit::{HitableHandle, Sphere};
use material::{Dielectric, Lambertian, MaterialHandle, Metal};
use tracer::Tracer;
use vec3::Vec3;

const DOWNSCALE: u32 = 1;
const WIDTH: u32 = 1280 / DOWNSCALE;
const HEIGHT: u32 = 720 / DOWNSCALE;
const BLOCK_SIZE: u32 = 40 / DOWNSCALE;
const WINDOW_SCALE: u32 = DOWNSCALE * 1;

fn main() -> Result<(), Error> {
    env_logger::init();
    rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build_global()
        .unwrap();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(
            (WIDTH * WINDOW_SCALE) as f64,
            (HEIGHT * WINDOW_SCALE) as f64,
        );
        WindowBuilder::new()
            .with_title("Pathtracer")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    let world: HitableHandle = random_scene();

    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let aspect_ratio = WIDTH as f32 / HEIGHT as f32;
    let focus_dist = 10.0;
    let aperture = 0.1;
    let camera = Camera::new(
        look_from,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
        focus_dist,
        aperture,
    );

    let tracer = Arc::new(Tracer::new(WIDTH, HEIGHT, BLOCK_SIZE, world, camera));

    let tracer_clone = tracer.clone();

    thread::spawn(move || {
        tracer_clone.render(1);
        save_screenshot(&tracer_clone);

        println!("Started rendering");
        let start = std::time::Instant::now();
        tracer_clone.render(1000);
        save_screenshot(&tracer_clone);
        println!("Rendering complete, took {:?}", start.elapsed());
    });

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            tracer.flush(pixels.get_frame());
            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize(size.width, size.height);
            }

            // Update internal state and request a redraw
            window.request_redraw();
        }
    });
}

fn save_screenshot(tracer: &Tracer) {
    let mut buf = vec![0; (WIDTH * HEIGHT * 4).try_into().unwrap()];
    tracer.flush(&mut buf);

    let img = RgbaImage::from_vec(WIDTH, HEIGHT, buf).unwrap();

    img.save("renders/render.png").unwrap();
}

fn random_scene() -> HitableHandle {
    let mut rng = rand::thread_rng();

    let mut world: Vec<Arc<HitableHandle>> = Vec::with_capacity(500);

    world.push(Arc::new(Box::new(Sphere {
        center: Vec3::new(0.0, -1000.0, -1.0),
        radius: 1000.0,
        material: Arc::new(Box::new(Lambertian {
            albedo: Vec3::new(0.5, 0.5, 0.5),
        })),
    })));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen_range(0.0..1.0);
            let center = Vec3::new(
                a as f32 + 0.9 * rng.gen_range(0.0..1.0),
                0.2,
                b as f32 + 0.9 * rng.gen_range(0.0..1.0),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let material: Arc<MaterialHandle>;

                if choose_mat < 0.8 {
                    // diffuse
                    material = Arc::new(Box::new(Lambertian {
                        albedo: Vec3::new(
                            rng.gen_range(0.0..1.0) * rng.gen_range(0.0..1.0),
                            rng.gen_range(0.0..1.0) * rng.gen_range(0.0..1.0),
                            rng.gen_range(0.0..1.0) * rng.gen_range(0.0..1.0),
                        ),
                    }));
                } else if choose_mat < 0.95 {
                    // metal
                    material = Arc::new(Box::new(Metal {
                        albedo: Vec3::new(
                            0.5 * (1.0 + rng.gen_range(0.0..1.0)),
                            0.5 * (1.0 + rng.gen_range(0.0..1.0)),
                            0.5 * (1.0 + rng.gen_range(0.0..1.0)),
                        ),
                        fuzz: 0.5 * rng.gen_range(0.0..1.0),
                    }));
                } else {
                    // glass
                    material = Arc::new(Box::new(Dielectric {
                        refraction_idx: 1.5,
                    }));
                }

                world.push(Arc::new(Box::new(Sphere {
                    center,
                    radius: 0.2,
                    material,
                })));
            }
        }
    }
    world.push(Arc::new(Box::new(Sphere {
        center: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Box::new(Dielectric {
            refraction_idx: 1.5,
        })),
    })));
    world.push(Arc::new(Box::new(Sphere {
        center: Vec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Box::new(Lambertian {
            albedo: Vec3::new(0.4, 0.2, 0.1),
        })),
    })));
    world.push(Arc::new(Box::new(Sphere {
        center: Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Box::new(Metal {
            albedo: Vec3::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        })),
    })));

    Box::new(BvhNode::new(&mut world[..], 0.0, 0.0))
}
