use crate::{
    Interval, NoneMaterial, Ray, AABB, {HitRecord, Hittable},
};
use std::rc::Rc;

pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
    bbox: AABB,
}

impl Default for HittableList {
    fn default() -> Self {
        Self::new()
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord {
            p: Default::default(),
            normal: Default::default(),
            mat: Box::new(NoneMaterial),
            t: 0.0,
            front_face: false,
        };
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in self.objects.iter() {
            if object.hit(
                r,
                &mut Interval::new(ray_t.min, closest_so_far),
                &mut temp_rec,
            ) {
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

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.bbox = AABB::around_boxes(&self.bbox, object.bounding_box());
        self.objects.push(object);
    }
}
