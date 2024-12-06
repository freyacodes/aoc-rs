use crate::util::get_input_string;

fn parse() -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let full_string = get_input_string(2024, 5);
    let (upper, lower) = full_string.split_once("\n\n").unwrap();

    let rules: Vec<(u32, u32)> = upper.lines()
        .map(|line| line.split_once('|').unwrap())
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .collect();

    let updates: Vec<Vec<u32>> = lower.lines().map(|line| {
        line.split(',').map(|c| c.parse().unwrap()).collect()
    }).collect();

    (rules, updates)
}

fn get_relevant_rules(rules: &Vec<(u32, u32)>, update: &Vec<u32>) -> Vec<(u32, u32)> {
    rules.iter()
        .filter(|(former, latter)| {
            update.contains(former) && update.contains(latter)
        }).map(|r| *r)
        .collect()
}

fn follows_rules(rules: &Vec<(u32, u32)>, update: &Vec<u32>) -> bool {
    'rule_check: for (former, latter) in get_relevant_rules(rules, update).iter() {
        for page in update.iter() {
            if page == former { continue 'rule_check; }
            if page == latter { return false; }
        }
    }
    true
}

fn part_one() -> u32 {
    let (rules, updates) = parse();

    updates.iter()
        .filter(|update| follows_rules(&rules, update))
        .map(|update| update[update.len() / 2])
        .sum()
}

fn reorder_update(rules: &Vec<(u32, u32)>, update: &mut Vec<u32>) {
    let relevant_rules = get_relevant_rules(rules, &update);

    relevant_rules.iter().for_each(|(former, latter)| {
        let index_former = update.iter().position(|p| *p == *former).unwrap();
        let index_latter = update.iter().position(|p| *p == *latter).unwrap();
        
        if index_former < index_latter { return; }
        let page = update.remove(index_former);
        update.insert(index_latter, page);
    });

    if follows_rules(&relevant_rules, &update) { return; }
    reorder_update(rules, update)
}

fn part_two() -> u32 {
    let (rules, updates) = parse();

    updates.into_iter()
        .filter(|update| !follows_rules(&rules, update))
        .map(|mut update| {
            reorder_update(&rules, &mut update);
            update[update.len() / 2]
        }).sum()
}

pub fn run() {
    println!("Part one: {}", part_one());
    println!("Part two: {}", part_two());
}