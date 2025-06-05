use crate::{color::Color, hittable::HitRecord, ray::Ray, vec3::Point3, Pdf, SpherePdf, Vec3};
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
            pdf: Arc::new(SpherePdf::new(Vec3::default())),
            skip_pdf: true,
            skip_pdf_ray: Ray::default(),
        }
    }
}

pub trait Material: DynClone + Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool;

    fn emitted(&self, r_in: &Ray, rec: &HitRecord, u: f32, v: f32, p: Point3) -> Color;

    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f32;
}

dyn_clone::clone_trait_object!(Material);

#[derive(Clone)]
pub struct NoneMaterial;
impl Material for NoneMaterial {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord, _srec: &mut ScatterRecord) -> bool {
        false
    }

    fn emitted(&self, _: &Ray, _: &HitRecord, _u: f32, _v: f32, _p: Point3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

    fn scattering_pdf(&self, _: &Ray, _: &HitRecord, _: &Ray) -> f32 {
        0.
    }
}
