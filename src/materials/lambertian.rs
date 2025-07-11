use super::material::{Material, ScatterRecord};
use crate::{vec3, Color, CosinePdf, HitRecord, Ray, SolidColor, Texture};

use std::sync::Arc;

#[derive(Clone)]
pub struct Lambertian {
    tex: Arc<dyn Texture>,
}
impl Lambertian {
    pub fn new(color: Color) -> Self {
        Self {
            tex: Arc::new(SolidColor::new(color)),
        }
    }

    pub fn with_texture(tex: Arc<dyn Texture>) -> Self {
        Self { tex }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        srec.attenuation = self.tex.value(rec.u, rec.v, rec.p);
        srec.pdf = Arc::new(CosinePdf::new(rec.normal));
        srec.skip_pdf = false;
        true
    }

    fn scattering_pdf(&self, _: &Ray, rec: &HitRecord, scattered: &Ray) -> f32 {
        let cos_theta = vec3::dot(rec.normal, vec3::unit_vector(scattered.direction()));
        if cos_theta < 0. {
            0.
        } else {
            cos_theta / std::f32::consts::PI
        }
    }
}
