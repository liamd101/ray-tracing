use crate::{color::Color, hittable::HitRecord, ray::Ray};
use dyn_clone::DynClone;

pub trait Material: DynClone {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
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
}

