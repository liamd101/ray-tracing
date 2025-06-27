use crate::core::utils::lerp;
use crate::radiometry::utils::{blackbody, find_interval, LAMBDA_MAX, LAMBDA_MIN};

const NUM_SPECTRUM_SAMPLES: usize = 4;

struct SampledWavelengths {
    lambdas: [f32; NUM_SPECTRUM_SAMPLES],
    values: [f32; NUM_SPECTRUM_SAMPLES],
    pdf: [f32; NUM_SPECTRUM_SAMPLES],
}
impl Default for SampledWavelengths {
    fn default() -> Self {
        Self {
            values: [0.; NUM_SPECTRUM_SAMPLES],
            pdf: [0.; NUM_SPECTRUM_SAMPLES],
            lambdas: [0.; NUM_SPECTRUM_SAMPLES],
        }
    }
}
impl SampledWavelengths {
    pub fn uniform(u: f32, lambda_min: f32, lambda_max: f32) -> Self {
        let mut swl: SampledWavelengths = Default::default();
        swl.lambdas[0] = lerp(u, lambda_min, lambda_max);
        let _delta = (lambda_max - lambda_min) / NUM_SPECTRUM_SAMPLES as f32;
        for _i in 1..NUM_SPECTRUM_SAMPLES {}
        swl
    }
}

struct SampledSpectrum {
    values: [f32; NUM_SPECTRUM_SAMPLES],
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
        for i in 0..NUM_SPECTRUM_SAMPLES {
            values[i] = v[i];
        }
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

    pub fn safe_div(a: SampledSpectrum, b: SampledSpectrum) -> SampledSpectrum {
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
        let mut out: Self = Default::default();
        for i in 0..NUM_SPECTRUM_SAMPLES {
            out[i] = self[i] - rhs[i];
        }
        out
    }
}
impl std::ops::Add for SampledSpectrum {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let mut out: Self = Default::default();
        for i in 0..NUM_SPECTRUM_SAMPLES {
            out[i] = self[i] + rhs[i];
        }
        out
    }
}
impl std::ops::Mul for SampledSpectrum {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let mut out: Self = Default::default();
        for i in 0..NUM_SPECTRUM_SAMPLES {
            out[i] = self[i] * rhs[i];
        }
        out
    }
}
impl std::ops::Div for SampledSpectrum {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        let mut out: Self = Default::default();
        for i in 0..NUM_SPECTRUM_SAMPLES {
            out[i] = self[i] / rhs[i];
        }
        out
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

/// Implementation of a spectral distributions
trait Spectrum {
    /// Takes in a wavelength and returns the value of the distribution at that wavelength
    fn call(&self, wavelength: f32) -> f32;
    /// Returns the maximum value achieved by the corresponding distribution
    fn max_value(&self) -> f32;
    /// Samples all values in `wavelengths` using `Spectrum::call` method on each wavelength
    fn sample(&self, wavelengths: &SampledWavelengths) -> SampledSpectrum;
}

struct ConstantSpectrum {
    c: f32,
}
impl ConstantSpectrum {
    pub fn new(c: f32) -> Self {
        Self { c }
    }
}

impl Spectrum for ConstantSpectrum {
    fn call(&self, _: f32) -> f32 {
        self.c
    }
    fn max_value(&self) -> f32 {
        self.c
    }
    fn sample(&self, _: &SampledWavelengths) -> SampledSpectrum {
        SampledSpectrum::default()
    }
}

/// An implementation of an illumination spectrum
/// This is the most accurate representation of an illumination spectrum I have implemented
/// For every wavelength in between `lambda_min` and `lambda_max`, it stores the corresponding
/// value. For the entire visible light range, this corresponds to 470 values
struct DenselySampledSpectrum {
    lambda_min: isize,
    lambda_max: isize,
    values: Vec<f32>,
}
impl Default for DenselySampledSpectrum {
    fn default() -> Self {
        Self::from_range(LAMBDA_MIN as usize, LAMBDA_MAX as usize)
    }
}
impl DenselySampledSpectrum {
    pub fn from_range(lambda_min: usize, lambda_max: usize) -> Self {
        Self {
            lambda_min: lambda_min as isize,
            lambda_max: lambda_max as isize,
            values: vec![0.; (lambda_min - lambda_max) as usize + 1],
        }
    }

