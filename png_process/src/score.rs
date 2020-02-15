use crate::fish::*;
use crate::parameters::*;
use crate::data::*;
use rand::prelude::*;

pub struct Living<'a> {
    pub t_map: &'a TemperatureMap,
    food_score: Vec<f64>,
    width: u32,
    height: u32,
}

pub struct LandScore {
    pub land_score: Vec<f64>,
}

impl LandScore {
    pub fn from_map(t_map: &TemperatureMap) -> Self {
        let mut land_score: Vec<f64> = vec![];
        land_score.resize(t_map.width as usize * t_map.height as usize, 0.0);

        // println!(" > land score");
        // land score
        for x in 0..t_map.width as i64 {
            for y in 0..t_map.height as i64 {
                if !t_map.is_ocean(x, y) {
                    for x_offset in -LAND_SCORE_SPREAD_RANGE..LAND_SCORE_SPREAD_RANGE + 1 {
                        for y_offset in -LAND_SCORE_SPREAD_RANGE..LAND_SCORE_SPREAD_RANGE + 1 {
                            let n_x = x + x_offset;
                            let n_y = y + y_offset;
                            if !t_map.in_range(n_x, n_y) {
                                continue;
                            }
                            if !t_map.is_ocean(n_x, n_y) {
                                continue;
                            }
                            let score = (x_offset * x_offset + y_offset * y_offset) as f64
                                / LAND_SCORE_SPREAD_RANGE as f64
                                / LAND_SCORE_SPREAD_RANGE as f64;
                            if score < 0.0 {
                                continue;
                            }
                            let pos = n_x + n_y * t_map.width as i64;
                            land_score[pos as usize] = score.max(land_score[pos as usize]);
                        }
                    }
                }
            }
        }

        Self { land_score }
    }
}

impl<'a> Living<'a> {
    pub fn from_map(t_map: &'a TemperatureMap, species: &'a Vec<Fish>) -> Self {
        let mut living = Self {
            t_map,
            width: t_map.width,
            height: t_map.height,
            food_score: vec![],
        };

        // println!(" > food score");
        living.food_score.clear();
        living.food_score.resize(living.t_map.width as usize * living.t_map.height as usize, 0.0);
        // food score
        for f in species.iter() {
            for x_offset in -FOOD_SCORE_SPREAD_RANGE..FOOD_SCORE_SPREAD_RANGE + 1 {
                for y_offset in -FOOD_SCORE_SPREAD_RANGE..FOOD_SCORE_SPREAD_RANGE + 1 {
                    let n_x = f.x + x_offset;
                    let n_y = f.y + y_offset;
                    if !living.t_map.in_range(n_x, n_y) {
                        continue;
                    }
                    let score = (x_offset * x_offset + y_offset * y_offset) as f64
                        / FOOD_SCORE_SPREAD_RANGE as f64
                        / FOOD_SCORE_SPREAD_RANGE as f64 - 1.0;
                    if score > 0.0 {
                        continue;
                    }
                    let pos = living.pos_at(n_x, n_y);
                    living.food_score[pos] += score;
                }
            }
        }

        living
    }

    const fn pos_at(&self, x: i64, y: i64) -> usize {
        (self.width as i64 * y + x) as usize
    }

    pub fn score(&self, rng: &mut SmallRng, x: i64, y: i64, optimal_temperature: f64, age: usize, land: &LandScore) -> f64 {
        let normal_dist: f64 = rng.sample(rand_distr::StandardNormal);
        normal_dist * NORMAL_K
            + land.land_score[self.pos_at(x, y)] * LAND_SCORE_K
            + -(self.t_map.get_temperature(x, y) - optimal_temperature).abs() * TEMP_SCORE_K
            - (-((FISH_MAX_AGE - age) as f64) * AGE_SCORE_K).exp()
            + self.food_score[self.pos_at(x, y)] * FOOD_SCORE_K
    }

    pub fn debug_score(&self, rng: &mut SmallRng, x: i64, y: i64, optimal_temperature: f64, age: usize, land: &LandScore) {
        let land_score = land.land_score[self.pos_at(x, y)];
        let land_score_normal = land_score * LAND_SCORE_K;
        let temp = -(self.t_map.get_temperature(x, y) - optimal_temperature).abs();
        let temp_normal = temp * TEMP_SCORE_K;
        let age = -(-((FISH_MAX_AGE - age) as f64) * AGE_SCORE_K).exp();
        let food_score = self.food_score[self.pos_at(x, y)];
        let food_score_normal = food_score * FOOD_SCORE_K;
        println!("land {} -> {} | temp {} -> {} | age {} | food {} -> {}",
                 land_score, land_score_normal,
                 temp, temp_normal, age, food_score, food_score_normal);
    }

    /*
    pub fn generate_image(&self, path_prefix: &String) {
        let mut image_land: image::RgbImage = image::RgbImage::new(self.width, self.height);
        copy_image(&self.t_map.path, &mut image_land);

        let min = min_of(&self.land_score);
        let max = max_of(&self.land_score);
        for (x, y, pixel) in image_land.enumerate_pixels_mut() {
            let [mut r, mut g, mut b] = pixel.0;
            let score = self.land_score[self.pos_at(x as i64, y as i64)];
            let score = (score - min) / (max - min);
            let mut x = (score * 255.0).floor();
            b = x as u8;
            *pixel = Rgb([r, g, b]);
        }
        println!("land score {}~{}", min, max);
        image_land.save(format!("{}_land.png", path_prefix));

        let mut image_food = image::RgbImage::new(self.width, self.height);
        copy_image(&self.t_map.path, &mut image_food);

        let min = min_of(&self.food_score);
        let max = max_of(&self.food_score);
        for (x, y, pixel) in image_food.enumerate_pixels_mut() {
            let [mut r, mut g, mut b] = pixel.0;
            let score = self.food_score[self.pos_at(x as i64, y as i64)];
            let score = (score - min) / (max - min);
            let mut x = (score * 255.0).floor();
            b = x as u8;
            *pixel = Rgb([r, g, b]);
        }
        println!("food score {}~{}", min, max);
        image_food.save(format!("{}_food.png", path_prefix));
    }
    */
}
