use palette::gradient::Gradient;
use palette::rgb::LinSrgb;

pub fn max_of(x: &Vec<f64>) -> f64 {
    let mut r = std::f64::MIN;
    for i in x {
        if *i > r { r = *i; }
    }
    return r;
}

pub fn min_of(x: &Vec<f64>) -> f64 {
    let mut r = std::f64::MAX;
    for i in x {
        if *i < r { r = *i; }
    }
    return r;
}

pub fn get_gradient() -> Gradient<LinSrgb> {
    /*
    return Gradient::new(vec![
        LinSrgb::new(30.0 / 255.0, 144.0 / 255.0, 255.0 / 255.0),
        LinSrgb::new(255.0 / 255.0, 165.0 / 255.0, 0.0 / 255.0)
    ]);*/
    return Gradient::new(vec![
        LinSrgb::new(74.0 / 255.0, 151.0 / 255.0, 231.0 / 255.0),
        LinSrgb::new(196.0 / 255.0, 237.0 / 255.0, 253.0 / 255.0)
    ]);
}

#[inline]
pub const fn xy_to_long_lat(x: u32, y: u32, base: (u32, u32)) -> (f64, f64) {
    let n_x = x + base.0;
    let n_y = y + base.1;
    (-179.95 + 0.1 * n_x as f64, 89.95 - 0.1 * n_y as f64)
}
