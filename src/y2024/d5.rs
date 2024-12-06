use crate::util;
use crate::util::get_input_string;

fn parse() -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let full_string = get_input_string(2024, 5);
    let (upper, lower) = full_string.split_once("\n\n").unwrap();

    let rules: Vec<(u32, u32)> = upper.lines()
        .map(|line| line.split_once('|').unwrap())
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .collect();

    let updates: Vec<Vec<u32>> = lower.lines().map(|line| {
        line.split(',').map(|c| c.parse().unwrap()).collect()
    }).collect();

    (rules, updates)
}

fn part_one() -> u32 {
    let (rules, updates) = parse();

    updates.iter()
        .filter(|update| {
            'rule_check: for (former, latter) in rules.iter() {
                let mut found_latter = false;
                for page in update.iter() {
                    if page == former { 
                        if found_latter {
                            return false;
                        } else {
                            continue 'rule_check;
                        }
                    }
                    if page == latter {
                        found_latter = true;
                    }
                }
            }
            true
        })
        .map(|update| update[update.len() / 2])
        .sum()
}

fn part_two() -> u32 {
    0
}

pub fn run() {
    println!("Part one: {}", part_one());
    println!("Part two: {}", part_two());
}