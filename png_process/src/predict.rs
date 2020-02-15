use crate::data::TemperatureMap;
use crate::parameters::*;

pub fn predict_temperature_map(t_maps: &Vec<TemperatureMap>, mean_year: usize, predict_start: usize) -> TemperatureMap {
    let mut tt: Vec<Option<f64>>;
    assert!(mean_year >= 1);
    let idx = t_maps.len() - 12 * PREDICT_LOOK_BACKWARD_STRIDE;
    let prev_yr = &t_maps[idx];
    tt = prev_yr.temperature.clone();
    for i in 2..mean_year + 1 {
        let idx = t_maps.len() - i * 12 * PREDICT_LOOK_BACKWARD_STRIDE;
        let tt_b = &t_maps[idx].temperature;
        for (idx, d) in tt.iter_mut().enumerate() {
            *d = match d {
                Some(t) => match tt_b[idx] {
                    Some(t_b) => Some(*t + t_b),
                    None => None
                },
                None => None
            };
        }
    }

    let pyr = t_maps.last().unwrap();
    let mut year = pyr.year;
    let mut month = pyr.month;
    month += 1;
    if month > 12 {
        year += 1;
        month = 1;
    }

    TemperatureMap {
        path: String::new(),
        width: t_maps.last().unwrap().width,
        height: t_maps.last().unwrap().height,
        temperature: tt.into_iter().map(|x| match x {
            Some(x) => Some((x / mean_year as f64) + (0.01 * ((t_maps.len() - predict_start) / 12) as f64)),
            None => None
        }).collect(),
        year,
        month
    }
}
