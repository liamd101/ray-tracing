use crate::{vec3, Color, HitRecord, Material, Ray, SolidColor, SpherePdf, Texture};

use std::sync::Arc;

use super::material::ScatterRecord;

#[derive(Clone)]
pub struct Isotropic {
    tex: Arc<dyn Texture>,
}
impl Isotropic {
    pub fn from_color(albedo: Color) -> Self {
        Self {
            tex: Arc::new(SolidColor::new(albedo)),
        }
    }

    pub fn new(tex: Arc<dyn Texture>) -> Self {
        Self { tex }
    }
}

impl Material for Isotropic {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        srec.attenuation = self.tex.value(rec.u, rec.v, rec.p);
        srec.pdf = Arc::new(SpherePdf::new(rec.normal));
        srec.skip_pdf = false;
        true
    }

    fn emitted(&self, _: &Ray, _: &HitRecord, _: f32, _: f32, _: vec3::Point3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

    fn scattering_pdf(&self, _: &Ray, _: &HitRecord, _: &Ray) -> f32 {
        1. / (4. * std::f32::consts::PI)
    }
}
