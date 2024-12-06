use std::collections::HashMap;
use crate::util::get_input;
use crate::util::point2::{point, Point2, CARDINALS_AND_DIAGONALS};

fn parse() -> HashMap<Point2, char> {
    let mut map: HashMap<Point2, char> = HashMap::new();
    get_input(2024, 4)
        .into_iter()
        .enumerate()
        .for_each(|(y, line)| {
            line.char_indices().for_each(|(x, c)| {
                map.insert(point(x as i32, y as i32), c);
            })
        });
    map
}

fn part_one() -> u32 {
    let map = parse();
    let mut matches = 0;
    map.iter()
        .filter(|(_, &c)| c == 'X')
        .for_each(|(p, _)| {
            CARDINALS_AND_DIAGONALS.iter().for_each(|dir| {
                if map.get(&(p + dir)) == Some(&'M')
                    && map.get(&(p + &(dir * 2))) == Some(&'A')
                    && map.get(&(p + &(dir * 3))) == Some(&'S') {
                    matches += 1;
                }
            })
        });
    matches
}

fn part_two() -> u32 {
    let directions = vec![point(-1, -1), point(1, -1), point(1, 1), point(-1, 1)];
    let map = parse();
    let mut matches = 0;
    map.iter()
        .filter(|(_, &c)| c == 'A')
        .for_each(|(p, _)| {
            let diagonals = directions.iter()
                .map(|dir| map.get(&(p + dir)))
                .collect::<Vec<Option<&char>>>();

            for diagonal in diagonals.iter() {
                if *diagonal != Some(&'M') && *diagonal != Some(&'S') {
                    return;
                }
            }
            
            if (diagonals[0] == diagonals[2] || diagonals[1] == diagonals[3]) {
                return;
            }
            matches += 1;
        });
    matches
}

pub fn run() {
    println!("Part one: {}", part_one());
    println!("Part two: {}", part_two());
}
