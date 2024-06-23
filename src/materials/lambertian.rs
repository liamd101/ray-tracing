use crate::{vec3, Color, HitRecord, Material, Ray, SolidColor, Texture};

use std::rc::Rc;

#[derive(Clone)]
pub struct Lambertian {
    tex: Rc<dyn Texture>,
}
impl Lambertian {
    pub fn new(color: Color) -> Self {
        Self {
            tex: Rc::new(SolidColor::new(color)),
        }
    }

    pub fn with_texture(tex: Rc<dyn Texture>) -> Self {
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
}
