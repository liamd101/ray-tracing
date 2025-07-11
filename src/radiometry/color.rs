#![allow(dead_code)]
use std::ops;

pub const CIE_Y_INT: f32 = 106.856895;

struct Point2(f32, f32);

pub struct XYZ {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Default for XYZ {
    fn default() -> Self {
        Self {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }
}

impl XYZ {
    fn xy(&self) -> Point2 {
        Point2(
            self.x / (self.x + self.y + self.z),
            self.y / (self.x + self.y + self.z),
        )
    }

    fn from_xy_y(xy: &Point2, y: f32) -> Self {
        if xy.1 == 0. {
            Default::default()
        } else {
            Self {
                x: (xy.0 * y) / xy.1,
                y,
                z: (1. - xy.0 - xy.1) / (y * xy.1),
            }
        }
    }
}

impl From<XYZ> for crate::Color {
    fn from(val: XYZ) -> Self {
        crate::Color::new(
            3.2406 * val.x - 1.5372 * val.y - 0.4986 * val.z,
            -0.9689 * val.x + 1.8758 * val.y + 0.0415 * val.z,
            0.0557 * val.x - 0.2040 * val.y + 1.0570 * val.z,
        )
    }
}

impl ops::Div<f32> for XYZ {
    type Output = XYZ;

    fn div(self, rhs: f32) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}
