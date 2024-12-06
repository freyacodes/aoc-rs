use std::{fs, ops};
use std::path::Path;
pub(crate) mod point2;

pub fn get_input_string(year: u16, day: u8) -> String {
    fs::read_to_string(Path::new(&format!("input/{}/{}.txt", year, day))).unwrap()
}

pub fn get_input(year: u16, day: u8) -> Vec<String> {
    get_input_string(year, day).lines().map(|s| s.to_owned()).collect::<Vec<String>>()
}
