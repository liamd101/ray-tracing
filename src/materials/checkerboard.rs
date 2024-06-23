use crate::{Color, Point3, Texture};
use std::rc::Rc;

pub struct Checkerboard {
    even: Rc<dyn Texture>,
    odd: Rc<dyn Texture>,
    inv_scale: f32,
}

impl Checkerboard {
    pub fn new(even: Rc<dyn Texture>, odd: Rc<dyn Texture>, scale: f32) -> Self {
        Checkerboard {
            odd,
            even,
            inv_scale: 1.0 / scale,
        }
    }
}

impl Texture for Checkerboard {
    fn value(&self, u: f32, v: f32, p: &Point3) -> Color {
        let x_int = (self.inv_scale * p.x()).floor() as i32;
        let y_int = (self.inv_scale * p.y()).floor() as i32;
        let z_int = (self.inv_scale * p.z()).floor() as i32;
        let is_even = (x_int + y_int + z_int) % 2 == 0;
        if is_even {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}
