use std::convert::TryInto;
use std::sync::Mutex;

use rayon::prelude::*;

use crate::camera::Camera;
use crate::hit::HitableHandle;

pub struct Tracer {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) block_size: u32,
    buffer: Mutex<Vec<u8>>,
    pub(crate) world: HitableHandle,
    pub(crate) camera: Camera,
}

impl Tracer {
    pub fn new(
        width: u32,
        height: u32,
        block_size: u32,
        world: HitableHandle,
        camera: Camera,
    ) -> Self {
        assert_eq!(width % block_size, 0);
        assert_eq!(height % block_size, 0);

        let buffer = Mutex::new(vec![0; (width * height * 4).try_into().unwrap()]);
        Tracer {
            width,
            height,
            block_size,
            buffer,
            world,
            camera,
        }
    }

    pub fn flush(&self, frame: &mut [u8]) {
        let buffer = self.buffer.lock().unwrap();

        frame.copy_from_slice(&buffer);
    }

    #[allow(clippy::suspicious_operation_groupings)]
    pub fn render(&self, steps: u32) {
        let block_count = (self.width * self.height) / (self.block_size * self.block_size);
        let block_count_in_line = self.width / self.block_size;

        (0..block_count).into_par_iter().for_each(|block_idx| {
            for line_idx in 0..self.block_size {
                let mut line_buffer = vec![0xffu8; (self.block_size * 4).try_into().unwrap()];

                let line_pos = (block_idx % block_count_in_line) * self.block_size
                    + line_idx * self.width
                    + (block_idx / block_count_in_line) * self.width * self.block_size;

                for i in 0..self.block_size {
                    let x = (line_pos + i) % self.width;
                    let y = self.height - ((line_pos + i) / self.width);

                    let color = self.trace(x, y, steps);

                    let rgba = [
                        (color.x * 255.99) as u8,
                        (color.y * 255.99) as u8,
                        (color.z * 255.99) as u8,
                        0xff,
                    ];

                    line_buffer[(i * 4) as usize..((i + 1) * 4) as usize].copy_from_slice(&rgba);
                }

                {
                    let mut buffer = self.buffer.lock().unwrap();

                    buffer[(line_pos * 4) as usize..((line_pos + self.block_size) * 4) as usize]
                        .copy_from_slice(&line_buffer);
                }
            }
        })
    }
}
