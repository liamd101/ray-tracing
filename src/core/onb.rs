use crate::{vec3, Vec3};

pub struct ONB {
    axis: [Vec3; 3],
}

impl ONB {
    pub fn new(n: Vec3) -> Self {
        let mut axis: [Vec3; 3] = [Vec3::default(); 3];

        axis[2] = vec3::unit_vector(n);
        let a: Vec3 = if axis[2].x().abs() > 0.9 {
            Vec3::new(0., 1., 0.)
        } else {
            Vec3::new(1., 0., 0.)
        };
        axis[1] = vec3::unit_vector(vec3::cross(axis[2], a));
        axis[0] = vec3::cross(axis[2], axis[1]);

        Self { axis }
    }

    pub fn u(&self) -> Vec3 {
        self.axis[0]
    }
    pub fn v(&self) -> Vec3 {
        self.axis[1]
    }
    pub fn w(&self) -> Vec3 {
        self.axis[2]
    }

    pub fn transform(&self, v: Vec3) -> Vec3 {
        (v.x() * self.axis[0]) + (v.y() * self.axis[1]) + (v.z() * self.axis[2])
    }
}
