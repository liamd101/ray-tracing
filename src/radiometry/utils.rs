pub const LAMBDA_MIN: f32 = 360.;
pub const LAMBDA_MAX: f32 = 830.;

pub fn blackbody(wavelength: f32, temp: f32) -> f32 {
    if temp <= 0. {
        return 0.;
    }
    const C: f32 = 299792458.;
    const H: f32 = 6.626_069_7e-34;
    const KB: f32 = 1.3806488e-23;

    let l_nm = wavelength * 1e-9;

    (2. * H * C.powi(2))
        / (l_nm.powi(5) * (std::f32::consts::E.powf(H * C / l_nm * KB * temp) - 1.))
}

pub fn find_interval<F>(sz: usize, pred: F) -> usize
where
    F: Fn(usize) -> bool,
{
    let mut size: isize = sz as isize - 2;
    let mut first: isize = 1;
    while size > 0 {
        let half: usize = (size as usize) >> 1;
        let middle: usize = first as usize + half;
        let pred_result = pred(middle);
        first = if pred_result {
            middle as isize + 1
        } else {
            first
        };
        size = if pred_result {
            size - (half + 1) as isize
        } else {
            half as isize
        };
    }

    ((first - 1).max(0).min(sz as isize - 2)) as usize
}
