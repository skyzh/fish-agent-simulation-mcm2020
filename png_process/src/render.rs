use crate::data::TemperatureMap;
use crate::fish::Fish;
use crate::parameters::*;
use image::Rgba;

pub fn render(t_map: &TemperatureMap, fish: &Vec<Fish>) -> image::RgbaImage {
    let mut img = t_map.get_image();

    for idx in 0..fish.len() {
        let f = &fish[idx];
        for x_offset in -1..2 {
            for y_offset in -1..2 {
                let xx = f.x + x_offset;
                let yy = f.y + y_offset;
                if xx < 0 || yy < 0 || xx >= t_map.width as i64 || yy >= t_map.height as i64 {
                    continue;
                }
                let [r, g, b, a] = img.get_pixel(xx as u32, yy as u32).0;
                let mut r = r as usize + 20;
                r = r.min(255);
                img.put_pixel(xx as u32, yy as u32, Rgba([r as u8, g as u8, b as u8, 255]));
            }
        }
    }

    img
}
