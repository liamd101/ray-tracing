use crate::{HitRecord, Hittable, Interval, Point3, Ray, Vec3, AABB};
use std::sync::Arc;
use rand::Rng;

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
    bbox: AABB,
}

impl Default for HittableList {
    fn default() -> Self {
        Self::new()
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool {
        let temp_rec: &mut HitRecord = &mut Default::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in self.objects.iter() {
            if object.hit(r, &mut Interval::new(ray_t.min, closest_so_far), temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }

    fn random(&self, origin: Point3) -> Vec3 {
        self.objects[rand::thread_rng().gen_range(0..self.objects.len())].random(origin)
    }

    fn pdf_value(&self, origin: Point3, direction: Vec3) -> f32 {
        let weight = 1. / self.objects.len() as f32;
        let mut sum = 0.;
        for obj in &self.objects {
            sum += weight * obj.pdf_value(origin, direction);
        }
        sum
    }
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            bbox: AABB::default(),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.bbox = AABB::around_boxes(&self.bbox, object.bounding_box());
        self.objects.push(object);
    }
}
