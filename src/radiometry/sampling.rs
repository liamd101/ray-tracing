use crate::core::utils::lerp;
use crate::radiometry::color::{CIE_Y_INT, XYZ};
use crate::radiometry::spectrum::{DenselySampledSpectrum, Spectrum};

pub const NUM_SPECTRUM_SAMPLES: usize = 4;

#[derive(Copy, Clone)]
pub struct SampledWavelengths {
    pub lambdas: [f32; NUM_SPECTRUM_SAMPLES],
    pub pdf: [f32; NUM_SPECTRUM_SAMPLES],
}
impl Default for SampledWavelengths {
    fn default() -> Self {
        Self {
            pdf: [0.; NUM_SPECTRUM_SAMPLES],
            lambdas: [0.; NUM_SPECTRUM_SAMPLES],
        }
    }
}
impl SampledWavelengths {
    pub fn uniform(u: f32, lambda_min: f32, lambda_max: f32) -> Self {
        let mut swl: SampledWavelengths = Default::default();
        swl.lambdas[0] = lerp(u, lambda_min, lambda_max);
        let delta = (lambda_max - lambda_min) / NUM_SPECTRUM_SAMPLES as f32;
        for i in 1..NUM_SPECTRUM_SAMPLES {
            swl.lambdas[i] = swl.lambdas[i - 1] + delta;
            if swl.lambdas[i] > lambda_max {
                swl.lambdas[i] = lambda_min + (swl.lambdas[i] - lambda_max);
            }
        }
        for i in 0..NUM_SPECTRUM_SAMPLES {
            swl.pdf[i] = 1. / (lambda_max - lambda_min);
        }
        swl
    }

    pub fn pdf(&self) -> SampledSpectrum {
        SampledSpectrum::new(&self.pdf)
    }

    pub fn secondary_terminated(&mut self) -> bool {
        self.pdf.iter().any(|&x| x != 0.)
    }

    pub fn terminate_secondary(&mut self) {
        if self.secondary_terminated() {
            return;
        }
        for i in 1..NUM_SPECTRUM_SAMPLES {
            self.pdf[i] = 0.
        }
        self.pdf[0] /= NUM_SPECTRUM_SAMPLES as f32;
    }
}
impl std::ops::Index<usize> for SampledWavelengths {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        &self.lambdas[index]
    }
}
impl std::ops::IndexMut<usize> for SampledWavelengths {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.lambdas[index]
    }
}

#[derive(Copy, Clone)]
pub struct SampledSpectrum {
    pub values: [f32; NUM_SPECTRUM_SAMPLES],
}
impl Default for SampledSpectrum {
    fn default() -> Self {
        Self {
            values: [0.; NUM_SPECTRUM_SAMPLES],
        }
    }
}
impl SampledSpectrum {
    pub fn new(v: &[f32; NUM_SPECTRUM_SAMPLES]) -> Self {
        let mut values: [f32; NUM_SPECTRUM_SAMPLES] = [0.; NUM_SPECTRUM_SAMPLES];
        values[..NUM_SPECTRUM_SAMPLES].copy_from_slice(&v[..NUM_SPECTRUM_SAMPLES]);
        Self { values }
    }

    pub fn call(&self) -> bool {
        for i in 0..NUM_SPECTRUM_SAMPLES {
            if self[i] != 0. {
                return true;
            }
        }
        false
    }

    pub fn safe_div(a: &SampledSpectrum, b: &SampledSpectrum) -> SampledSpectrum {
        let mut r: SampledSpectrum = Default::default();
        for i in 0..NUM_SPECTRUM_SAMPLES {
            r[i] = if b[i] != 0. { a[i] / b[i] } else { 0. };
        }
        r
    }

    pub fn average(&self) -> f32 {
        self.values.iter().sum::<f32>() / (self.values.len() as f32)
    }

    pub fn bool(&self) -> bool {
        self.values.iter().any(|&x| x != 0.)
    }

    pub fn to_xyz(&self, lambda: &SampledWavelengths) -> XYZ {
        let x = DenselySampledSpectrum::x().sample(lambda);
        let y = DenselySampledSpectrum::y().sample(lambda);
        let z = DenselySampledSpectrum::z().sample(lambda);

        let pdf: SampledSpectrum = lambda.pdf();
        XYZ {
            x: Self::safe_div(&(*self * x), &pdf).average(),
            y: Self::safe_div(&(*self * y), &pdf).average(),
            z: Self::safe_div(&(*self * z), &pdf).average(),
        } / CIE_Y_INT
    }
}

impl std::ops::Index<usize> for SampledSpectrum {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}
impl std::ops::IndexMut<usize> for SampledSpectrum {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.values[index]
    }
}

impl std::ops::Sub for SampledSpectrum {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        SampledSpectrum {
            values: std::array::from_fn(|i| self.values[i] - rhs.values[i]),
        }
    }
}
impl std::ops::Add for SampledSpectrum {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        SampledSpectrum {
            values: std::array::from_fn(|i| self.values[i] + rhs.values[i]),
        }
    }
}
impl std::ops::Mul for SampledSpectrum {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        SampledSpectrum {
            values: std::array::from_fn(|i| self.values[i] * rhs.values[i]),
        }
    }
}
impl std::ops::Div for SampledSpectrum {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        SampledSpectrum {
            values: std::array::from_fn(|i| self.values[i] / rhs.values[i]),
        }
    }
}

impl std::ops::DivAssign for SampledSpectrum {
    fn div_assign(&mut self, rhs: Self) {
        for i in 0..NUM_SPECTRUM_SAMPLES {
            self[i] /= rhs[i];
        }
    }
}
impl std::ops::MulAssign for SampledSpectrum {
    fn mul_assign(&mut self, rhs: Self) {
        for i in 0..NUM_SPECTRUM_SAMPLES {
            self[i] *= rhs[i];
        }
    }
}
impl std::ops::SubAssign for SampledSpectrum {
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..NUM_SPECTRUM_SAMPLES {
            self[i] -= rhs[i];
        }
    }
}
impl std::ops::AddAssign for SampledSpectrum {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..NUM_SPECTRUM_SAMPLES {
            self[i] += rhs[i];
        }
    }
}
