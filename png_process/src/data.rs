use std::{fs, io};
use image::{GrayImage, GenericImageView, RgbImage, RgbaImage, Rgb, Rgba};
use crate::parameters::*;
use crate::predict::predict_temperature_map;

pub struct TemperatureMap {
    pub temperature: Vec<Option<f64>>,
    pub width: u32,
    pub height: u32,
    pub year: i32,
    pub month: i32,
    pub path: String,
}

#[inline(always)]
pub fn u8_to_temperature(result: u8) -> f64 {
    result as f64 / 255.0 * (35.0 + 2.0) - 2.0
}

#[inline(always)]
pub fn temperature_to_u8(result: f64) -> u8 { ((result + 2.0) / (35.0 + 2.0) * 255.0) as u8 }

impl TemperatureMap {
    pub fn is_ocean(&self, x: i64, y: i64) -> bool {
        let t = &self.temperature[y as usize * self.width as usize + x as usize];
        return t.is_some();
    }
    pub fn pos_of(&self, x: i64, y: i64) -> usize {
        y as usize * self.width as usize + x as usize
    }
    pub fn get_temperature(&self, x: i64, y: i64) -> f64 {
        return self.temperature[self.pos_of(x, y)].unwrap();
    }
    pub fn in_range(&self, x: i64, y: i64) -> bool {
        if x < 0 || y < 0 {
            return false;
        }
        if x as u32 >= self.width || y as u32 >= self.height {
            return false;
        }
        return true;
    }
    pub fn get_image(&self) -> image::RgbaImage {
        use palette::{LinSrgb, Hsv, Srgb, Gradient};

        let grad = Gradient::new(vec![
            LinSrgb::new(30.0 / 255.0, 144.0 / 255.0, 255.0 / 255.0),
            LinSrgb::new(255.0 / 255.0, 165.0 / 255.0, 0.0 / 255.0)
        ]);

        let mut image: RgbaImage = RgbaImage::new(self.width, self.height);
        for (x, y, pixel) in image.enumerate_pixels_mut() {
            *pixel = Rgba(match &self.temperature[self.pos_of(x as i64, y as i64)] {
                Some(t) => {
                    let tt = temperature_to_u8(*t) as f64 / 256.0;
                    let tt = grad.get(tt).into_format();
                    [tt.red, tt.green, tt.blue, 255]
                }
                None => [255, 255, 255, 0]
            })
        }
        image
    }
    pub fn generate_image(&self, path: &str) {
        self.get_image().save(path).unwrap();
    }
}

fn process_background() {
    let mut image = image::open(BACKGROUND_IMG_ORIG).unwrap();
    let image = image.crop(CROP_X, CROP_Y, CROP_W, CROP_H);
    image.save(BACKGROUND_IMG);
}

pub fn load_data() -> Vec<TemperatureMap> {
    process_background();
    let mut entries = fs::read_dir("../NASAM").unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>().unwrap();
    entries.sort();
    let mut result = vec![];
    let mut csv_str: String = String::new();
    for (idx, path) in entries.iter().enumerate() {
        let path = path.as_path();
        let path_str: String = path.to_str().unwrap().to_owned().to_string();
        if !path_str.ends_with(".PNG") {
            continue;
        }
        let mut image = image::open(path).unwrap();
        let image = image.crop(CROP_X, CROP_Y, CROP_W, CROP_H);

        // let image = image.resize(image.width() / 2, image.height() / 2, image::imageops::Nearest);

        let (year, month) = parse_date_m(&path_str);

        let crop_path = format!("out/{}-{}.png", year, month);
        if SAVE_CROP_IMAGE {
            image.save(&crop_path).unwrap();
        }

        if OUTPUT_SCOTLAND_TEMPERATURE {
            let color = image.as_luma8().unwrap().get_pixel(SCOTLAND_CENTER_X, SCOTLAND_CENTER_Y).0[0];
            if color != 255 {
                csv_str += format!("{}, {}, {}-{}, {}\n", year, month, year, month, u8_to_temperature(color)).as_ref();
            } else {
                csv_str += format!("{}, {}, {}-{}, NaN\n", year, month, year, month).as_ref();
            }
        }

        let grey = image.as_luma8().unwrap();

        let mut temperature: Vec<Option<f64>> = vec![];
        for (x, y, pixel) in grey.enumerate_pixels() {
            let result = pixel.0[0];
            if result != 255 {
                temperature.push(Some(u8_to_temperature(result)));
            } else {
                temperature.push(None);
            }
        }
        if year > DATA_YEAR {
            break;
        }
        result.push(TemperatureMap {
            temperature,
            width: image.width(),
            height: image.height(),
            year,
            month,
            path: crop_path,
        });
        println!("({}/{}) {}-{} imported, map size {}*{}", idx, entries.len(), year, month, image.width(), image.height());
    }
    println!("{} data imported", result.len());
    let predict_begin = result.len();
    for i in 0..PREDICT_MONTH {
        let m = predict_temperature_map(&result, PREDICT_LOOK_BACKWARD_YEAR, predict_begin);
        println!("({}/{}) {}-{} predicted", i, PREDICT_MONTH, m.year, m.month);
        if SAVE_PREDICT_IMAGE {
            m.generate_image(format!("out/predict_{}-{}.png", m.year, m.month).as_str());
        }
        result.push(m);
    }
    if OUTPUT_SCOTLAND_TEMPERATURE {
        println!("\n\n{}\n\n", csv_str);
    }
    result
}


fn parse_date_x(path: &String) -> (i32, i32, i32) {
    (path[15..19].to_owned().parse::<i32>().unwrap(),
     path[20..22].to_owned().parse::<i32>().unwrap(),
     path[23..25].to_owned().parse::<i32>().unwrap()
    )
}

fn parse_date_m(path: &String) -> (i32, i32) {
    (path[16..20].to_owned().parse::<i32>().unwrap(),
     path[21..23].to_owned().parse::<i32>().unwrap()
    )
}

