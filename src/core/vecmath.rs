use crate::{vec3, Vec3, utils};

pub fn spherical_triangle_area(a: Vec3, b: Vec3, c: Vec3) -> f32 {
    let numerator = vec3::dot(a, vec3::cross(b, c));
    let denominator = 1. + vec3::dot(a, b) + vec3::dot(b, c) + vec3::dot(a, c);
    (2. * (numerator / denominator)).tan().abs()
}

pub fn spherical_quad_area(a: Vec3, b: Vec3, c: Vec3, d: Vec3) -> f32 {
    let axb = vec3::cross(a, b);
    let bxc = vec3::cross(b, c);
    let cxd = vec3::cross(c, d);
    let dxa = vec3::cross(d, a);
    if axb.length_squared() == 0.
        || bxc.length_squared() == 0.
        || cxd.length_squared() == 0.
        || dxa.length_squared() == 0.
    {
        return 0.;
    }
    let axb = vec3::unit_vector(axb);
    let bxc = vec3::unit_vector(bxc);
    let cxd = vec3::unit_vector(cxd);
    let dxa = vec3::unit_vector(dxa);
    todo!();
}
