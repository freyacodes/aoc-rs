use std::fs;
use std::path::Path;

pub fn get_input(day: u8) -> Vec<String> {
    let full_string = fs::read_to_string(Path::new(&format!("input/2021/{}.txt", day))).unwrap();
    full_string.lines().map(|s| s.to_owned()).collect::<Vec<String>>()
}
