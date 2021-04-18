#![deny(clippy::all)]
#![forbid(unsafe_code)]
#![allow(clippy::many_single_char_names)]
#![allow(dead_code)]
#![allow(clippy::identity_op)]

use std::thread;
use std::sync::Arc;
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

mod vec3;
mod ray;
mod tracer;
mod render;
mod hit;
mod camera;

use vec3::Vec3;
use tracer::Tracer;
use hit::{HitableHolder, Sphere};

const DOWNSCALE: u32 = 1;
const WIDTH: u32 = 1280 / DOWNSCALE;
const HEIGHT: u32 = 720 / DOWNSCALE;
const BLOCK_SIZE: u32 = 40 / DOWNSCALE;
const WINDOW_SCALE: u32 = DOWNSCALE * 1;

fn main() -> Result<(), Error> {
    env_logger::init();
    rayon::ThreadPoolBuilder::new().num_threads(4).build_global().unwrap();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new((WIDTH * WINDOW_SCALE) as f64, (HEIGHT * WINDOW_SCALE) as f64);
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

    let world: HitableHolder = Box::new(vec![
        Sphere { center: Vec3::new(0.0, 0.0, -1.0), radius: 0.5 },
        Sphere { center: Vec3::new(0.0, -100.5, -1.0), radius: 100.0 },
    ]);

    let tracer = Arc::new(Tracer::new(WIDTH, HEIGHT, BLOCK_SIZE, world));

    let tracer_clone = tracer.clone();

    thread::spawn(move || {
        tracer_clone.render(1);
        println!("Started rendering");
        let start = std::time::Instant::now();
        tracer_clone.render(100);
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
