use std::collections::{HashMap, HashSet};
use std::time::Instant;
use crate::util;
use crate::util::point2::Point2;

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

fn part_one() -> usize {
    let map = util::parse_char_map(2024, 12);
    let mut counted_points: HashSet<Point2> = HashSet::new();
    let mut price = 0;

    for point in map.keys() {
        if counted_points.contains(point) { continue; }
        let region = find_region(&map, *point);
        price += count_circumference(&region) * region.len();
        region.into_iter().for_each(|p| { counted_points.insert(p); })
    }
    
    price
}

pub(crate) fn run() {
    let timestamp_first = Instant::now();
    println!("Part one: {:?} ({:?})", part_one(), timestamp_first.elapsed());
    //let timestamp_second = Instant::now();
    //println!("Part two: {:?} ({:?})", part_two(), timestamp_second.elapsed());
}