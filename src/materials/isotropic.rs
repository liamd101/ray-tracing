use crate::{vec3, Color, HitRecord, Material, Ray, SolidColor, Texture};

use std::sync::Arc;

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
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        _pdf: &mut f32,
    ) -> bool {
        *scattered = Ray::new(rec.p, vec3::random_unit_vector(), r_in.time());
        *attenuation = self.tex.value(rec.u, rec.v, &rec.p);
        true
    }

    // default emission (we are not a light)
    fn emitted(&self, _: f32, _: f32, _: &vec3::Point3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

    fn scattering_pdf(&self, _: &Ray, _: &HitRecord, _: &Ray) -> f32 {
        0.
    }
}
