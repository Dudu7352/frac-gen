use image::ImageBuffer;

use std::thread;

use crate::{
    functions::calculate_part::calculate_part,
    options::{fractal_options::FractalOptions, render_range::RenderRange},
};

pub struct FracGenerator {
    pub opts: FractalOptions,
}

impl FracGenerator {
    pub fn new(opts: FractalOptions) -> Self {
        Self { opts }
    }

    pub async fn generate_image(&self, threads: usize) -> ImageBuffer<image::Luma<u8>, Vec<u8>> {
        let mut pixels = Vec::with_capacity(self.opts.resolution * self.opts.resolution);
        let mut th = Vec::with_capacity(threads);

        for thread_id in 0..threads {
            let end = if thread_id == threads - 1 {
                self.opts.resolution
            } else {
                (self.opts.resolution / threads) * thread_id + 1
            };

            let opts_clone = self.opts.clone();
            let render_range = RenderRange::new((opts_clone.resolution / threads) * thread_id, end);
            th.push(thread::spawn(move || {
                calculate_part(thread_id, opts_clone, render_range)
            }));
        }

        for t in th {
            pixels.append(&mut t.join().unwrap());
        }

        ImageBuffer::from_raw(
            self.opts.resolution as u32,
            self.opts.resolution as u32,
            pixels,
        )
        .unwrap()
    }
}
