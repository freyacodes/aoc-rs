use crate::y2021::util;

pub fn run() {
    let mut part_one = 0;
    let mut iter = util::get_input(1)
        .into_iter()
        .map(|s| s.parse::<u32>().unwrap());
    let mut last = iter.next().unwrap();

    iter.for_each(|x| {
        if x > last {
            part_one += 1;
        }
        last = x
    });

    println!("Part one: {}", part_one) // 1476 too low
}
