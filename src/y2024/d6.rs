use crate::util::parse_char_map;
use crate::util::point2::{point, Point2};
use std::collections::{HashMap, HashSet};

fn simulate_guard(map: &HashMap<Point2, char>) -> Option<u32> {
    let directions = vec![point(0, -1), point(1, 0), point(0, 1), point(-1, 0)];

    let mut guard_pos = map.iter().find(|(_, &c)| c == '^').unwrap().0.clone();
    let mut direction_index = 0;
    let mut direction = directions[direction_index];
    let mut visited: HashSet<Point2> = HashSet::new();
    let mut turn_positions: HashSet<(Point2, Point2)> = HashSet::new();
    visited.insert(guard_pos);

    loop {
        let new_position = &guard_pos + &direction;
        let location = map.get(&new_position);
        if location.is_none() { break; }
        if location.unwrap() == &'#' {
            // Loop detection
            if turn_positions.contains(&(guard_pos, direction)) {
                return None
            }
            turn_positions.insert((guard_pos, direction));
            direction_index += 1;
            direction = directions[direction_index % 4];
            continue
        }

        guard_pos = new_position;
        visited.insert(new_position);
    }

    Some(visited.len() as u32)
}

fn part_one() -> u32 {
    let map = parse_char_map(2024, 6);
    simulate_guard(&map).unwrap()
}

fn part_two() -> u32 {
    let mut map = parse_char_map(2024, 6);
    
    let total = map.iter().filter(|(_, &c)| c == '.').count();
    let mut count = 0; 
    
    map.clone().into_iter()
        .filter(|(_, c)| c == &'.')
        .filter(|(p, _)| {
            map.insert(*p, '#');
            let loop_encountered = simulate_guard(&map).is_none();
            map.insert(*p, '.');
            count += 1;
            println!("{}/{}", count, total);
            loop_encountered
        }).count() as u32
}

pub fn run() {
    println!("Part one: {}", part_one());
    println!("Part two: {}", part_two());
}