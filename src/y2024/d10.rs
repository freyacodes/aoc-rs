use crate::util::point2::{point, Point2, CARDINALS};
use crate::util::parse_char_map;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

fn parse() -> HashMap<Point2, u8> {
    parse_char_map(2024, 10).into_iter()
        .map(|(k, v)| (k, v.to_digit(10).unwrap() as u8))
        .collect()
}

fn search_slope(map: &HashMap<Point2, u8>, p: &Point2, next_height: u8) -> Vec<Point2> {
    CARDINALS.iter().map(|cardinal| p + cardinal).filter(|neighbor| {
        let h = map.get(&neighbor);
        h == Some(&next_height)
    }).collect()
}

fn get_slope_score(map: &HashMap<Point2, u8>, p: Point2, part_two: bool) -> u32 {
    let mut points = vec![p];
    for next_height in 1..10 {
        points = points.into_iter().map(|p| search_slope(map, &p, next_height)).flatten().collect();
    }
    if part_two {
        points.len() as u32
    } else {
        points.iter().collect::<HashSet<&Point2>>().len() as u32
    }
}

fn solve(part_two: bool) -> u32 {
    let map = parse();

    map.iter().filter(|(_, &h)| h == 0)
        .map(|(p, _)| get_slope_score(&map, *p, part_two)).sum()
}

pub(crate) fn run() {
    let timestamp_first = Instant::now();
    println!("Part one: {} ({:?})", solve(false), timestamp_first.elapsed());
    let timestamp_second = Instant::now();
    println!("Part two: {} ({:?})", solve(true), timestamp_second.elapsed());
}
