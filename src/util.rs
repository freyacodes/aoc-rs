use std::collections::HashMap;
use std::path::Path;
use std::fs;
use crate::util::point2::{point, Point2};

pub(crate) mod point2;

pub fn get_input_string(year: u16, day: u8) -> String {
    fs::read_to_string(Path::new(&format!("input/{}/{}.txt", year, day))).unwrap()
}

pub fn get_input(year: u16, day: u8) -> Vec<String> {
    get_input_string(year, day).lines().map(|s| s.to_owned()).collect::<Vec<String>>()
}

pub fn parse_char_map(year: u16, day: u8) -> HashMap<Point2, char> {
    let mut map: HashMap<Point2, char> = HashMap::new();
    get_input(year, day).into_iter()
        .enumerate()
        .for_each(|(y, line)| {
            line.char_indices().for_each(|(x, c)| {
                map.insert(point(x as i32, y as i32), c);
            })
        });
    map
}
