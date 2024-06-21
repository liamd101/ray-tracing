use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    interval::Interval,
    ray::Ray,
};

use std::sync::Arc;

pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: AABB,
}

impl BvhNode {
    pub fn from_list(mut list: HittableList) -> Self {
        let len = list.objects.len();
        Self::new(&mut list.objects, 0, len)
    }

    fn new(mut objects: &mut [Arc<dyn Hittable>], start: usize, end: usize) -> Self {
        let axis = rand::random::<usize>() % 3;
        let comparator = match axis {
            0 => Self::box_x_compare,
            1 => Self::box_y_compare,
            _ => Self::box_z_compare,
        };
        let object_span = end - start;

        if object_span != 1 && object_span != 2 {
            objects[start..end].sort_by(comparator);
            let mid = start + object_span / 2;
            let left = Arc::new(Self::new(&mut objects, start, mid));
            let right = Arc::new(Self::new(&mut objects, mid, end));
            let bbox = AABB::around_boxes(&left.bounding_box(), &right.bounding_box());
            return Self { left, right, bbox };
        }

        let (&left, &right) = match object_span {
            1 => {
                let single_node = &objects[start];
                (single_node, single_node)
            }
            2 => {
                let left_node = &objects[start];
                let right_node = &objects[start + 1];
                (left_node, right_node)
            }
            _ => {
                panic!("Should have been handled already")
            }
        };
        let bbox = AABB::around_boxes(&left.bounding_box(), &right.bounding_box());

        Self { left, right, bbox }
    }

    fn box_compare(
        a: &Arc<dyn Hittable>,
        b: &Arc<dyn Hittable>,
        axis: usize,
    ) -> std::cmp::Ordering {
        let a_box = a.bounding_box();
        let b_box = b.bounding_box();
        a_box
            .axis_interval(axis)
            .min
            .partial_cmp(&b_box.axis_interval(axis).min)
            .unwrap()
    }

    fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> std::cmp::Ordering {
        Self::box_compare(a, b, 0)
    }

    fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> std::cmp::Ordering {
        Self::box_compare(a, b, 1)
    }

    fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> std::cmp::Ordering {
        Self::box_compare(a, b, 2)
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool {
        if !self.bbox.hit(r, ray_t) {
            return false;
        }

        let hit_left = self.left.hit(r, ray_t, rec);
        let hit_right = self.right.hit(
            r,
            &mut Interval::new(ray_t.min, if hit_left { rec.t } else { ray_t.max }),
            rec,
        );

        hit_left || hit_right
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}
