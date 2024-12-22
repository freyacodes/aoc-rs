use std::collections::{HashMap, HashSet};
use std::time::Instant;
use crate::util::{get_input_string, parse_char_map_from_string};
use crate::util::point2::{Point2, CARDINALS};
use crate::util::point2::point;

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
                map.remove(&robot_target);
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

fn debug_print(_robot_position: &Point2, _map: &HashMap<Point2, char>) {
    /*let mut char_map = [['.'; 14]; 7];

    for (p, c) in map {
        char_map[p.y as usize][p.x as usize] = *c;
    }
    char_map[robot_position.y as usize][robot_position.x as usize] = '@';
    char_map.iter().for_each(|row| { println!("{}", row.iter().collect::<String>()); });
    println!();*/
}

fn recurse_boxes_vertical(
    map: &HashMap<Point2, char>,
    pos: &Point2,
    movement: i32,
    set: &mut HashSet<Point2>
) -> bool {
    let me = map[&pos];
    let other_pos = pos + &point(if me == '[' { 1 } else { -1 }, 0);
    
    set.insert(pos.clone());
    set.insert(other_pos.clone());
    
    let parent_1_pos = point(pos.x, pos.y + movement);
    let parent_2_pos = point(other_pos.x, other_pos.y + movement);
    
    let parent_1_char = map.get(&parent_1_pos);
    let parent_2_char = map.get(&parent_2_pos);
    
    if parent_1_char == Some(&'#') { return false }
    if parent_2_char == Some(&'#') { return false }
    
    if parent_1_char.is_some() && !recurse_boxes_vertical(map, &parent_1_pos, movement, set) { return false }
    if parent_2_char.is_some() && !recurse_boxes_vertical(map, &parent_2_pos, movement, set) { return false }
    
    true
}

fn vertical_push(map: &mut HashMap<Point2, char>, movement: i32, pos: &Point2) -> bool {
    let mut set: HashSet<Point2> = HashSet::new();
    let can_push = recurse_boxes_vertical(map, pos, movement, &mut set);
    if !can_push { return false }
    
    let mut vec = set.into_iter().collect::<Vec<Point2>>();
    vec.sort_by(|a, b| a.y.cmp(&b.y));
    if movement == 1 { vec.reverse() }
    
    for from in vec.iter() {
        let to = point(from.x, from.y + movement);
        let char = map.remove(&from).unwrap();
        assert_eq!(map.insert(to, char), None);
    }
    
    true
}

fn part_two() -> i32 {
    let (original_map, mut robot_position, moves) = parse();
    
    robot_position = point(robot_position.x * 2, robot_position.y);
    let mut map: HashMap<Point2, char> = HashMap::new();
    for (p, c) in original_map {
        let p1 = point(p.x*2, p.y);
        let p2 = point(p.x*2+1, p.y);
        if c == 'O' {
            map.insert(p1, '[');
            map.insert(p2, ']');
        } else {
            map.insert(p1, c);
            map.insert(p2, c);
        }
    }

    'movement_loop: for mov in moves.chars() {
        debug_print(&robot_position, &map);
        let movement = movement_vector(mov);
        let robot_target = &robot_position + &movement;
        let immediate_obstacle = map.get(&robot_target);

        if immediate_obstacle.is_none() {
            robot_position = robot_target;
            continue 'movement_loop
        } else if immediate_obstacle == Some(&'#') {
            continue 'movement_loop
        }

        assert!(immediate_obstacle == Some(&'[') || immediate_obstacle == Some(&']'));

        let mut search_distance = 1;
        let is_vertical = movement.y != 0;
        
        if is_vertical {
            if vertical_push(&mut map, movement.y, &robot_target) {
                robot_position = robot_target;
            }
            continue 'movement_loop;
        }
        
        loop {
            search_distance += 1;
            let search_position = &robot_position + &(&movement * search_distance);
            let search_tile = map.get(&search_position);
            if search_tile.is_none() {
                for offset in (2..=search_distance).rev() {
                    let from = &robot_position + &(&movement * (offset - 1));
                    let to = &robot_position + &(&movement * offset);
                    let removed = map.remove(&from).unwrap();
                    map.insert(to, removed);
                }
                robot_position = robot_target;
                break;
            } else if search_tile == Some(&'#') {
                continue 'movement_loop
            }
        }
    }
    
    debug_print(&robot_position, &map);

    map.into_iter().filter(|(_, c)| *c == '[').map(|(p, _)| p.x + p.y * 100).sum()
}

pub(crate) fn run() {
    let timestamp_first = Instant::now();
    println!("Part one: {:?} ({:?})", part_one(), timestamp_first.elapsed());
    let timestamp_second = Instant::now();
    println!("Part two: {:?} ({:?})", part_two(), timestamp_second.elapsed());
}