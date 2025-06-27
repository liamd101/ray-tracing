use rand;

pub const PI: f64 = std::f64::consts::PI;
pub const INFINITY: f32 = f32::INFINITY;

/// Helper function for converting degrees to radians
pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI as f32 / 180.0
}

/// Generates a pseudorandom floating point number in the range [0, 1]
pub fn random_double() -> f32 {
    rand::random::<f32>()
}

/// Generates a pseudorandom number within the range [min, max]
pub fn random_double_range(min: f32, max: f32) -> f32 {
    min + (max - min) * random_double()
}

/// Linearly interpolates the input value `x` between `a` and `b`
pub fn lerp(x: f32, a: f32, b: f32) -> f32 {
    (1. - x) * a + x * b
}
