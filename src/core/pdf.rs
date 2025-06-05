use crate::{onb, utils, vec3, Hittable, Point3, Vec3};

use std::sync::Arc;

pub trait Pdf {
    fn generate(&self) -> Vec3;

    fn value(&self, direction: Vec3) -> f32;
}

pub struct SpherePdf;
impl SpherePdf {
    pub fn new(_: Vec3) -> Self {
        Self
    }
}

impl Pdf for SpherePdf {
    fn value(&self, _direction: Vec3) -> f32 {
        1. / (4. * std::f32::consts::PI)
    }

    fn generate(&self) -> Vec3 {
        vec3::random_unit_vector()
    }
}

pub struct CosinePdf {
    uvw: onb::ONB,
}
impl CosinePdf {
    pub fn new(w: Vec3) -> Self {
        Self {
            uvw: onb::ONB::new(w),
        }
    }
}

impl Pdf for CosinePdf {
    fn value(&self, direction: Vec3) -> f32 {
        let cosine_theta = vec3::dot(vec3::unit_vector(direction), self.uvw.w());
        (cosine_theta / std::f32::consts::PI).max(0.)
    }

    fn generate(&self) -> Vec3 {
        self.uvw.transform(vec3::random_cosine_direction())
    }
}

pub struct HittablePdf {
    objects: Arc<dyn Hittable>,
    origin: Point3,
}

impl HittablePdf {
    pub fn new(objects: Arc<dyn Hittable>, origin: Point3) -> Self {
        Self { objects, origin }
    }
}

impl Pdf for HittablePdf {
    fn generate(&self) -> Vec3 {
        self.objects.random(self.origin)
    }

    fn value(&self, direction: Vec3) -> f32 {
        self.objects.pdf_value(self.origin, direction)
    }
}

pub struct MixturePdf {
    pdfs: [Arc<dyn Pdf>; 2],
}
impl MixturePdf {
    pub fn new(pdf0: Arc<dyn Pdf>, pdf1: Arc<dyn Pdf>) -> Self {
        Self { pdfs: [pdf0, pdf1] }
    }
}

impl Pdf for MixturePdf {
    fn generate(&self) -> Vec3 {
        if utils::random_double() < 0.5 {
            self.pdfs[0].generate()
        } else {
            self.pdfs[1].generate()
        }
    }

    fn value(&self, direction: Vec3) -> f32 {
        (0.5 * self.pdfs[0].value(direction)) + (0.5 * self.pdfs[1].value(direction))
    }
}