    pub fn new(spec: Option<&dyn Spectrum>, lambda_min: usize, lambda_max: usize) -> Self {
        let mut values: Vec<f32> = vec![0.; lambda_max - lambda_min + 1];
        if let Some(spec) = spec {
            for lambda in lambda_min..=lambda_max {
                values[lambda - lambda_min] = spec.call(lambda as f32);
            }
        }
        Self {
            lambda_min: lambda_min as isize,
            lambda_max: lambda_max as isize,
            values,
        }
    }

    pub fn scale(&mut self, s: f32) {
        for num in &mut self.values {
            *num *= s;
        }
    }
}

impl Spectrum for DenselySampledSpectrum {
    fn call(&self, lambda: f32) -> f32 {
        let offset = lambda.round() as isize - self.lambda_min as isize;
        if offset < 0 || offset >= self.values.len() as isize {
            0.
        } else {
            self.values[offset as usize]
        }
    }

    fn max_value(&self) -> f32 {
        self.values
            .iter()
            .max_by(|x, y| x.partial_cmp(&y).unwrap())
            .copied()
            .unwrap_or(0.)
    }

    fn sample(&self, wavelengths: &SampledWavelengths) -> SampledSpectrum {
        let mut s: SampledSpectrum = Default::default();
        for i in 0..NUM_SPECTRUM_SAMPLES {
            let offset = wavelengths.values[i].round() as isize - self.lambda_min as isize;
            if offset < 0 || offset >= self.values.len() as isize {
                s.values[i] = 0.;
            } else {
                s.values[i] = self.values[i];
            }
        }
        s
    }
}

impl PartialEq for DenselySampledSpectrum {
    fn eq(&self, rhs: &DenselySampledSpectrum) -> bool {
        if self.lambda_min != rhs.lambda_min
            || self.lambda_max != rhs.lambda_max
            || self.values.len() != rhs.values.len()
        {
            return false;
        }

        for (l, r) in self.values.iter().cloned().zip(rhs.values.iter().cloned()) {
            if l != r {
                return false;
            }
        }

        true
    }
}

/// A piecewise linear representation of a illumination spectrum
/// It stores vectors of wavelength-value pairs, which are linearly interpolated to achieve an
/// estimate of a real illumination spectrum
#[derive(Default)]
struct PiecewiseLinearSpectrum {
    /// The wavelengths to be used in interpolation
    lambdas: Vec<f32>,
    /// The corresponding values to be used in interpolation
    values: Vec<f32>,
}

impl Spectrum for PiecewiseLinearSpectrum {
    fn call(&self, wavelength: f32) -> f32 {
        if self.lambdas.is_empty()
            || wavelength < *self.lambdas.first().unwrap()
            || wavelength > *self.lambdas.last().unwrap()
        {
            return 0.;
        }
        let o: usize = find_interval(self.lambdas.len(), |lambda| {
            self.lambdas[lambda] < wavelength
        });
        let t: f32 = (wavelength - self.lambdas[o]) / (self.lambdas[o + 1] - self.lambdas[o]);
        lerp(t, self.lambdas[o], self.lambdas[o + 1])
    }

    fn sample(&self, wavelengths: &SampledWavelengths) -> SampledSpectrum {
        let mut s: SampledSpectrum = Default::default();
        for i in 0..NUM_SPECTRUM_SAMPLES {
            s.values[i] = self.call(i as f32);
        }
        s
    }

    fn max_value(&self) -> f32 {
        self.values
            .iter()
            .max_by(|x, y| x.partial_cmp(&y).unwrap())
            .copied()
            .unwrap_or(0.)
    }
}

/// A spectrum corresponding to a blackbody illuminator at a specific temperature
struct BlackbodySpectrum {
    temp: f32,
    norm_factor: f32,
}
impl BlackbodySpectrum {
    pub fn new(temp: f32) -> Self {
        let lambda_max = 2.8977721e-3 / temp;
        Self {
            temp,
            norm_factor: blackbody(lambda_max * 1e-9, temp),
        }
    }
}

impl Spectrum for BlackbodySpectrum {
    fn call(&self, wavelength: f32) -> f32 {
        blackbody(wavelength, self.temp) * self.norm_factor
    }

    fn max_value(&self) -> f32 {
        todo!()
    }

    fn sample(&self, wavelengths: &SampledWavelengths) -> SampledSpectrum {
        let mut s: SampledSpectrum = Default::default();
        for i in 0..NUM_SPECTRUM_SAMPLES {
            s.values[i] = self.call(wavelengths.values[i]);
        }
        s
    }
}
