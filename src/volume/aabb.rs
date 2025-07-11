use crate::{interval::Interval, ray::Ray, vec3::Point3};

use std::ops;

#[derive(Clone, Debug, Default)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AABB {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        AABB { x, y, z }.pad_to_minimum()
    }

    fn pad_to_minimum(mut self) -> Self {
        let delta = 0.0001;
        if self.x.size() <= delta {
            self.x.expand(delta);
        }
        if self.y.size() <= delta {
            self.y.expand(delta);
        }
        if self.z.size() <= delta {
            self.z.expand(delta);
        }
        self
    }

    pub fn empty() -> Self {
        Self::default()
    }

    pub fn around_points(a: Point3, b: Point3) -> Self {
        let x = if a.x() <= b.x() {
            Interval::new(a.x(), b.x())
        } else {
            Interval::new(b.x(), a.x())
        };
        let y = if a.y() <= b.y() {
            Interval::new(a.y(), b.y())
        } else {
            Interval::new(b.y(), a.y())
        };
        let z = if a.z() <= b.z() {
            Interval::new(a.z(), b.z())
        } else {
            Interval::new(b.z(), a.z())
        };
        AABB::new(x, y, z).pad_to_minimum()
    }

    pub fn around_boxes(a: &AABB, b: &AABB) -> Self {
        let x = Interval::around_intervals(&a.x, &b.x);
        let y = Interval::around_intervals(&a.y, &b.y);
        let z = Interval::around_intervals(&a.z, &b.z);
        AABB::new(x, y, z).pad_to_minimum()
    }

    pub fn axis_interval(&self, axis: usize) -> &Interval {
        match axis {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Invalid axis"),
        }
    }

    pub fn longest_axis(&self) -> usize {
        let x_size = self.x.size();
        let y_size = self.y.size();
        let z_size = self.z.size();
        if x_size > y_size {
            if x_size > z_size {
                0
            } else {
                2
            }
        } else if y_size > z_size {
            1
        } else {
            2
        }
    }

    pub fn hit(&self, r: &Ray, ray_t: &mut Interval) -> bool {
        let ray_orig = r.origin();
        let ray_dir = r.direction();

        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let adinv = 1.0 / ray_dir.0[axis];
            let t0 = (ax.min - ray_orig.0[axis]) * adinv;
            let t1 = (ax.max - ray_orig.0[axis]) * adinv;
            let (t0, t1) = if adinv < 0.0 { (t1, t0) } else { (t0, t1) };
            let tmin = if t0 > ray_t.min { t0 } else { ray_t.min };
            let tmax = if t1 < ray_t.max { t1 } else { ray_t.max };

            if tmax <= tmin {
                return false;
            }
        }

        true
    }

    pub fn area(&self) -> f32 {
        self.x.size() * self.y.size() * self.z.size()
    }
}

impl ops::Add<Point3> for &AABB {
    type Output = AABB;

    fn add(self, offset: Point3) -> Self::Output {
        AABB::new(
            self.x.clone() + offset.x(),
            self.y.clone() + offset.y(),
            self.z.clone() + offset.z(),
        )
    }
}
