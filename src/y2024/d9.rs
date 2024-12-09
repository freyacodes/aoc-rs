use std::time::Instant;
use crate::util::get_input_string;

struct FileEntry {
    initial_index: usize,
    length: u8,
    id: u32
}

fn parse() -> (Vec<Option<u32>>, Vec<FileEntry>) {
    let mut disk: Vec<Option<u32>> = vec![];
    let mut file_indices: Vec<FileEntry> = vec![];

    get_input_string(2024, 9)
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .enumerate()
        .for_each(|(i, input_digit)| {
            let is_free_space = i % 2 == 1;

            if is_free_space {
                for _ in 0..input_digit { disk.push(None); }
            } else {
                let id = (i / 2) as u32;
                file_indices.push(FileEntry {
                    initial_index: disk.len(),
                    length: input_digit,
                    id,
                });
                for _ in 0..input_digit { disk.push(Some(id)); }
            }
        });

    (disk, file_indices)
}

fn calc_checksum(disk: Vec<Option<u32>>) -> u64 {
    disk.into_iter()
        .enumerate()
        .map(|(i, f)| i as u64 * f.unwrap_or(0) as u64)
        .sum()
}

fn part_one() -> u64 {
    let mut disk = parse().0;

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

    calc_checksum(disk)
}

fn part_two() -> u64 {
    let (mut disk, file_indices) = parse();
    
    file_indices.into_iter().rev().for_each(|entry| {
        let mut successive_empty_sectors = 0;
        let mut free_index: Option<usize> = None;
        for (i, sector) in disk.iter().enumerate() {
            if i == entry.initial_index { return; } 
            if sector.is_none() { successive_empty_sectors += 1; }
            else { successive_empty_sectors = 0; }
            //println!("{}", successive_empty_sectors);

            if successive_empty_sectors == entry.length {
                free_index = Some(i - entry.length as usize + 1);
                break
            }
        }

        free_index.inspect(|free_index| {
            for i in 0..entry.length as usize {
                disk[free_index + i] = disk[entry.initial_index + i].take();
            }
        });

        //let test = disk.iter().map(|o| o.map(|n| (n + 48) as u8 as char)).map(|o| o.unwrap_or('.')).collect::<String>();
        //println!("{}", test);
    });

    calc_checksum(disk)
}

pub(crate) fn run() {
    let timestamp_first = Instant::now();
    println!("Part one: {} ({:?})", part_one(), timestamp_first.elapsed());
    let timestamp_second = Instant::now();
    println!("Part two: {} ({:?})", part_two(), timestamp_second.elapsed());
}
