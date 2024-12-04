use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let input: Vec<(i64, i64)> = read_to_string("inputs/day1")
        .unwrap()
        .split('\n')
        .filter(|l| l.len() > 0)
        .map(|l| l.split_once(' ').unwrap())
        .map(|(a, b)| {
            (
                a.trim().parse::<i64>().unwrap(),
                b.trim().parse::<i64>().unwrap(),
            )
        })
        .collect();
    p1(input.clone());
    p2(input);
}

fn p1(input: Vec<(i64, i64)>) {
    let mut left: Vec<i64> = input.iter().map(|(a, _)| *a).collect();
    let mut right: Vec<i64> = input.iter().map(|(_, b)| *b).collect();
    left.sort();
    right.sort();
    let mut sum: i64 = 0;
    for i in 0..left.len() {
        sum += (left.get(i).unwrap() - right.get(i).unwrap()).abs();
    }
    println!("{}", sum);
}

fn p2(input: Vec<(i64, i64)>) {
    let right = input
        .iter()
        .map(|(_, b)| *b)
        .fold(HashMap::new(), |mut right_map, number| {
            right_map
                .entry(number)
                .and_modify(|count| *count = *count + 1)
                .or_insert(1);
            right_map
        });
    let sum: i64 = input
        .iter()
        .map(|(a, _)| *a)
        .map(|num| num * right.get(&num).map_or(0, |a| *a))
        .sum();
    println!("{}", sum);
}
