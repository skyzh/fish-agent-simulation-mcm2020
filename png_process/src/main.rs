mod data;
mod fish;
mod predict;
mod parameters;
mod utils;
mod score;
mod render;
mod export;

fn main() {
    let data = data::load_data();
    // first of all, randomly spawn fish
    fish::one_epoch(&data);
}
