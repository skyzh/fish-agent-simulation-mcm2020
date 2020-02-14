use std::{fs, io};
use image::{GrayImage, GenericImageView};
use image::imageops::filter3x3;

pub struct TemperatureMap {
    pub temperature: Vec<Option<f64>>,
    pub width: u32,
    pub height: u32,
    pub year: i32,
    pub month: i32,
    pub path: String
}


impl TemperatureMap {
    pub fn is_ocean(&self, x: i64, y: i64) -> bool {
        let t = &self.temperature[y as usize * self.width as usize + x as usize];
        return t.is_some();
    }
    pub fn get_temperature(&self, x: i64, y: i64) -> f64 {
        return self.temperature[y as usize * self.width as usize + x as usize].unwrap();
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
}

pub fn loadData() -> Vec<TemperatureMap> {
    let mut entries = fs::read_dir("../NASAM").unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>().unwrap();
    entries.sort();
    let mut result = vec![];
    for path in entries {
        let path = path.as_path();
        let path_str: String = path.to_str().unwrap().to_owned().to_string();
        if !path_str.ends_with(".PNG") {
            continue;
        }
        let mut image = image::open(path).unwrap();
        let image = image.crop(1185, 60, 1130, 540);
        // let image = image.resize(image.width() / 2, image.height() / 2, image::imageops::Nearest);

        let (year, month) = parse_date_m(&path_str);
        let crop_path = format!("out/{}-{}.png", year, month);
        image.save(&crop_path).unwrap();

        let grey = image.as_luma8().unwrap();

        let mut temperature: Vec<Option<f64>> = vec![];
        for (x, y, pixel) in grey.enumerate_pixels() {
            let result = pixel.0[0];
            if result != 255 {
                temperature.push(Some(result as f64 / 255.0 * (35.0 + 2.0) - 2.0));
            } else {
                temperature.push(None);
            }
        }
        if year > 2007 {
            break;
        }
        result.push(TemperatureMap {
            temperature,
            width: image.width(),
            height: image.height(),
            year,
            month,
            path: crop_path
        });
        println!("{}-{} imported, map size {}*{}", year, month, image.width(), image.height());
    }
    println!("{} data imported", result.len());
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

