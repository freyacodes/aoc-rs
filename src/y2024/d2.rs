use std::cmp::Ordering::Equal;
use crate::util::get_input;

fn parse() -> Vec<Vec<u32>> {
    get_input(2024, 2)
        .iter()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        }).collect()
}

fn check_report(vec: &Vec<u32>) -> bool {
    let ordering = vec[0].partial_cmp(&vec[1]).unwrap();

    if ordering == Equal { return false; }

    for i in 0..(vec.len() - 1) {
        let a = vec[i];
        let b = vec[i + 1];

        if ordering != a.partial_cmp(&b).unwrap() { return false; }
        if a.abs_diff(b) > 3 { return false; }
    }
    true
}

fn part_one() -> usize {
    parse().into_iter().filter(check_report).count()
}

fn part_two() -> usize {
    parse().into_iter().filter(|vec| {
        if check_report(vec) { return true; }
        for i in 0..vec.len() {
            let mut new_vec = vec.clone();
            new_vec.remove(i);
            if (check_report(&new_vec)) { return true; }
        }
        false
    }).count()
}

pub fn run() {
    println!("Part one: {}", part_one());
    println!("Part two: {}", part_two());
}