mod parameters_mackerel;
mod fast_predict;

pub use parameters_mackerel::*;
pub use fast_predict::*;

pub const SAVE_PREDICT_IMAGE: bool = false;
pub const BACKGROUND_IMG: &str = "NASAM_AVG.PNG";
pub const BACKGROUND_IMG_ORIG: &str = "NASAM_AVG_ORIG.PNG";
pub const PLOT_FONT: &[u8] = include_bytes!("/Library/Fonts/Arial Unicode.ttf");
pub const PLOT_FONT_SIZE: f32 = 24.0;
pub const OUTPUT_SCOTLAND_TEMPERATURE: bool = false;
pub const OUTPUT_EPOCH_JSON: bool = true;
pub const SAVE_CROP_IMAGE: bool = false;
pub const DRAW_GADGET: bool = false;