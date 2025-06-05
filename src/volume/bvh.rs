use crate::{HitRecord, Hittable, HittableList, Interval, Ray, AABB, Point3, Vec3};

use std::sync::Arc;

pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: AABB,
}

impl BvhNode {
    pub fn from_list(mut list: HittableList) -> Self {
        Self::new_optimized(&mut list.objects)
    }

    fn new_optimized(objects: &mut [Arc<dyn Hittable>]) -> Self {
        let bbox = objects.iter().fold(AABB::empty(), |acc, obj| {
            AABB::around_boxes(&acc, obj.bounding_box())
        });
        let axis = bbox.longest_axis();
        let comparator = match axis {
            0 => Self::box_x_compare,
            1 => Self::box_y_compare,
            2 => Self::box_z_compare,
            _ => panic!("Invalid axis"),
        };

        objects.sort_by(comparator);
        // cheeky check to see if we should just return right away
        match objects.len() {
            1 => {
                let single_node = objects[0].clone();
                return Self {
                    left: single_node.clone(),
                    right: single_node,
                    bbox,
                };
            }
            2 => {
                let left_node = objects[0].clone();
                let right_node = objects[1].clone();
                return Self {
                    left: left_node,
                    right: right_node,
                    bbox,
                };
            }
            _ => (),
        };

        let mut left_area: Vec<f32> = vec![0.0; objects.len()];
        let mut right_area: Vec<f32> = vec![0.0; objects.len()];

        let boxes: Vec<AABB> = objects
            .iter()
            .map(|obj| obj.bounding_box().clone())
            .collect();

        let mut left_box = boxes.first().unwrap().to_owned();
        left_area[0] = left_box.area();
        for i in 1..boxes.len() {
            left_box = AABB::around_boxes(&left_box, &boxes[i]);
            left_area[i] = left_box.area();
        }

        let mut right_box = boxes.get(objects.len() - 1).unwrap().to_owned();
        right_area[objects.len() - 1] = right_box.area();
        for i in (1..(objects.len() - 1)).rev() {
            right_box = AABB::around_boxes(&right_box, &boxes[i]);
            right_area[i] = right_box.area();
        }

        let mut min_sah = f32::MAX;
        let mut min_sah_idx = 0;
        for i in 0..(objects.len() - 1) {
            let sah = (i as f32)
                + 1.0 * left_area[i]
                + ((objects.len() - 1 - i) as f32) * right_area[i + 1];
            if sah < min_sah {
                min_sah = sah;
                min_sah_idx = i;
            }
        }

        let left: Arc<dyn Hittable> = if min_sah_idx == 0 {
            objects[0].clone()
        } else {
            Arc::new(Self::new_optimized(&mut objects[0..min_sah_idx + 1]))
        };
        let right: Arc<dyn Hittable> = if min_sah_idx + 1 == objects.len() - 1 {
            objects[min_sah_idx + 1].clone()
        } else {
            Arc::new(Self::new_optimized(&mut objects[min_sah_idx + 1..]))
        };

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
            .total_cmp(&b_box.axis_interval(axis).min)
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
        let interval = &mut Interval::new(ray_t.min, if hit_left { rec.t } else { ray_t.max });
        let hit_right = self.right.hit(r, interval, rec);

        hit_left || hit_right
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
