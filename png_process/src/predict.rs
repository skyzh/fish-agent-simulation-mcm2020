use crate::data::TemperatureMap;
use crate::parameters::*;

pub fn predict_temperature_map(t_maps: &Vec<TemperatureMap>, mean_year: usize, predict_start: usize) -> TemperatureMap {
    let mut tt: Vec<Option<f64>>;
    assert!(mean_year >= 1);
    let idx = t_maps.len() - 12;
    let prev_yr = &t_maps[idx];
    tt = prev_yr.temperature.clone();
    for i in 2..mean_year + 1 {
        let idx = t_maps.len() - i * 12;
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
    TemperatureMap {
        path: String::new(),
        width: t_maps.last().unwrap().width,
        height: t_maps.last().unwrap().height,
        temperature: tt.into_iter().map(|x| match x {
            Some(x) => Some(x / mean_year as f64 + 0.001 * (t_maps.len() - predict_start) as f64),
            None => None
        }).collect(),
        year: prev_yr.year + 1,
        month: prev_yr.month,
    }
}
