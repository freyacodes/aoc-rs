use std::cmp::min_by;
use std::collections::{HashMap, HashSet};
use std::time::Instant;
use log::error;
use crate::util;
use crate::util::point2::{point, Point2, CARDINALS};

fn recurse_region(map: &HashMap<Point2, char>, set: &mut HashSet<Point2>, target_char: char, point: Point2) {
    if set.contains(&point) { return; }
    if map.get(&point) != Some(&target_char) { return; }
    set.insert(point);
    point.cardinals().for_each(|p| recurse_region(map, set, target_char, p));
}

fn find_region(map: &HashMap<Point2, char>, origin: Point2) -> HashSet<Point2> {
    let mut set: HashSet<Point2> = HashSet::new();
    let target_char = map[&origin];

    recurse_region(map, &mut set, target_char, origin);

    set
}

fn count_circumference(set: &HashSet<Point2>) -> usize {
    set.iter().map(|point| {
        point.cardinals().filter(|c| !set.contains(c)).count()
    }).sum()
}

fn count_sides(set: &HashSet<Point2>) -> usize {
    let from_cardinals: usize = set.iter().map(|point| {
        let filled_neighbors: Vec<bool> = point.cardinals().map(|c| set.contains(&c)).collect();
        let fill_count = filled_neighbors.iter().filter(|b| **b).count();

        match fill_count {
            0 => 4,
            1 => 2,
            3 => 0,
            4 => 0,
            2 => {
                if filled_neighbors[0] == filled_neighbors[2] { 
                    0
                } else {
                    1
                }
            }
            _ => panic!()
        }
    }).sum();
    
    let from_diagonals: usize = set.iter().map(|point| {
        let neighbors = point.cardinals_and_diagonals()
            .map(|c| set.contains(&c))
            .collect::<Vec<bool>>();
        
        let mut corners = 0;
        if neighbors[0] && !neighbors[1] && neighbors[2] { corners += 1; }
        if neighbors[2] && !neighbors[3] && neighbors[4] { corners += 1; }
        if neighbors[4] && !neighbors[5] && neighbors[6] { corners += 1; }
        if neighbors[6] && !neighbors[7] && neighbors[0] { corners += 1; }
        
        corners
    }).sum();
    
    from_cardinals + from_diagonals
}

fn solve(part_two: bool) -> usize {
    let map = util::parse_char_map(2024, 12);
    let mut counted_points: HashSet<Point2> = HashSet::new();
    let mut price = 0;

    for point in map.keys() {
        if counted_points.contains(point) { continue; }
        let region = find_region(&map, *point);
        if part_two {
            price += count_sides(&region) * region.len();
        } else {
            price += count_circumference(&region) * region.len();
        }
        region.into_iter().for_each(|p| { counted_points.insert(p); })
    }

    price
}

pub(crate) fn run() {
    let timestamp_first = Instant::now();
    println!("Part one: {:?} ({:?})", solve(false), timestamp_first.elapsed());
    let timestamp_second = Instant::now();
    println!("Part two: {:?} ({:?})", solve(true), timestamp_second.elapsed());
}