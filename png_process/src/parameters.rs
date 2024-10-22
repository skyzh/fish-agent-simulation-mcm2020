mod parameters_mackerel;
//pub use parameters_mackerel::*;
mod parameters_herring;
pub use parameters_herring::*;

mod full_predict;
pub use full_predict::*;


mod fast_predict;
//pub use fast_predict::*;


pub const CROP_X: u32 = 1185;
pub const CROP_Y: u32 = 60;
pub const CROP_W: u32 = 1130;
pub const CROP_H: u32 = 540;

pub const IMAGE_W: u32 = 3600;
pub const IMAGE_H: u32 = 1800;

pub const SAVE_PREDICT_IMAGE: bool = false;
pub const BACKGROUND_IMG: &str = "NASAM_AVG.PNG";
pub const BACKGROUND_IMG_ORIG: &str = "NASAM_AVG_ORIG.PNG";
pub const PLOT_FONT: &[u8] = include_bytes!("/Library/Fonts/Arial Unicode.ttf");
pub const PLOT_FONT_SIZE: f32 = 24.0;
pub const OUTPUT_SCOTLAND_TEMPERATURE: bool = false;
pub const OUTPUT_EPOCH_JSON: bool = true;
pub const SAVE_CROP_IMAGE: bool = false;
pub const DRAW_GADGET: bool = false;
pub const DRAW_FISH: bool = false;
pub const OUTPUT_FFMPEG_SERIES: bool = false;
pub const OUTPUT_INDEX_IMAGE: bool = true;
pub const USE_ORIG_BACKGROUND: bool = false;
pub const JSON_DISABLE: bool = false;

pub fn MAKE_JSON_AFTER(year: i32, month: i32) -> bool {
    if JSON_DISABLE {
        return false;
    }
    if year % 10 == 0 && month == 9 {
        return true;
    }
    if year >= 2050 {
        return true;
    }
    return false;
}
