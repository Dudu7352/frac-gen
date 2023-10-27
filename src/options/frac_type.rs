use num::Complex;

use crate::functions::get_val::get_val_u8;

#[derive(Clone)]
pub enum FracType {
    Mandelbrot,
    Julia{ c: Complex<f64> },
}

impl FracType {
    pub fn get_val(&self, pos: Complex<f64>) -> u8 {
        match &self {
            FracType::Mandelbrot => get_val_u8(Complex::new(0.0, 0.0), pos, 2.0),
            FracType::Julia{ c } => get_val_u8(pos, *c, 2.0),
        }
    }
}