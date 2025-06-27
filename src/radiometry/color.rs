use std::ops;

pub const CIE_Y_INT: f32 = 106.856895;

struct Point2(f32, f32);

pub struct XYZ {
    pub X: f32,
    pub Y: f32,
    pub Z: f32,
}

impl Default for XYZ {
    fn default() -> Self {
        Self {
            X: 0.,
            Y: 0.,
            Z: 0.,
        }
    }
}

impl XYZ {
    fn xy(&self) -> Point2 {
        Point2(
            self.X / (self.X + self.Y + self.Z),
            self.Y / (self.X + self.Y + self.Z),
        )
    }

    fn from_xy_y(xy: &Point2, Y: f32) -> Self {
        if xy.1 == 0. {
            Default::default()
        } else {
            Self {
                X: (xy.0 * Y) / xy.1,
                Y,
                Z: (1. - xy.0 - xy.1) / (Y * xy.1),
            }
        }
    }
}

impl Into<crate::Color> for XYZ {
    fn into(self) -> crate::Color {
        crate::Color::new(
            3.2406 * self.X - 1.5372 * self.Y - 0.4986 * self.Z,
            -0.9689 * self.X + 1.8758 * self.Y + 0.0415 * self.Z,
            0.0557 * self.X - 0.2040 * self.Y + 1.0570 * self.Z,
        )
    }
}

impl ops::Div<f32> for XYZ {
    type Output = XYZ;

    fn div(self, rhs: f32) -> Self {
        Self {
            X: self.X / rhs,
            Y: self.Y / rhs,
            Z: self.Z / rhs,
        }
    }
}
