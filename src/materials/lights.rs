use crate::{Color, Material, Point3, Texture};

use std::rc::Rc;

#[derive(Clone)]
pub struct DiffuseLight {
    emit: Rc<dyn Texture>,
}
impl DiffuseLight {
    pub fn new(emit: Rc<dyn Texture>) -> Self {
        Self { emit }
    }

    pub fn from_color(color: Color) -> Self {
        Self {
            emit: Rc::new(crate::SolidColor::new(color)),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        _: &crate::Ray,
        _: &crate::HitRecord,
        _: &mut crate::Color,
        _: &mut crate::Ray,
    ) -> bool {
        false
    }

    fn emitted(&self, u: f32, v: f32, p: &Point3) -> Color {
        self.emit.value(u, v, p)
    }
}
