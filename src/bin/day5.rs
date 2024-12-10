use std::fs::read_to_string;

struct Ordering {
    left: u64,
    right: u64,
}

fn main() {
    let input: Vec<String> = read_to_string("inputs/day5")
        .unwrap()
        .split("\n\n")
        .map(|x| x.to_string())
        .collect();
    let ordering_rules: Vec<Ordering> = input[0]
        .split('\n')
        .filter(|l| l.len() > 0)
        .map(|ordering| ordering.split_once('|').unwrap())
        .map(|(left, right)| (left.parse::<u64>().unwrap(), right.parse::<u64>().unwrap()))
        .map(|(left, right)| Ordering { left, right })
        .collect();

    let update: Vec<Vec<u64>> = input[1]
        .split('\n')
        .filter(|l| l.len() > 0)
        .map(|ordering| {
            ordering
                .split(',')
                .map(|page| page.parse::<u64>().unwrap())
                .collect()
        })
        .collect();

    println!("{}", p1(&ordering_rules, &update));
    println!("{}", p2(&ordering_rules, &update));
}

fn p1(ordering_rules: &Vec<Ordering>, updates: &Vec<Vec<u64>>) -> u64 {
    let has_correct_ordering = |update: &Vec<u64>| {
        ordering_rules.iter().all(|ordering_rule| {
            let split: Vec<&[u64]> = update.split(|x| *x == ordering_rule.right).collect();
            split.len() == 1
                || !update.contains(&ordering_rule.left)
                || split[0].contains(&ordering_rule.left)
        })
    };
    updates
        .iter()
        .filter(|update| has_correct_ordering(update))
        .map(|update| update[update.len() / 2])
        .sum()
}

fn p2(ordering_rules: &Vec<Ordering>, updates: &Vec<Vec<u64>>) -> u64 {
    let _ = (ordering_rules, updates);
    2
}
