use rand::prelude::*;
use crate::data::TemperatureMap;
use std::ops::Deref;
use image::{Rgb, Rgba};
use crate::parameters::*;
use rusttype::Font;
use crate::utils::*;
use crate::score::*;

pub struct Fish {
    pub x: i64,
    pub y: i64,
    pub age: usize,
    pub optimal_temperature: f64,
    pub alive: bool,
}

pub fn one_epoch(map: &Vec<TemperatureMap>) {
    let font_data: &[u8] = PLOT_FONT;
    let font: Font<'static> = Font::from_bytes(font_data).unwrap();

    let mut thread_rng = rand::thread_rng();
    let mut rng = rand::rngs::SmallRng::from_rng(thread_rng).unwrap();
    let mut fish: Vec<Fish> = vec![];
    println!("spawning fish...");
    let initial_map = &map[0];
    let land_score = LandScore::from_map(initial_map);

    if OUTPUT_INDEX_IMAGE {
        let img = land_score.generate_image(initial_map);
        img.save("result/land_score.png").unwrap();
    }

    {
        let _fish = vec![];
        let living = Living::from_map(initial_map, &_fish);
        for i in 0..FISH_SPAWN_INITIAL {
            let x = rng.gen_range(0, initial_map.width - 1) as i64;
            let y = rng.gen_range(0, initial_map.height - 1) as i64;
            let age = rng.gen_range(0, FISH_MAX_AGE);
            let optimal_temperature = rng.gen_range(OPTIMAL_TEMPERATURE_LOW, OPTIMAL_TEMPERATURE_HIGH);

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

    if OUTPUT_EPOCH_JSON {
        let mut file = std::fs::File::create("result/land.json").unwrap();
        land_score.export(&mut file).unwrap();
    }

    println!("{} fish spawned", fish.len());
    let mut id = 0;
    for (map_idx, t_map) in map.iter().enumerate() {
        let begin_time = std::time::SystemTime::now();
        print!("({:03}/{:03}) processing {:04}-{:02}", map_idx, map.len(), t_map.year, t_map.month);

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

        // [0] Calculate living index
        // println!("> generating living index");
        let living = Living::from_map(t_map, &fish);

        if OUTPUT_EPOCH_JSON && t_map.year >= MAKE_JSON_AFTER as i32 {
            let mut file = std::fs::File::create(format!("result/{}-{}-living.json", t_map.year, t_map.month)).unwrap();
            living.export(&mut file);
            let mut file = std::fs::File::create(format!("result/{}-{}.json", t_map.year, t_map.month)).unwrap();
            crate::export::export(&mut file, &fish).unwrap();
        }

        if OUTPUT_INDEX_IMAGE && t_map.year >= MAKE_JSON_AFTER as i32 {
            let img = living.generate_image();
            img.save(format!("result/food_{}-{}.png", t_map.year, t_map.month)).unwrap();
        }

        // [6] Plot distribution
        // println!("> plotting...");

        let mut img = match DRAW_FISH {
            true => crate::render::render(t_map, &fish),
            false => crate::render::render(t_map, &vec![])
        };

        if DRAW_GADGET {
            imageproc::drawing::draw_hollow_circle_mut(
                &mut img,
                (SCOTLAND_CENTER_X as i32, SCOTLAND_CENTER_Y as i32),
                SCOTLAND_RADIUS as i32,
                Rgba([0, 168, 204, 255]));

            imageproc::drawing::draw_text_mut(
                &mut img,
                Rgba([20, 40, 80, 255]),
                10, 30, rusttype::Scale::uniform(PLOT_FONT_SIZE),
                &font,
                format!("{}-{}, Scotland {}", t_map.year, t_map.month, cnt).as_str(),
            );
        }

        if t_map.year >= MAKE_JSON_AFTER as i32 {
            img.save(format!("result/{}-{}.png", t_map.year, t_map.month)).unwrap();

            if OUTPUT_FFMPEG_SERIES {
                img.save(format!("result/pic{:04}.png", id)).unwrap();
            }
        }

        print!(", {:.3} C", living.avg_temp());

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

            let fish_eaten = rng.gen_range(EATEN_RATE_MIN * fish.len() as f64, EATEN_RATE_MAX * fish.len() as f64) as usize;

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
        id += 1;

        println!(", done in {:5}ms", begin_time.elapsed().unwrap().as_millis());
    }
}
