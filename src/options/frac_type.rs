use num::Complex;

#[derive(Clone)]
pub enum FracType {
    Mandelbrot,
    Julia(Complex<f64>),
}

