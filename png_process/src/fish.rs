use rand::prelude::*;
use crate::data::TemperatureMap;
use std::ops::Deref;
use image::Rgb;
use crate::parameters::*;

pub struct Fish {
    pub x: i64,
    pub y: i64,
    pub age: usize,
    pub optimal_temperature: f64,
    pub alive: bool,
}

fn max_of(x: &Vec<f64>) -> f64 {
    let mut r = std::f64::MIN;
    for i in x {
        if *i > r { r = *i; }
    }
    return r;
}

fn min_of(x: &Vec<f64>) -> f64 {
    let mut r = std::f64::MAX;
    for i in x {
        if *i < r { r = *i; }
    }
    return r;
}

pub struct Living<'a> {
    pub t_map: &'a TemperatureMap,
    food_score: Vec<f64>,
    width: u32,
    height: u32,
}

pub struct LandScore {
    pub land_score: Vec<f64>,
}

fn copy_image(path: &str, image: &mut image::RgbImage) {
    let image_original = image::open(path).unwrap();
    let image_original = image_original.as_luma8().unwrap();

    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let luma_color = image_original.get_pixel(x, y).0[0];
        *pixel = Rgb([luma_color, luma_color, luma_color]);
    }
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

pub fn one_epoch(map: &Vec<TemperatureMap>) {
    let mut thread_rng = rand::thread_rng();
    let mut rng = rand::rngs::SmallRng::from_rng(thread_rng).unwrap();
    let mut fish: Vec<Fish> = vec![];
    println!("spawning fish...");
    let initial_map = &map[0];
    let land_score = LandScore::from_map(initial_map);
    {
        let _fish = vec![];
        let living = Living::from_map(initial_map, &_fish);
        for i in 0..FISH_SPAWN_INITIAL {
            let x = rng.gen_range(0, initial_map.width - 1) as i64;
            let y = rng.gen_range(0, initial_map.height - 1) as i64;
            let age = rng.gen_range(0, FISH_MAX_AGE);
            let optimal_temperature = rng.gen_range(9.0, 10.0);

            if initial_map.is_ocean(x, y) {
                if living.score(&mut rng, x, y, optimal_temperature, age, &land_score) >= SCORE_THRESHOLD {
                    fish.push(Fish {
                        x,
                        y,
                        age,
                        optimal_temperature,
                        alive: true,
                    })
                }
            }
        }
    }

    println!("{} fish spawned", fish.len());
    let mut id = 0;
    for (map_idx, t_map) in map.iter().enumerate() {
        let begin_time = std::time::SystemTime::now();
        print!("({:03}/{:03}) processing {:04}-{:02}", map_idx, map.len(), t_map.year, t_map.month);
        // [0] Calculate living index
        // println!("> generating living index");
        let living = Living::from_map(t_map, &fish);
        // living.generate_image(&format!("result/living_{}-{}_", t_map.year, t_map.month));

        // [1] Fish move to optimal place

        let optimal_places: Vec<Option<(f64, i64, i64)>> = fish.iter().map(|f| {
            if t_map.is_ocean(f.x, f.y) {
                let current_score = living.score(&mut rng, f.x, f.y, f.optimal_temperature, f.age, &land_score);
                if current_score < SCORE_THRESHOLD {
                    return None;
                }
            }

            let mut optimal_place: Option<(f64, i64, i64)> = None;

            // [1.1] Searching within FISH_MAX_MOVE radius the fish, select random points to improve performance
            for _random_points_idx in 0..((FISH_MAX_MOVE as f64).powf(1.5)) as usize {
                let mut x_offset;
                let mut y_offset;
                loop {
                    x_offset = rng.gen_range(-FISH_MAX_MOVE, FISH_MAX_MOVE);
                    y_offset = rng.gen_range(-FISH_MAX_MOVE, FISH_MAX_MOVE);
                    if x_offset * x_offset + y_offset * y_offset <= FISH_MAX_MOVE * FISH_MAX_MOVE {
                        if x_offset * x_offset + y_offset * y_offset >= FISH_MIN_MOVE * FISH_MIN_MOVE {
                            break;
                        }
                    }
                }
                let next_x = f.x as i64 + x_offset;
                let next_y = f.y as i64 + y_offset;
                if !t_map.in_range(next_x, next_y) {
                    continue;
                }
                if !t_map.is_ocean(next_x, next_y) {
                    continue;
                }

                let here_score = living.score(&mut rng, next_x, next_y, f.optimal_temperature, f.age, &land_score);
                if optimal_place.is_some() {
                    let score = optimal_place.unwrap().0;
                    if here_score > score {
                        optimal_place = Some((here_score, next_x, next_y));
                    }
                } else {
                    optimal_place = Some((here_score, next_x, next_y));
                }
            }
            optimal_place
        }).collect();

        for idx in 0..fish.len() {
            let f = &mut fish[idx];
            let op = &optimal_places[idx];
            if op.is_some() {
                let place = op.unwrap();
                if place.0 < SCORE_THRESHOLD {
                    // Score too low, die
                    f.alive = false;
                } else {
                    f.x = place.1;
                    f.y = place.2;
                }
            } else {
                // No optimal place found, kill the fish
                f.alive = false;
            }
            f.age += 1;
        }
        // println!("> fish moved");

        // [2] Spawn new fish
        // println!("> spawning new fish");
        let mut x = 0;

        for idx in 0..fish.len() {
            let mut new_fish: Option<Fish> = None;
            {
                let f = &fish[idx];
                if !f.alive {
                    continue;
                }
                if f.age >= OPTIMAL_SPAWN_AGE {
                    if rng.gen::<f64>() < SPAWN_RATE {
                        new_fish = Some(Fish {
                            x: f.x,
                            y: f.y,
                            optimal_temperature: f.optimal_temperature,
                            age: 0,
                            alive: true,
                        });
                    }
                }
            };
            if new_fish.is_some() {
                x += 1;
                fish.push(new_fish.unwrap());
            }
        }
        print!(", {:6} spawned", x);

        // [3] Kill some fish

        if t_map.month <= 4 {
            // println!("> some fish are being eaten");

            fish.shuffle(&mut rng);

            let fish_eaten = rng.gen_range(0.1 * fish.len() as f64, 0.3 * fish.len() as f64) as usize;

            for i in 0..fish_eaten {
                fish.pop();
            }

            print!(", {:6} fish eaten", fish_eaten);
        } else {
            print!(", {:>6} fish eaten", "no");
        }

        // [4] Clear dead fish
        // println!("> clear dead fish");
        let o_len = fish.len();
        fish = fish.into_iter().filter(|fish| fish.alive).collect();
        print!(", {:6} fish left in this epoch, {:6} died", fish.len(), o_len - fish.len());

        // [5] Scotland Fish
        let mut cnt = 0;
        for f in &fish {
            let delta_x = f.x as f64 - SCOTLAND_CENTER_X as f64;
            let delta_y = f.y as f64 - SCOTLAND_CENTER_Y as f64;
            if delta_x * delta_x + delta_y * delta_y < SCOTLAND_RADIUS * SCOTLAND_RADIUS {
                cnt += 1;
            }
        }
        print!(", {:6} fish in Scotland", cnt);

        // [6] Plot distribution
        // println!("> plotting...");
        let mut img: image::RgbImage = image::RgbImage::new(t_map.width, t_map.height);
        copy_image(BACKGROUND_IMG, &mut img);

        for idx in 0..fish.len() {
            let f = &fish[idx];
            for x_offset in -1..2 {
                for y_offset in -1..2 {
                    let xx = f.x + x_offset;
                    let yy = f.y + y_offset;
                    if xx < 0 || yy < 0 || xx >= t_map.width as i64 || yy >= t_map.height as i64 {
                        continue;
                    }
                    let [mut r, mut g, mut b] = img.get_pixel(xx as u32, yy as u32).0;
                    if r as usize + 20 <= 255 {
                        r += 20;
                    }

                    img.put_pixel(xx as u32, yy as u32, Rgb([r, g, b]));
                }
            }
        }

        let img = imageproc::drawing::draw_hollow_circle(
            &img,
            (SCOTLAND_CENTER_X as i32, SCOTLAND_CENTER_Y as i32),
            SCOTLAND_RADIUS as i32,
            Rgb([0, 255, 0]));

        img.save(format!("result/{}-{}.png", t_map.year, t_map.month)).unwrap();
        img.save(format!("result/pic{:04}.png", id)).unwrap();
        id += 1;

        println!(", done in {:5}ms", begin_time.elapsed().unwrap().as_millis());
    }
}
