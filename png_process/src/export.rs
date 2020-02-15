use crate::fish::Fish;
use std::io;
use std::fs::File;
use std::io::Write;
use crate::score::Living;

fn export_fish(file: &mut File, fish: &Fish) -> io::Result<()> {
    file.write(b"{")?;
    file.write(format!("\"x\": {}, \"y\": {}, \"age\": {}", fish.x, fish.y, fish.age).as_bytes())?;
    file.write(b"}")?;
    Ok(())
}

pub fn export(file: &mut File, fish: &Vec<Fish>) -> io::Result<()> {
    file.write(b"{ \"fish\": [\n")?;
    let mut is_first = true;
    for f in fish {
        if is_first {
            is_first = false;
        } else {
            file.write(b",")?;
        }
        export_fish(file, f)?;
    }
    file.write(b"\n] }")?;
    Ok(())
}
