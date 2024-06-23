use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    interval::Interval,
    ray::Ray,
};

use std::rc::Rc;

pub struct BvhNode {
    left: Rc<dyn Hittable>,
    right: Rc<dyn Hittable>,
    bbox: AABB,
}

impl BvhNode {
    pub fn from_list(mut list: HittableList) -> Self {
        Self::new_optimized(&mut list.objects)
    }

    fn new_optimized(objects: &mut [Rc<dyn Hittable>]) -> Self {
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
        let (left, right) = match objects.len() {
            1 => {
                let single_node = objects[0].clone();
                (single_node.clone(), single_node)
            }
            2 => {
                let left_node = objects[0].clone();
                let right_node = objects[1].clone();
                (left_node, right_node)
            }
            _ => {
                let mid = objects.len() / 2;
                let left = Rc::new(Self::new_optimized(&mut objects[..mid]));
                let right = Rc::new(Self::new_optimized(&mut objects[mid..]));
                return Self { left, right, bbox };
            }
        };

        Self { left, right, bbox }
    }

    fn box_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>, axis: usize) -> std::cmp::Ordering {
        let a_box = a.bounding_box();
        let b_box = b.bounding_box();
        a_box
            .axis_interval(axis)
            .min
            .total_cmp(&b_box.axis_interval(axis).min)
    }

    fn box_x_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> std::cmp::Ordering {
        Self::box_compare(a, b, 0)
    }

    fn box_y_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> std::cmp::Ordering {
        Self::box_compare(a, b, 1)
    }

    fn box_z_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> std::cmp::Ordering {
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
}

#[cfg(test)]
mod test_hit {
    use super::*;
    use crate::{
        material::NoneMaterial,
        sphere::Sphere,
        vec3::{Point3, Vec3},
    };

    #[test]
    fn test_hit() {
        let sphere1 = Rc::new(Sphere::stationary(
            Point3::new(0.0, 0.0, 0.0),
            1.0,
            Box::new(NoneMaterial),
        ));
        let sphere2 = Rc::new(Sphere::stationary(
            Point3::new(2.0, 0.0, 0.0),
            1.0,
            Box::new(NoneMaterial),
        ));
    }
}
