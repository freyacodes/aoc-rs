use crate::util::point2::{point, Point2};
use crate::util::parse_char_map;
use std::collections::HashSet;

fn part_one() -> u32 {
    let map = parse_char_map(2024, 6);
    let directions = vec![point(0, -1), point(1, 0), point(0, 1), point(-1, 0)];

    let mut guard_pos = map.iter().find(|(_, &c)| c == '^').unwrap().0.clone();
    let mut direction_index = 0;
    let mut direction = directions[direction_index];
    let mut visited: HashSet<Point2> = HashSet::new();
    visited.insert(guard_pos);
    
    loop {
        println!("{:?} {:?}", guard_pos, direction);
        let new_position = &guard_pos + &direction;
        let location = map.get(&new_position);
        if location.is_none() { break; }
        if location.unwrap() == &'#' {
            direction_index += 1;
            direction = directions[direction_index % 4];
            continue
        }
        
        guard_pos = new_position;
        visited.insert(new_position);
    }
    
    visited.len() as u32
}

fn part_two() -> u32 {
    0
}

pub fn run() {
    println!("Part one: {}", part_one());
    println!("Part two: {}", part_two());
}