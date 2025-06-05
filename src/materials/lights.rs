use crate::{Color, Material, Point3, Texture};
use crate::{HitRecord, Ray};

use std::sync::Arc;

#[derive(Clone)]
pub struct DiffuseLight {
    emit: Arc<dyn Texture>,
}
impl DiffuseLight {
    pub fn new(emit: Arc<dyn Texture>) -> Self {
        Self { emit }
    }

    pub fn from_color(color: Color) -> Self {
        Self {
            emit: Arc::new(crate::SolidColor::new(color)),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        _: &crate::Ray,
        _: &crate::HitRecord,
        _: &mut crate::ScatterRecord,
    ) -> bool {
        false
    }

    fn emitted(&self, _: &Ray, rec: &HitRecord, u: f32, v: f32, p: Point3) -> Color {
        if !rec.front_face {
            Color::new(0., 0., 0.)
        } else {
            self.emit.value(u, v, p)
        }
    }

    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f32 {
        0.
    }
}
