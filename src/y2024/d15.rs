use std::collections::HashMap;
use std::time::Instant;
use crate::util::{get_input_string, parse_char_map_from_string};
use crate::util::point2::{Point2, CARDINALS};

fn parse() -> (HashMap<Point2, char>, Point2, String) {
    let input_str = get_input_string(2024, 15);
    let (map_str, moves_str) = input_str.split_once("\n\n").unwrap();
    
    let mut map = parse_char_map_from_string(map_str.to_string());
    let robot_position = *map.iter().find(|(_, &c)| c == '@').unwrap().0;
    map = map.into_iter().filter(|(_, v)| *v != '.' && *v != '@').collect();
    
    let moves = moves_str.chars().filter(|&c| c != '\n').collect::<String>();

    (map, robot_position, moves)
}

fn movement_vector(char: char) -> Point2 {
    match char { 
        '>' => CARDINALS[0],
        'v' => CARDINALS[1],
        '<' => CARDINALS[2],
        '^' => CARDINALS[3],
        _ => unreachable!(),
    }
}

fn debug_print(robot_position: &Point2, map: &HashMap<Point2, char>) {
    /*let mut char_map = [['.'; 8]; 8];
    
    for (p, c) in map {
        char_map[p.y as usize][p.x as usize] = *c;
    }
    char_map[robot_position.y as usize][robot_position.x as usize] = '@';
    char_map.iter().for_each(|row| { println!("{}", row.iter().collect::<String>()); });
    println!();*/
}

fn part_one() -> i32 {
    let (mut map, mut robot_position, moves) = parse();
    
    'movement_loop: for mov in moves.chars() {
        let movement = movement_vector(mov);
        let robot_target = &robot_position + &movement;
        
        let immediate_obstacle = map.get(&robot_target);
        
        if immediate_obstacle.is_none() { 
            robot_position = robot_target;
            continue 'movement_loop
        } else if immediate_obstacle == Some(&'#') {
            continue 'movement_loop
        }
        
        assert_eq!(immediate_obstacle, Some(&'O'));

        let mut search_distance = 1;
        loop {
            search_distance += 1;
            let search_position = &robot_position + &(&movement * search_distance);
            let search_tile = map.get(&search_position);
            if search_tile.is_none() {
                let test = map.remove(&robot_target);
                map.insert(search_position, 'O');
                robot_position = robot_target;
                break;
            } else if search_tile == Some(&'#') {
                continue 'movement_loop
            }
        }
    }
    
    map.into_iter().filter(|(_, c)| *c == 'O').map(|(p, _)| p.x + p.y * 100).sum()
}

fn part_two() {
    let (original_map, mut robot_position, moves) = parse();
    
    
}

pub(crate) fn run() {
    let timestamp_first = Instant::now();
    println!("Part one: {:?} ({:?})", part_one(), timestamp_first.elapsed());
    let timestamp_second = Instant::now();
    println!("Part two: {:?} ({:?})", part_two(), timestamp_second.elapsed());
}