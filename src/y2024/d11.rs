use std::collections::HashMap;
use std::time::Instant;
use crate::util::get_input_string;

fn parse() -> Vec<u64> {
    get_input_string(2024, 11)
        .split_ascii_whitespace()
        .map(|s| s.parse::<u64>().unwrap()).collect()
}

fn part_one() -> usize {
    let mut stones = parse();

    for _ in 0..25 {
        stones = stones.into_iter()
            .flat_map(|stone| {
                if stone == 0 {
                    return vec![1]
                }
                let magnitude = stone.ilog10() + 1;
                if magnitude % 2 == 0 {
                    let divisor = 10u64.pow(magnitude/2);
                    vec![stone / divisor, stone % divisor]
                } else {
                    vec![stone * 2024]
                }
            }).collect();
    }

    stones.len()
}

fn part_two() {

}

pub(crate) fn run() {
    let timestamp_first = Instant::now();
    println!("Part one: {:?} ({:?})", part_one(), timestamp_first.elapsed());
    //let timestamp_second = Instant::now();
    //println!("Part two: {} ({:?})", part_two(), timestamp_second.elapsed());
}