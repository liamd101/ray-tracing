use crate::vec3::{Vec3, Point3};

pub struct Ray {
    orig: Point3,
    dir: Vec3,
    tm: f32,
}

impl Default for Ray {
    fn default() -> Self {
        Ray {
            orig: Point3::default(),
            dir: Vec3::default(),
            tm: 0.0,
        }
    }
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3, time: f32) -> Self {
        Ray {
            orig: origin,
            dir: direction,
            tm: time,
        }
    }

    pub fn origin(&self) -> Point3 {
        self.orig
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn time(&self) -> f32 {
        self.tm
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.orig + self.dir * t
    }
}
