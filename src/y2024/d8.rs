use std::collections::{HashMap, HashSet};
use std::time::Instant;
use crate::util::parse_char_map;
use crate::util::point2::Point2;

fn get_frequencies(map: &HashMap<Point2, char>) -> HashSet<char> {
    map.iter().filter(|(_, &c)| c != '.')
        .map(|(_, c)| *c)
        .collect()
}

fn get_antinodes(map: &HashMap<Point2, char>, frequency: char, harmonics: bool) -> HashSet<Point2> {
    let antennas = map.iter().filter(|(_, &c)| c == frequency)
        .map(|(&p, _)| p)
        .collect::<Vec<Point2>>();

    let mut antinodes: HashSet<Point2> = HashSet::new();
    antennas.iter().for_each(|antenna| {
        antennas.iter().for_each(|other| {
            if antenna == other { return; }
            let diff = other - antenna;
            let mut factor = 2;
            loop {
                let antinode = antenna + &(&diff * factor);
                if !map.contains_key(&antinode) { break; }
                antinodes.insert(antinode);
                if !harmonics { break; }
                factor += 1
            }
        });
        
        if harmonics { antinodes.insert(antenna.clone()); }
    });

    antinodes
}

fn solve(harmonics: bool) -> u32 {
    let map = parse_char_map(2024, 8);
    let mut antinodes: HashSet<Point2> = HashSet::new();
    get_frequencies(&map).into_iter().for_each(|frequency| {
        get_antinodes(&map, frequency, harmonics).into_iter().for_each(|a| { antinodes.insert(a); });
    });

    antinodes.len() as u32
}

fn part_two() -> u32 {
    0
}

pub(crate) fn run() {
    let timestamp_first = Instant::now();
    println!("Part one: {} ({:?})", solve(false), timestamp_first.elapsed());
    let timestamp_second = Instant::now();
    println!("Part two: {} ({:?})", solve(true), timestamp_second.elapsed());
}