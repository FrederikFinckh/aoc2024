use std::fs::read_to_string;

#[derive(Clone)]
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
    let orderings: Vec<Ordering> = input[0]
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

    println!("{}", p1(&orderings, &update));
    println!("{}", p2(&orderings, &update));
}

fn p1(orderings: &Vec<Ordering>, updates: &Vec<Vec<u64>>) -> u64 {
    let has_correct_ordering = |update: &Vec<u64>| {
        orderings.iter().all(|ordering_rule| {
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

fn p2(orderings: &Vec<Ordering>, updates: &Vec<Vec<u64>>) -> u64 {
    let has_correct_ordering = |update: &Vec<u64>| {
        orderings.iter().all(|ordering_rule| {
            let split: Vec<&[u64]> = update.split(|x| *x == ordering_rule.right).collect();
            split.len() == 1
                || !update.contains(&ordering_rule.left)
                || split[0].contains(&ordering_rule.left)
        })
    };
    updates
        .iter()
        .filter(|update| !has_correct_ordering(update))
        .map(|update| fix_ordering(orderings, update))
        .map(|fixed_update| fixed_update[fixed_update.len() / 2])
        .sum()
}

fn fix_ordering(orderings: &Vec<Ordering>, update: &Vec<u64>) -> Vec<u64> {
    let num_before_and_after: Vec<(u64, Vec<u64>, Vec<u64>)> = update
        .iter()
        .map(|x| {
            (
                *x,
                // must be before thoes entries
                orderings
                    .iter()
                    .filter(|rule| update.contains(&rule.right))
                    .filter(|rule| rule.left == *x)
                    .map(|ordering| ordering.right)
                    .collect::<Vec<u64>>(),
                // must be after these entries
                orderings
                    .iter()
                    .filter(|rule| update.contains(&rule.left))
                    .filter(|rule| rule.right == *x)
                    .map(|ordering| ordering.left)
                    .collect::<Vec<u64>>(),
            )
        })
        .collect();

    let mut built_up: Vec<u64> = Vec::new();
    while built_up.len() < update.len() {
        built_up.append(
            &mut num_before_and_after
                .iter()
                .filter(|(x, _, _)| !built_up.contains(x))
                .filter(|(_x, must_be_before, must_be_after)| {
                    must_be_after.iter().all(|y| built_up.contains(y))
                        && !must_be_before.iter().any(|y| built_up.contains(y))
                })
                .map(|(x, _, _)| *x)
                .collect(),
        );
    }

    built_up
}
