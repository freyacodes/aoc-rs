use crate::util;

fn count_increases<'a>(iter: &mut impl Iterator<Item=&'a u64>) -> u64 {
    let mut last = iter.next().unwrap();
    iter.fold(0, |acc, x| {
        let b = x > last;
        last = x;
        if b { acc + 1 } else { acc }
    })
}

pub fn run() {
    let input: Vec<u64> = util::get_input(2021, 1)
        .iter()
        .map(|s| s.parse().unwrap())
        .collect();

    let part_one = count_increases(&mut input.iter());
    let part_two = count_increases(&mut input.windows(3)
        .map(|x| {
            x.iter().sum::<u64>()
        }).collect::<Vec<u64>>().iter());

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);
}
