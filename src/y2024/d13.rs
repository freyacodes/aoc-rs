use crate::util::get_input_string;
use crate::util::point2l::{point, Point2L};
use regex::Regex;
use std::time::Instant;

#[derive(Debug)]
struct Machine {
    a: Point2L,
    b: Point2L,
    goal: Point2L,
}

fn parse() -> Vec<Machine> {
    let regex = Regex::new(r"(\d+)\D+(\d+)\D+(\d+)\D+(\d+)\D+(\d+)\D+(\d+)").unwrap();
    regex.captures_iter(get_input_string(2024, 13).as_str())
        .map(|capture| {
            let mut numbers = capture.iter().skip(1).map(|c| c.unwrap().as_str().parse::<i64>().unwrap());
            Machine {
                a: point(numbers.next().unwrap(), numbers.next().unwrap()),
                b: point(numbers.next().unwrap(), numbers.next().unwrap()),
                goal: point(numbers.next().unwrap(), numbers.next().unwrap()),
            }
        }).collect()
}

fn solve(offset: i64) -> i64 {
    parse().into_iter().map(|machine| {
        let goal = &machine.goal + &point(offset, offset);
        let d = machine.a.x * machine.b.y - machine.b.x * machine.a.y;
        let a = (goal.x * machine.b.y - goal.y * machine.b.x) / d;
        let b = (machine.a.x * goal.y - machine.a.y * goal.x) / d;
        if &(&machine.a * a) + &(&machine.b * b) == goal {
            a * 3 + b
        } else {
            0
        }
    }).sum()
}

pub(crate) fn run() {
    let timestamp_first = Instant::now();
    println!("Part one: {:?} ({:?})", solve(0), timestamp_first.elapsed());
    let timestamp_second = Instant::now();
    println!("Part two: {:?} ({:?})", solve(10000000000000), timestamp_second.elapsed());
}