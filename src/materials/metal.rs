use crate::material::{Material, ScatterRecord};

use crate::{vec3, Color, HitRecord, Ray};

#[derive(Clone)]
pub struct Metal {
    albedo: Color,
    fuzz: f32,
}
impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}
impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        let reflected = crate::vec3::reflect(r_in.direction(), rec.normal);
        let reflected = vec3::unit_vector(reflected) + (self.fuzz * vec3::random_unit_vector());
        srec.attenuation = self.albedo;
        srec.skip_pdf = true;
        srec.skip_pdf_ray = Ray::new(rec.p, reflected, r_in.time());
        true
    }
}
