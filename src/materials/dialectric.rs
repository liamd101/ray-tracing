use super::material::{Material, ScatterRecord};

use crate::{utils, vec3, Color, HitRecord, Ray};

use std::sync::Arc;

#[derive(Clone)]
pub struct Dielectric {
    refraction_index: f32,
}
impl Dielectric {
    pub fn new(refraction_index: f32) -> Self {
        Self { refraction_index }
    }

    fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * f32::powi(1.0 - cosine, 5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        srec.attenuation = Color::new(1.0, 1.0, 1.0);
        srec.skip_pdf = true;
        srec.pdf = Arc::new(crate::SpherePdf::new(rec.normal));
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = vec3::unit_vector(r_in.direction());
        let cos_theta = f32::min(vec3::dot(-unit_direction, rec.normal), 1.0);
        let sin_theta = f32::sqrt(1.0 - cos_theta * cos_theta);

        let cannot_refract = (ri * sin_theta) > 1.0;
        let direction: vec3::Vec3 =
            if cannot_refract || Dielectric::reflectance(cos_theta, ri) > utils::random_double() {
                vec3::reflect(unit_direction, rec.normal)
            } else {
                vec3::refract(unit_direction, rec.normal, ri)
            };

        srec.skip_pdf_ray = Ray::new(rec.p, direction, r_in.time());
        true
    }
}
