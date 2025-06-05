use crate::{
    utils::degrees_to_radians, vec3, Interval, Material, NoneMaterial, Point3, Ray, Vec3, AABB,
};

use std::sync::Arc;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Arc<dyn Material>,
    pub t: f32,
    pub u: f32,
    pub v: f32,
    pub front_face: bool,
}

impl Default for HitRecord {
    fn default() -> Self {
        HitRecord {
            p: Default::default(),
            normal: Default::default(),
            mat: Arc::new(NoneMaterial),
            t: 0.0,
            u: 0.0,
            v: 0.0,
            front_face: false,
        }
    }
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = vec3::dot(r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool;

    fn bounding_box(&self) -> &AABB;

    fn pdf_value(&self, origin: Point3, direction: Vec3) -> f32;

    fn random(&self, origin: Point3) -> Vec3;
}

pub struct Translate {
    object: Arc<dyn Hittable>,
    offset: Vec3,
    bbox: AABB,
}
impl Translate {
    pub fn new(object: Arc<dyn Hittable>, offset: Vec3) -> Self {
        let bbox = object.bounding_box() + offset;
        Self {
            object,
            offset,
            bbox,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool {
        let offset_ray = Ray::new(r.origin() - self.offset, r.direction(), r.time());

        if !self.object.hit(&offset_ray, ray_t, rec) {
            return false;
        }

        rec.p += self.offset;

        true
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }

    fn random(&self, _origin: Point3) -> Vec3 {
        Vec3::new(1., 0., 0.)
    }

    fn pdf_value(&self, _origin: Point3, _direction: Vec3) -> f32 {
        0.
    }
}

pub struct RotateY {
    object: Arc<dyn Hittable>,
    sin_theta: f32,
    cos_theta: f32,
    bbox: AABB,
}
impl RotateY {
    pub fn new(object: Arc<dyn Hittable>, angle: f32) -> Self {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = object.bounding_box().clone();

        let mut min = Point3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
        let mut max = Point3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f32 * bbox.x.max + (1 - i) as f32 * bbox.x.min;
                    let y = j as f32 * bbox.y.max + (1 - j) as f32 * bbox.y.min;
                    let z = k as f32 * bbox.z.max + (1 - k) as f32 * bbox.z.min;

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(newx, y, newz);

                    for c in 0..3 {
                        if tester.0[c] > max.0[c] {
                            max.0[c] = tester.0[c];
                        }
                        if tester.0[c] < min.0[c] {
                            min.0[c] = tester.0[c];
                        }
                    }
                }
            }
        }

        let bbox = AABB::around_points(min, max);

        RotateY {
            object,
            sin_theta,
            cos_theta,
            bbox,
        }
    }
}
impl Hittable for RotateY {
    fn hit(&self, r: &Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool {
        let mut origin = r.origin();
        let mut direction = r.direction();

        origin.0[0] = self.cos_theta * r.origin().x() - self.sin_theta * r.origin().z();
        origin.0[2] = self.sin_theta * r.origin().x() + self.cos_theta * r.origin().z();

        direction.0[0] = self.cos_theta * r.direction().x() - self.sin_theta * r.direction().z();
        direction.0[2] = self.sin_theta * r.direction().x() + self.cos_theta * r.direction().z();

        let rotated_ray = Ray::new(origin, direction, r.time());

        if !self.object.hit(&rotated_ray, ray_t, rec) {
            return false;
        }

        let mut p = rec.p;
        p.0[0] = self.cos_theta * rec.p.x() + self.sin_theta * rec.p.z();
        p.0[2] = -self.sin_theta * rec.p.x() + self.cos_theta * rec.p.z();

        let mut normal = rec.normal;
        normal.0[0] = self.cos_theta * rec.normal.x() + self.sin_theta * rec.normal.z();
        normal.0[2] = -self.sin_theta * rec.normal.x() + self.cos_theta * rec.normal.z();

        rec.p = p;
        rec.normal = normal;

        true
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }

    fn random(&self, _origin: Point3) -> Vec3 {
        Vec3::new(1., 0., 0.)
    }

    fn pdf_value(&self, _origin: Point3, _direction: Vec3) -> f32 {
        0.
    }
}
