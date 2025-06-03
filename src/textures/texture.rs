use crate::{Color, Point3};

pub trait Texture: Send + Sync {
    fn value(&self, u: f32, v: f32, p: &Point3) -> Color;
}

#[derive(Default, Clone)]
pub struct SolidColor {
    albedo: Color,
}
impl Texture for SolidColor {
    fn value(&self, _: f32, _: f32, _: &Point3) -> Color {
        self.albedo
    }
}
impl SolidColor {
    pub fn new(color: Color) -> Self {
        SolidColor { albedo: color }
    }

    pub fn from_rgb(red: f32, green: f32, blue: f32) -> Self {
        SolidColor {
            albedo: Color::new(red, green, blue),
        }
    }
}
