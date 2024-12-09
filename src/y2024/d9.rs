use std::time::Instant;
use crate::util::get_input_string;

fn parse() -> Vec<u8> {
    get_input_string(2024, 9)
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect()
}

fn part_one() -> u64 {
    let mut disk: Vec<Option<u32>> = vec![];

    parse().into_iter().enumerate().for_each(|(i, input_digit)| {
        let is_free_space = i % 2 == 1;

        if is_free_space {
            for _ in 0..input_digit { disk.push(None); }
        } else {
            let id = (i / 2) as u32;
            for _ in 0..input_digit { disk.push(Some(id)); }
        }
    });

    let mut low_index = 0;
    let mut high_index = disk.len() - 1;
    'defragment: loop {
        loop {
            //println!("l{} {}", low_index, high_index);
            if disk[low_index].is_none() { break; };
            if low_index == high_index { break 'defragment; }
            low_index += 1;
        }

        loop {
            //println!("h{} {}", low_index, high_index);
            if disk[high_index].is_some() { break; };
            if low_index == high_index { break 'defragment; }
            high_index -= 1;
        }
        
        disk[low_index] = disk[high_index].take();
    }
    
    disk.into_iter()
        .filter(|&x| x.is_some())
        .enumerate()
        .map(|(i,f)| i as u64 * f.unwrap() as u64)
        .sum()
}

fn part_two() -> u64 {
    todo!()
}

pub(crate) fn run() {
    let timestamp_first = Instant::now();
    println!("Part one: {} ({:?})", part_one(), timestamp_first.elapsed());
    //let timestamp_second = Instant::now();
    //println!("Part two: {} ({:?})", part_two(), timestamp_second.elapsed());
}
