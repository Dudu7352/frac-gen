use image::ImageBuffer;

use std::thread;

use crate::{
    functions::calculate_part::calculate_part,
    options::{fractal_options::FractalOptions, render_range::RenderRange, gen_method::GenMethod},
};

pub struct FracGenerator {
    pub opts: FractalOptions,
    pub gen_method: GenMethod
}

impl FracGenerator {
    pub fn new(opts: FractalOptions, gen_method: GenMethod) -> Self {
        Self { opts, gen_method }
    }

    fn to_image_buffer(&self, pixels: Vec<u8>) -> Result<ImageBuffer<image::Luma<u8>, Vec<u8>>, String> {
        match ImageBuffer::from_raw(
            self.opts.resolution as u32,
            self.opts.resolution as u32,
            pixels,
        ) {
            Some(val) => Ok(val),
            None => Err(String::from("Image could not be created")),
        }
    }
    

    pub async fn generate_image(&self) -> Result<ImageBuffer<image::Luma<u8>, Vec<u8>>, String> {
        match self.gen_method {
            GenMethod::SinglethreadAsync => self.generate_image_singlethread().await,
            GenMethod::MultithreadAsync { threads } => self.generate_image_multithread(threads).await,
        }
    }

    async fn generate_image_singlethread(&self) -> Result<ImageBuffer<image::Luma<u8>, Vec<u8>>, String> {
        self.to_image_buffer(
            calculate_part(0, self.opts.clone(), RenderRange::new(0, self.opts.resolution))
        )
    }

    async fn generate_image_multithread(&self, threads: usize) -> Result<ImageBuffer<image::Luma<u8>, Vec<u8>>, String> {
        let mut pixels = Vec::with_capacity(self.opts.resolution * self.opts.resolution);
        let mut th = Vec::with_capacity(threads);

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

        self.to_image_buffer(pixels)
    }
}
