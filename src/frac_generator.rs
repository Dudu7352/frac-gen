use image::ImageBuffer;
use log::info;

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

    pub async fn generate_image(&self, threads: usize) -> Result<ImageBuffer<image::Luma<u8>, Vec<u8>>, String> {
        let mut pixels = Vec::with_capacity(self.opts.resolution * self.opts.resolution);
        let mut th = Vec::with_capacity(threads);

        if self.opts.log {
            env_logger::init();
            info!("Starting {} threads", threads);
        }

        for thread_id in 0..threads {
            let end = if thread_id == threads - 1 {
                self.opts.resolution
            } else {
                (self.opts.resolution / threads) * (thread_id + 1)
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

        if self.opts.log {
            info!("Add generator threads finished working");
            info!("Creating Imagebuffer");
            println!("pixels: {}", pixels.len());
        }

        match ImageBuffer::from_raw(
            self.opts.resolution as u32,
            self.opts.resolution as u32,
            pixels,
        ) {
            Some(val) => Ok(val),
            None => Err(String::from("Image could not be created")),
        }
    }
}
