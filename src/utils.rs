use rand;

pub const PI: f64 = 3.1415926535897932385;
pub const INFINITY: f32 = f32::INFINITY;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI as f32 / 180.0
}

pub fn random_double() -> f32 {
    rand::random::<f32>()
}

pub fn random_double_range(min: f32, max: f32) -> f32 {
    min + (max - min) * random_double()
}
