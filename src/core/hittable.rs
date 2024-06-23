use crate::{vec3, Interval, Material, NoneMaterial, Point3, Ray, Vec3, AABB};

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub mat: Box<dyn Material>,
}

impl Default for HitRecord {
    fn default() -> Self {
        HitRecord {
            p: Default::default(),
            normal: Default::default(),
            t: 0.0,
            front_face: false,
            mat: Box::new(NoneMaterial),
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

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool;

    fn bounding_box(&self) -> &AABB;
}
