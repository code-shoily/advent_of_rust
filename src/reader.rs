use std::fs;

pub fn read_input(year: u16, day: u16) -> String {
    let filename = format!("./inputs/{}_{}.txt", year, day);
    fs::read_to_string(filename).expect("File not found")
}