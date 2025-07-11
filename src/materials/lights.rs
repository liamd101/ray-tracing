use crate::radiometry::{
    sampling,
    spectrum::{self, BlackbodySpectrum},
};
use crate::{Color, Material, Point3, Texture};
use crate::{HitRecord, Ray};

use std::sync::Arc;

#[derive(Clone)]
pub struct DiffuseLight {
    emit: Arc<dyn Texture>,
    emit_spectrum: Arc<dyn spectrum::Spectrum>,
}
impl DiffuseLight {
    pub fn new(emit: Arc<dyn Texture>) -> Self {
        Self {
            emit,
            emit_spectrum: Arc::new(BlackbodySpectrum::new(1000.)),
        }
    }

    pub fn from_color(color: Color) -> Self {
        Self {
            emit: Arc::new(crate::SolidColor::new(color)),
            emit_spectrum: Arc::new(BlackbodySpectrum::new(1000.)),
        }
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, _: &Ray, rec: &HitRecord, u: f32, v: f32, p: Point3) -> Color {
        if !rec.front_face {
            Color::new(0., 0., 0.)
        } else {
            self.emit.value(u, v, p)
        }
    }

    fn emitted_spectrum(
        &self,
        _: &Ray,
        _: &HitRecord,
        _: f32,
        _: f32,
        _: Point3,
        lambda: &sampling::SampledWavelengths,
    ) -> sampling::SampledSpectrum {
        self.emit_spectrum.sample(lambda)
    }
}
