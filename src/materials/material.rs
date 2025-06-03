use crate::{color::Color, hittable::HitRecord, ray::Ray, vec3::Point3};
use dyn_clone::DynClone;

pub trait Material: DynClone + Send + Sync {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;

    fn emitted(&self, u: f32, v: f32, p: &Point3) -> Color;
}

dyn_clone::clone_trait_object!(Material);

#[derive(Clone)]
pub struct NoneMaterial;
impl Material for NoneMaterial {
    fn scatter(
        &self,
        _r_in: &Ray,
        _rec: &HitRecord,
        _attenuation: &mut Color,
        _scattered: &mut Ray,
    ) -> bool {
        false
    }

    fn emitted(&self, _u: f32, _v: f32, _p: &Point3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}
