pub const SCOTLAND_CENTER_X: u32 = 563;
pub const SCOTLAND_CENTER_Y: u32 = 310;
pub const SCOTLAND_RADIUS: f64 = 80.0;
pub const CROP_X: u32 = 1185;
pub const CROP_Y: u32 = 60;
pub const CROP_W: u32 = 1130;
pub const CROP_H: u32 = 540;
pub const LAND_SCORE_K: f64 = 1.0;
pub const FOOD_SCORE_K: f64 = 1.0 / 100.0;
pub const TEMP_SCORE_K: f64 = 1.0;
pub const AGE_SCORE_K: f64 = 0.5;
pub const NORMAL_K: f64 = 0.1;
pub const SCORE_THRESHOLD: f64 = -10.0;
pub const FISH_SPAWN_INITIAL: usize = 100000;
pub const FISH_MAX_AGE: usize = 17 * 12;
pub const FISH_MAX_MOVE: i64 = 50;
pub const FISH_MIN_MOVE: i64 = 0;
pub const FOOD_SCORE_SPREAD_RANGE: i64 = 5;
pub const LAND_SCORE_SPREAD_RANGE: i64 = 50;
pub const SPAWN_RATE: f64 = 0.5;
pub const OPTIMAL_SPAWN_AGE: usize = 12 * 2;
pub const SAVE_CROP_IMAGE: bool = false;
pub const OUTPUT_SCOTLAND_TEMPERTURE: bool = false;


pub const PREDICT_LOOK_BACKWARD_YEAR: usize = 5;
pub const PREDICT_MONTH: usize = 30 * 12 + 3;
pub const DATA_YEAR: i32 = 2021;

/*
pub const PREDICT_LOOK_BACKWARD_YEAR: usize = 1;
pub const PREDICT_MONTH: usize = 1;
pub const DATA_YEAR: i32 = 2003;
*/

pub const SAVE_PREDICT_IMAGE: bool = false;
pub const BACKGROUND_IMG: &str = "NASAM_AVG.PNG";
pub const BACKGROUND_IMG_ORIG: &str = "NASAM_AVG_ORIG.PNG";
pub const PLOT_FONT: &[u8] = include_bytes!("/Library/Fonts/Arial Unicode.ttf");
pub const PLOT_FONT_SIZE: f32 = 24.0;