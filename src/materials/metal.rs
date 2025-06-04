use crate::{vec3, Color, HitRecord, Material, Ray};

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
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        _pdf: &mut f32,
    ) -> bool {
        let reflected = crate::vec3::reflect(r_in.direction(), rec.normal);
        let reflected = vec3::unit_vector(reflected) + (self.fuzz * vec3::random_unit_vector());
        *scattered = Ray::new(rec.p, reflected, r_in.time());
        *attenuation = self.albedo;
        vec3::dot(scattered.direction(), rec.normal) > 0.0
    }

    fn emitted(&self, _: &Ray, _: &HitRecord, _: f32, _: f32, _: &vec3::Point3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f32 {
        0.
    }
}
