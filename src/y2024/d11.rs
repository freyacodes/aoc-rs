use std::collections::HashMap;
use std::time::Instant;
use crate::util::get_input_string;

fn parse() -> HashMap<u64, u64> {
    let mut counts: HashMap<u64, u64> = HashMap::new();

    get_input_string(2024, 11)
        .split_ascii_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .for_each(|s| {
            counts.entry(s).and_modify(|c| *c += 1).or_insert(1);
        });

    counts
}

fn apply_rules(stone: u64) -> Vec<u64> {
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
}

fn one_blink(old: HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut new_counts: HashMap<u64, u64> = HashMap::new();
    
    old.into_iter().for_each(|(stone, factor)| {
        apply_rules(stone).iter().for_each(|&new_stone| {
            new_counts.entry(new_stone).and_modify(|c| *c += factor).or_insert(factor);
        });
    });

    new_counts
}

fn solve(iterations: u32) -> u64 {
    let mut stones = parse();

    for _ in 0..iterations {
        stones = one_blink(stones);
    }

    stones.into_iter().map(|(_,c)| c).sum()
}

pub(crate) fn run() {
    let timestamp_first = Instant::now();
    println!("Part one: {:?} ({:?})", solve(25), timestamp_first.elapsed());
    let timestamp_second = Instant::now();
    println!("Part two: {} ({:?})", solve(75), timestamp_second.elapsed());
}