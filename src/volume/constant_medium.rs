use crate::{
    utils, Color, HitRecord, Hittable, Interval, Isotropic, Material, Point3, Texture, Vec3,
};

use std::sync::Arc;

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    neg_inv_density: f32,
    phase_function: Arc<dyn Material>,
}

impl ConstantMedium {
    pub fn new(boundary: Arc<dyn Hittable>, density: f32, tex: Arc<dyn Texture>) -> Self {
        Self {
            boundary,
            neg_inv_density: -1. / density,
            phase_function: Arc::new(Isotropic::new(tex)),
        }
    }

    pub fn with_color(boundary: Arc<dyn Hittable>, density: f32, albedo: Color) -> Self {
        Self {
            boundary,
            neg_inv_density: -1. / density,
            phase_function: Arc::new(Isotropic::from_color(albedo)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &crate::Ray, ray_t: &mut crate::Interval, rec: &mut crate::HitRecord) -> bool {
        let mut rec1: HitRecord = HitRecord::default();
        let mut rec2: HitRecord = HitRecord::default();
        if !self.boundary.hit(r, &mut Interval::universe(), &mut rec1) {
            return false;
        }
        if !self.boundary.hit(
            r,
            &mut Interval::new(rec1.t + 0.0001, f32::INFINITY),
            &mut rec2,
        ) {
            return false;
        }
        if rec1.t < ray_t.min {
            rec1.t = ray_t.min;
        }
        if rec2.t > ray_t.max {
            rec2.t = ray_t.max;
        }

        if rec1.t >= rec2.t {
            return false;
        }
        if rec1.t < 0. {
            rec1.t = 0.;
        }

        let ray_length = r.direction().length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * f32::ln(utils::random_double());

        if hit_distance > distance_inside_boundary {
            return false;
        }

        rec.t = rec1.t + (hit_distance / ray_length);
        rec.p = r.at(rec.t);

        rec.normal = Vec3::new(1., 0., 0.);
        rec.front_face = true;
        rec.mat = self.phase_function.clone();

        true
    }

    fn bounding_box(&self) -> &super::AABB {
        self.boundary.bounding_box()
    }

    fn random(&self, _origin: Point3) -> Vec3 {
        Vec3::new(1., 0., 0.)
    }

    fn pdf_value(&self, _origin: Point3, _direction: Vec3) -> f32 {
        0.
    }
}
