mod data;
mod fish;
mod predict;
mod parameters;

fn main() {
    let data = data::load_data();
    // first of all, randomly spawn fish
    fish::one_epoch(&data);
}
