use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;

use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{dot, Point3, Vec3};
use crate::aabb::AABB;

pub struct Sphere {
    center: Point3,
    radius: f32,
    mat: Box<dyn Material>,
    bbox: AABB,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32, mat: Box<dyn Material>) -> Self {
        let rvec = Vec3([radius, radius, radius]);
        let bbox = AABB::around_points(center - rvec, center + rvec);
        Sphere {
            center,
            radius,
            mat,
            bbox,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool {
        let oc = self.center - r.origin();
        let a = r.direction().length_squared();
        let h = dot(r.direction(), oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = (h * h) - (a * c);
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.mat = self.mat.clone();

        true
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}
