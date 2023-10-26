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
}