use std::time::Instant;
use regex::Regex;
use crate::util::get_input_string;
use crate::util::point2::{point, Point2};

struct Machine {
    button_a: Point2,
    button_b: Point2,
    goal: Point2,
}

fn parse() -> Vec<Machine> {
    let regex = Regex::new(r"(\d+)\D+(\d+)\D+(\d+)\D+(\d+)\D+(\d+)\D+(\d+)").unwrap();
    regex.captures_iter(get_input_string(2024, 13).as_str())
        .map(|capture| {
            let mut numbers = capture.iter().skip(1).map(|c| c.unwrap().as_str().parse::<i32>().unwrap());
            Machine {
                button_a: point(numbers.next().unwrap(), numbers.next().unwrap()),
                button_b: point(numbers.next().unwrap(), numbers.next().unwrap()),
                goal: point(numbers.next().unwrap(), numbers.next().unwrap())
            }
        }).collect()
}

fn bruteforce() -> usize {
    parse().into_iter().map(|machine| {
        let max_a_presses = machine.goal.x / machine.button_a.x;

        (0..=max_a_presses).map(|a_presses| {
            let preliminary_position = &machine.button_a * a_presses;
            let remaining_to_goal = &machine.goal - &preliminary_position;
            let b_presses_x = remaining_to_goal.x as f32 / machine.button_b.x as f32;
            let b_presses_y = remaining_to_goal.y as f32 / machine.button_b.y as f32;
            
            if b_presses_x != b_presses_y || b_presses_y % 1f32 != 0f32 { return None };
            Some(a_presses * 3 + b_presses_x as i32)
        }).filter(|o| o.is_some())
            .map(|o| o.unwrap())
            .min()
            .unwrap_or(0) as usize
    }).sum()
}

fn part_one() -> usize {
    parse().into_iter().map(|machine| {
        let mut a_presses = 0;
        let impossible_number_of_a_presses = (machine.goal.x / machine.button_a.x) + 1;
        let mut did_step_one = false;
        let mut initial_a_presses = 0;
        let mut later_a_presses: Option<i32> = None;
        
        loop {
            // TODO: Handle step 2
            if a_presses > impossible_number_of_a_presses { 
                if did_step_one { break } else { return 0 }
            }
            if (machine.goal.x - a_presses * machine.button_a.x) % machine.button_b.x != 0 {
                a_presses += 1;
                continue
            }
            if (machine.goal.y - a_presses * machine.button_a.y) % machine.button_b.y != 0 {
                a_presses += 1;
                continue
            }
            
            if did_step_one { 
                later_a_presses = Some(a_presses);
                break
            } else {
                initial_a_presses = a_presses;
                did_step_one = true;
                a_presses += 1;
            }
        }

        let a_step_size = later_a_presses.unwrap_or(1000000) - initial_a_presses;
        let possible_steps = (impossible_number_of_a_presses - initial_a_presses) / a_step_size;
        (0..=possible_steps).map(|step| {
            let a_presses = initial_a_presses + a_step_size * step;
            let b_presses = (machine.goal.x - a_presses * machine.button_a.x) / machine.button_b.x;
            a_presses * 3 + b_presses
        }).min().unwrap() as usize
    }).sum()
}

pub(crate) fn run() {
    let timestamp_first = Instant::now();
    // 72980 too high
    // 33427
    println!("Part one: {:?} ({:?})", bruteforce(), timestamp_first.elapsed());
    //let timestamp_second = Instant::now();
    //println!("Part two: {:?} ({:?})", part_two(), timestamp_second.elapsed());
}