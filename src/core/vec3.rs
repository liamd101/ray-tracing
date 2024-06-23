use crate::utils::{random_double, random_double_range};
use std::ops;

#[derive(Clone, Copy)]
pub struct Vec3(pub [f32; 3]);
pub type Point3 = Vec3;

impl Default for Vec3 {
    fn default() -> Self {
        Vec3([0.0, 0.0, 0.0])
    }
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3([x, y, z])
    }

    pub fn x(&self) -> f32 {
        self.0[0]
    }

    pub fn y(&self) -> f32 {
        self.0[1]
    }

    pub fn z(&self) -> f32 {
        self.0[2]
    }

    pub fn length_squared(&self) -> f32 {
        self.0[0] * self.0[0] + self.0[1] * self.0[1] + self.0[2] * self.0[2]
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn random() -> Self {
        Vec3([random_double(), random_double(), random_double()])
    }

    pub fn random_range(min: f32, max: f32) -> Self {
        Vec3([
            random_double_range(min, max),
            random_double_range(min, max),
            random_double_range(min, max),
        ])
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.0[0].abs() < s && self.0[1].abs() < s && self.0[2].abs() < s
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0[0] += rhs.0[0];
        self.0[1] += rhs.0[1];
        self.0[2] += rhs.0[2];
    }
}
impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.0[0] *= rhs;
        self.0[1] *= rhs;
        self.0[2] *= rhs;
    }
}
impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        let inv = 1.0 / rhs;
        *self *= inv;
    }
}

impl ops::Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Vec3([
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
        ])
    }
}
impl ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Vec3([
            self.0[0] - rhs.0[0],
            self.0[1] - rhs.0[1],
            self.0[2] - rhs.0[2],
        ])
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Vec3([
            self.0[0] * rhs.0[0],
            self.0[1] * rhs.0[1],
            self.0[2] * rhs.0[2],
        ])
    }
}
impl ops::Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Vec3([self.0[0] * rhs, self.0[1] * rhs, self.0[2] * rhs])
    }
}
impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        rhs * self
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        let inv = 1.0 / rhs;
        self * inv
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Vec3([-self.0[0], -self.0[1], -self.0[2]])
    }
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

pub fn dot(u: Vec3, v: Vec3) -> f32 {
    u.0[0] * v.0[0] + u.0[1] * v.0[1] + u.0[2] * v.0[2]
}

pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3([
        u.0[1] * v.0[2] - u.0[2] * v.0[1],
        u.0[2] * v.0[0] - u.0[0] * v.0[2],
        u.0[0] * v.0[1] - u.0[1] * v.0[0],
    ])
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random_range(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_unit_vector() -> Vec3 {
    unit_vector(random_in_unit_sphere())
}

pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if dot(in_unit_sphere, normal) > 0.0 {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n * 2.0 * dot(v, n)
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta = dot(-uv, n).min(1.0);
    let r_out_perp = etai_over_etat * (uv + n * cos_theta);
    let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * n;
    r_out_perp + r_out_parallel
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(
            random_double_range(-1.0, 1.0),
            random_double_range(-1.0, 1.0),
            0.0,
        );
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}
