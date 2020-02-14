use rand::prelude::*;
use crate::data::TemperatureMap;
use std::ops::Deref;
use image::Rgb;

pub struct Fish {
    pub x: i64,
    pub y: i64,
    pub age: usize,
    pub optimal_temperature: f64,
    pub alive: bool
}

pub fn one_epoch(map: &Vec<TemperatureMap>) {
    let mut thread_rng = rand::thread_rng();
    let mut rng = rand::rngs::SmallRng::from_rng(thread_rng).unwrap();
    let mut fish: Vec<Fish> = vec![];
    println!("spawning fish...");
    const FISH_SPAWN_INITIAL: usize = 100000;
    const FISH_MAX_AGE: usize = 17;
    const FISH_MAX_MOVE: i64 = 50;

    let initial_map = &map[0];
    for i in 0..FISH_SPAWN_INITIAL {
        let x = rng.gen_range(0, initial_map.width) as i64;
        let y = rng.gen_range(0, initial_map.height) as i64;
        let age = rng.gen_range(0, FISH_MAX_AGE);
        if initial_map.is_ocean(x, y) {
            fish.push(Fish {
                x,
                y,
                age,
                optimal_temperature: 10.0,
                alive: true
            })
        }
    }

    println!("{} fish spawned", fish.len());
    let mut id = 0;
    for t_map in map.iter() {
        println!("processing {}-{}", t_map.year, t_map.month);

        // [1] Fish move to optimal place
        println!("> moving fish");
        for idx in 0..fish.len() {
            let f = &mut fish[idx];

            // [1.1] Searching within FISH_MAX_MOVE radius the fish
            let mut optimal_place: Option<(f64, i64, i64)> = None;
            for x_offset in -FISH_MAX_MOVE..FISH_MAX_MOVE + 1 {
                for y_offset in -FISH_MAX_MOVE..FISH_MAX_MOVE + 1 {
                    if x_offset * x_offset + y_offset * y_offset > FISH_MAX_MOVE * FISH_MAX_MOVE {
                        continue;
                    }
                    let next_x = f.x as i64 + x_offset;
                    let next_y = f.y as i64 + y_offset;
                    if !t_map.in_range(next_x, next_y) {
                        continue;
                    }
                    if !t_map.is_ocean(next_x, next_y) {
                        continue;
                    }
                    let here_temp = t_map.get_temperature(next_x, next_y);
                    if optimal_place.is_some() {
                        let temp = optimal_place.unwrap().0;
                        let cmpa = (temp - f.optimal_temperature).abs();
                        let cmpb = (here_temp - f.optimal_temperature).abs();
                        if cmpa > cmpb || cmpa == cmpb && rng.gen::<f64>() < 0.5 {
                            optimal_place = Some((here_temp, next_x, next_y));
                        }
                    } else {
                        optimal_place = Some((here_temp, next_x, next_y));
                    }
                }
            }

            if optimal_place.is_some() {
                f.x = optimal_place.unwrap().1;
                f.y = optimal_place.unwrap().2;
            } else {
                // No optimal place found, kill the fish
                f.alive = false;
            }
        }

        // [N-1] Clear dead fish
        println!("> clear dead fish");
        fish = fish.into_iter().filter(|fish| fish.alive).collect();
        println!("> {} fish left in this epoch", fish.len());

        // [N] Plot distribution
        println!("> plotting...");
        let mut image = image::RgbImage::new(t_map.width, t_map.height);
        let image_original = image::open(&t_map.path).unwrap();
        let image_original = image_original.as_luma8().unwrap();

        for (x, y, pixel) in image.enumerate_pixels_mut() {
            let luma_color = image_original.get_pixel(x, y).0[0];
            *pixel = Rgb([luma_color, luma_color, luma_color]);
        }

        for idx in 0..fish.len() {
            let f = &fish[idx];
            for x_offset in -1..2 {
                for y_offset in -1..2 {
                    let xx = f.x + x_offset;
                    let yy = f.y + y_offset;
                    if xx < 0 || yy < 0 || xx >= t_map.width as i64 || yy >= t_map.height as i64 {
                        continue;
                    }
                    let [mut r, mut g, mut b] = image.get_pixel(xx as u32, yy as u32).0;
                    if r as usize + 20 <= 255 {
                        r += 20;
                    }

                    image.put_pixel(xx as u32, yy as u32, Rgb([r, g, b]));
                }
            }

        }

        image.save(format!("result/{}-{}.png", t_map.year, t_map.month)).unwrap();
        image.save(format!("result/pic{:04}.png", id)).unwrap();
        id += 1;
    }
}
