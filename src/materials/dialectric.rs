use crate::{utils, vec3, Color, HitRecord, Material, Ray};

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
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        _pdf: &mut f32,
    ) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_direction = vec3::unit_vector(r_in.direction());
        let cos_theta = f32::min(vec3::dot(-unit_direction, rec.normal), 1.0);
        let sin_theta = f32::sqrt(1.0 - cos_theta * cos_theta);
        let cannot_refract = ri * sin_theta > 1.0;
        let direction: vec3::Vec3 =
            if cannot_refract || Dielectric::reflectance(cos_theta, ri) > utils::random_double() {
                vec3::reflect(unit_direction, rec.normal)
            } else {
                vec3::refract(unit_direction, rec.normal, ri)
            };
        *scattered = Ray::new(rec.p, direction, r_in.time());
        true
    }

    fn emitted(&self, _: &Ray, _: &HitRecord, _: f32, _: f32, _: &vec3::Point3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f32 {
        0.
    }
}
