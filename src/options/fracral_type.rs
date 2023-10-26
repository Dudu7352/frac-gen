use num::Complex;

#[derive(Clone)]
pub enum FractalType {
    Mandelbrot,
    Julia(Complex<f64>),
}
