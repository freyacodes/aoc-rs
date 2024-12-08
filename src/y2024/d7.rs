use std::time::Instant;
use regex::Regex;
use crate::util::get_input;

fn parse() -> Vec<(u64, Vec<u64>)> {
    let regex = Regex::new(r"\d+").unwrap();
    get_input(2024, 7).iter().map(|s| {
        let mut numbers = regex.captures_iter(s).map(|c| {
            c.get(0).unwrap().as_str().parse::<u64>().unwrap()
        }).collect::<Vec<u64>>();
        
        (numbers.remove(0), numbers)
    }).collect::<Vec<(u64, Vec<u64>)>>()
}

fn calc(target: u64, acc: u64, remainder: &[u64], concat: bool) -> bool {
    if acc == target { return true; }
    if remainder.is_empty() { return false; }
    
    if calc(target, acc + remainder[0], &remainder[1..], concat) {
        true
    } else if calc(target, acc * remainder[0], &remainder[1..], concat) {
        true
    } else if concat {
        let right = remainder[0];
        let left = 10u64.pow(right.ilog10() + 1) * acc;
        calc(target, left + right, &remainder[1..], concat)
    } else { 
        false
    }
}

fn solve(part_two: bool) -> u64 {
    let input = parse();
    input.iter()
        .filter(|(target, numbers)| calc(*target, numbers[0], &numbers[1..], part_two))
        .map(|(target, _)| *target)
        .sum()
}

pub(crate) fn run() {
    let timestamp_first = Instant::now();
    println!("Part one: {} ({:?})", solve(false), timestamp_first.elapsed());
    let timestamp_second = Instant::now();
    println!("Part two: {} ({:?})", solve(true), timestamp_second.elapsed());
}