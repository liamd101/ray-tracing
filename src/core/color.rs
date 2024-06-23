use crate::vec3::Vec3;
use crate::interval::Interval;

pub type Color = Vec3;

fn linear_to_gamma(x: f32) -> f32 {
    if x > 0.0 {
        x.sqrt()
    } else {
        0.0
    }
}

pub fn write_color(out: &mut dyn std::io::Write, color: Color) {
    let r = color.x();
    let g = color.y();
    let b = color.z();
    let r = linear_to_gamma(r);
    let g = linear_to_gamma(g);
    let b = linear_to_gamma(b);

    let intensity = Interval::new(0.0, 0.999);
    let rbyte = (256.0 * intensity.clamp(r)) as u32;
    let gbyte = (256.0 * intensity.clamp(g)) as u32;
    let bbyte = (256.0 * intensity.clamp(b)) as u32;

    writeln!(out, "{} {} {}", rbyte, gbyte, bbyte).unwrap();
}
