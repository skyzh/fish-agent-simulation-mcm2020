mod data;
mod fish;

fn main() {
    let data = data::loadData();
    // first of all, randomly spawn fish
    fish::one_epoch(&data);
}
