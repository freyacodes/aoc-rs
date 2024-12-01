use crate::util;

fn parse() -> (Vec<u32>, Vec<u32>) {
    let tuples = util::get_input(2024, 1).iter().map(|line| {
        let mut iter = line.split_ascii_whitespace();
        let l = iter.next().unwrap().parse::<u32>().unwrap();
        let r = iter.next().unwrap().parse::<u32>().unwrap();
        (l, r)
    }).collect::<Vec<(u32, u32)>>();

    tuples.iter().map(|t| t.1)
        .zip(tuples.iter().map(|t| t.0))
        .collect()
}

fn part_one() -> u32 {
    let (mut left, mut right) = parse();

    left.sort();
    right.sort();

    left.into_iter()
        .zip(right.into_iter())
        .map(|(l, r)| l.abs_diff(r))
        .sum()
}

fn part_two() -> u32 {
    let (left, right) = parse();

    left.into_iter().map(|l| {
        right.iter().filter(|&&r| r == l).sum::<u32>()
    }).sum()
}

pub fn run() {
    println!("Part one: {}", part_one());
    println!("Part two: {}", part_two());
}