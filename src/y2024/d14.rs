use std::thread::sleep;
use std::time::{Duration, Instant};
use regex::Regex;
use crate::util::get_input_string;
use crate::util::point2::Point2;
use crate::util::point2::point;

struct Robot {
    initial_position: Point2,
    vel: Point2,
}

fn parse() -> Vec<Robot> {
    let regex = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

    regex.captures_iter(get_input_string(2024, 14).as_str())
        .map(|capture| {
            let mut numbers = capture.iter().skip(1).map(|c| c.unwrap().as_str().parse::<i32>().unwrap());
            Robot {
                initial_position: point(numbers.next().unwrap(), numbers.next().unwrap()),
                vel: point(numbers.next().unwrap(), numbers.next().unwrap()),
            }
        }).collect()
}

fn move_robot(map_size: &Point2, seconds: i32, robot: &Robot) -> Point2 {
    let mut x = (robot.initial_position.x + robot.vel.x * seconds) % map_size.x;
    let mut y = (robot.initial_position.y + robot.vel.y * seconds) % map_size.y;
    if x < 0 { x = map_size.x + x; }
    if y < 0 { y = map_size.y + y; }
    point(x, y)
}

fn part_one() -> i32 {
    //let map_size = point(11, 7);
    let map_size = point(101, 103);
    let seconds = 100;

    let mut robot_positions: Vec<Point2> = Vec::new();
    parse().into_iter().for_each(|robot| {
        robot_positions.push(move_robot(&map_size, seconds, &robot));
    });

    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;
    let mid_x = map_size.x / 2;
    let mid_y = map_size.y / 2;
    robot_positions.iter().for_each(|p| {
        if p.x < mid_x && p.y < mid_y { q1 += 1 }
        if p.x > mid_x && p.y < mid_y { q2 += 1 }
        if p.x < mid_x && p.y > mid_y { q3 += 1 }
        if p.x > mid_x && p.y > mid_y { q4 += 1 }
    });

    q1 * q2 * q3 * q4
}

fn is_tree(map: &[[bool; 103]; 101]) -> bool {
    let mut contiguous_robots = 0;
    for row in map {
        for col in row {
            if *col { 
                contiguous_robots += 1;
                if contiguous_robots == 10 { return true }
            }
            else { contiguous_robots = 0 }
        }
    }
    false
}

fn part_two() -> i32 {
    let map_size = point(101, 103);
    let robots = parse();

    let mut seconds = 0;
    loop {
        let mut map = [[false; 103]; 101];
        robots.iter().for_each(|robot| {
            let pos = move_robot(&map_size, seconds, &robot);
            map[pos.x as usize][pos.y as usize] = true;
        });
        
        let is_tree = is_tree(&map);
        if !is_tree {
            seconds += 1; 
            continue; 
        }

        println!("T={}", seconds);
        map.into_iter().map(|row| {
            row.into_iter().map(|b| if b { '#' } else { '.' }).collect::<String>()
        }).for_each(|s| println!("{}", s));
        println!();
        return seconds
    }
}

pub(crate) fn run() {
    let timestamp_first = Instant::now();
    println!("Part one: {:?} ({:?})", part_one(), timestamp_first.elapsed());
    let timestamp_second = Instant::now();
    println!("Part two: {:?} ({:?})", part_two(), timestamp_second.elapsed());
}