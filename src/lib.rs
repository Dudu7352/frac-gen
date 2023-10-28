pub mod frac_generator;
pub mod functions;
pub mod options;

#[cfg(test)]
mod tests {
    use num::Complex;

    use crate::{options::{
        frac_type::FracType,
        fractal_options::FractalOptions, gen_method::GenMethod,
    }, frac_generator::FracGenerator};

    #[tokio::test]
    async fn small_image() {
        let res = 100;
        let fractal_options = FractalOptions::new(0.3, res, 0, Complex::new(0.0, 0.0), FracType::Mandelbrot, true);
        let frac_generator = FracGenerator::new(fractal_options, GenMethod::MultithreadAsync { threads: 10 });
        let img = frac_generator.generate_image().await.unwrap();
        assert!(img.width() == res as u32);
    }
}
