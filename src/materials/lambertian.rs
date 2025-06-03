use crate::{vec3, Color, HitRecord, Material, Ray, SolidColor, Texture};

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
    ) -> bool {
        let mut scatter_direction = rec.normal + vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        *scattered = Ray::new(rec.p, scatter_direction, r_in.time());
        *attenuation = self.tex.value(rec.u, rec.v, &rec.p);
        true
    }

    fn emitted(&self, _: f32, _: f32, _: &vec3::Point3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}
