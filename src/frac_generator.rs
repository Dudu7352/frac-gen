use image::ImageBuffer;

use std::thread;

use crate::options::fractal_options::FractalOptions;

pub struct FracGenerator {
    pub opts: FractalOptions
}

impl FracGenerator {
    pub fn new(opts: FractalOptions) -> Self { 
        Self { 
            opts
        } 
    }

    pub async fn generate_image(&self, threads: usize) -> ImageBuffer<image::Luma<u8>, Vec<u8>> {
        let mut pixels = Vec::with_capacity(self.opts.resolution * self.opts.resolution);
        let mut th = Vec::with_capacity(threads);

        for i in 0..threads {
            
            let end = if i == threads - 1 {
                self.opts.resolution
            } else {
                (self.opts.resolution / threads) * i + 1
            };

            let opts_clone = self.opts.clone();
            th.push(thread::spawn(move || {
                //TODO: image part calculation
                vec![]
            }));
        }

        for t in th {
            pixels.append(&mut t.join().unwrap());
        }

        ImageBuffer::from_raw(self.opts.resolution as u32, self.opts.resolution as u32, pixels).unwrap()
    }
}