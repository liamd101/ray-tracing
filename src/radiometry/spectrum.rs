use crate::core::utils::lerp;
use crate::radiometry::color::XYZ;
use crate::radiometry::sampling::{SampledSpectrum, SampledWavelengths, NUM_SPECTRUM_SAMPLES};
use crate::radiometry::utils::{blackbody, find_interval, LAMBDA_MAX, LAMBDA_MIN};

/// Implementation of a spectral distributions
pub trait Spectrum: Sync + Send {
    /// Takes in a wavelength and returns the value of the distribution at that wavelength
    fn call(&self, wavelength: f32) -> f32;
    /// Returns the maximum value achieved by the corresponding distribution
    fn max_value(&self) -> f32;
    /// Samples all values in `wavelengths` using `Spectrum::call` method on each wavelength
    fn sample(&self, wavelengths: &SampledWavelengths) -> SampledSpectrum;
}

fn inner_product(f: &dyn Spectrum, g: &dyn Spectrum) -> f32 {
    let mut integral = 0.;
    for lambda in (LAMBDA_MIN as usize)..=(LAMBDA_MAX as usize) {
        integral += f.call(lambda as f32) * g.call(lambda as f32);
    }
    integral
}
fn spectrum_to_XYZ(s: &dyn Spectrum) -> XYZ {
    XYZ {
        X: inner_product(&DenselySampledSpectrum::X(), s),
        Y: inner_product(&DenselySampledSpectrum::Y(), s),
        Z: inner_product(&DenselySampledSpectrum::Z(), s),
    }
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
pub struct DenselySampledSpectrum {
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

    pub fn X() -> Self {
        todo!()
    }
    pub fn Y() -> Self {
        todo!()
    }
    pub fn Z() -> Self {
        todo!()
    }
}

impl Spectrum for DenselySampledSpectrum {
    fn call(&self, lambda: f32) -> f32 {
        let offset = lambda.round() as isize - self.lambda_min;
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
            let offset = wavelengths.lambdas[i].round() as isize - self.lambda_min;
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
pub struct PiecewiseLinearSpectrum {
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

    fn sample(&self, _: &SampledWavelengths) -> SampledSpectrum {
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
pub struct BlackbodySpectrum {
    temp: f32,
    norm_factor: f32,
}
impl BlackbodySpectrum {
    pub fn new(temp: f32) -> Self {
        let lambda_max = 2.897_772e-3 / temp;
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
            s.values[i] = self.call(wavelengths.lambdas[i]);
        }
        s
    }
}
