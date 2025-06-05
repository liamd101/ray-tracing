use crate::{vec3, Color, HitRecord, Material, Ray, SolidColor, Texture, ONB};

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
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        pdf: &mut f32,
    ) -> bool {
        let uvw = ONB::new(rec.normal);
        // let mut scatter_direction = rec.normal + vec3::random_unit_vector();
        let scatter_direction = uvw.transform(vec3::random_cosine_direction());
        *scattered = Ray::new(rec.p, vec3::unit_vector(scatter_direction), r_in.time());
        *attenuation = self.tex.value(rec.u, rec.v, &rec.p);
        *pdf = vec3::dot(uvw.w(), scattered.direction()) / std::f32::consts::PI;
        true
    }

    fn emitted(&self, _: &Ray, _: &HitRecord, _: f32, _: f32, _: &vec3::Point3) -> Color {
        Color::new(0.0, 0.0, 0.0)
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
