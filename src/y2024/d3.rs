use crate::util::get_input_string;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref MUL_REGEX: Regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    static ref ALL_REGEX: Regex = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
}

fn part_one() -> u32 {
    MUL_REGEX.captures_iter(&*get_input_string(2024, 3)).map(|c| {
        c.get(0).unwrap();
        let (_, [a, b]) = c.extract();
        a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap()
    }).sum()
}

fn part_two() -> u32 {
    let mut allowed = true;
    ALL_REGEX.captures_iter(&*get_input_string(2024, 3)).map(|c| {
        match c.get(0).unwrap().as_str() {
            "do()" => {
                allowed = true;
                0
            }
            "don't()" => {
                allowed = false;
                0
            }
            _ => {
                if !allowed { return 0; }
                let a = c.get(1).unwrap().as_str().parse::<u32>().unwrap();
                let b = c.get(2).unwrap().as_str().parse::<u32>().unwrap();
                a*b
            }
        }
    }).sum()
}

pub fn run() {
    println!("Part one: {}", part_one());
    println!("Part two: {}", part_two());
}