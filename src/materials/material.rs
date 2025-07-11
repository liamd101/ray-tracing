use crate::radiometry::sampling;
use crate::{vec3, Color, HitRecord, Ray};

use crate::{Pdf, SpherePdf};

use dyn_clone::DynClone;
use std::sync::Arc;

pub struct ScatterRecord {
    pub attenuation: Color,
    pub pdf: Arc<dyn Pdf>,
    pub skip_pdf: bool,
    pub skip_pdf_ray: Ray,
}
impl Default for ScatterRecord {
    fn default() -> Self {
        Self {
            attenuation: Color::default(),
            pdf: Arc::new(SpherePdf::new(vec3::Vec3::default())),
            skip_pdf: true,
            skip_pdf_ray: Ray::default(),
        }
    }
}

#[allow(unused_variables)]
pub trait Material: DynClone + Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        false
    }

    fn emitted(&self, r_in: &Ray, rec: &HitRecord, u: f32, v: f32, p: vec3::Point3) -> Color {
        Color::new(0., 0., 0.)
    }

    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f32 {
        0.
    }

    fn emitted_spectrum(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        u: f32,
        v: f32,
        p: vec3::Point3,
        lambda: &sampling::SampledWavelengths,
    ) -> sampling::SampledSpectrum {
        todo!()
    }
}

dyn_clone::clone_trait_object!(Material);

#[derive(Clone)]
pub struct NoneMaterial;
impl Material for NoneMaterial {}
