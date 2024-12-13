use std::{collections::HashMap, env::args, fs::read_to_string};

fn main() {
    let input: Vec<u64> = read_to_string(format!(
        "inputs/day11{}",
        match args().last() {
            Some(x) if x == "test".to_string() => "_test",
            _ => "",
        }
    ))
    .unwrap()
    .trim()
    .split_ascii_whitespace()
    .filter(|x| x.len() > 0)
    .map(|x| x.parse::<u64>().unwrap())
    .collect();
    println!("{}", p1(&input));
    println!("{}", p2(&input));
}

fn blink(stones: &Vec<u64>) -> Vec<u64> {
    let mut new_stones = stones.clone();
    for index in (0..stones.len()).rev() {
        let stone = stones[index];
        if stone == 0 {
            new_stones[index] = 1;
        } else if stone.to_string().len() % 2 == 0 {
            let stone_string: Vec<char> = stone.to_string().chars().collect();
            let left_stone = stone_string[0..stone_string.len() / 2]
                .iter()
                .collect::<String>()
                .parse::<u64>()
                .unwrap();
            let right_stone = stone_string[stone_string.len() / 2..]
                .iter()
                .collect::<String>()
                .parse::<u64>()
                .unwrap();
            new_stones[index] = right_stone;
            new_stones.insert(index, left_stone);
        } else {
            new_stones[index] = stone * 2024;
        }
    }
    new_stones
}

fn p1(input: &Vec<u64>) -> usize {
    let mut stones = input.clone();
    for _ in 0..25 {
        stones = blink(&stones);
    }
    stones.len()
}
//impl Iterator<Item = u64>
fn blink182(
    stones: &Vec<u64>,
    mut cache: HashMap<u64, Vec<u64>>,
) -> (Vec<u64>, HashMap<u64, Vec<u64>>) {
    (
        stones
            .iter()
            .flat_map(|stone| match cache.get(stone) {
                Some(x) => x.clone(),
                None => {
                    let stones = update_single_stone(*stone);
                    cache.insert(*stone, stones.clone());
                    stones
                }
            })
            .collect(),
        cache.clone(),
    )
}

fn update_single_stone(stone: u64) -> Vec<u64> {
    if stone == 0 {
        vec![1]
    } else if stone.to_string().len() % 2 == 0 {
        let stone_string: Vec<char> = stone.to_string().chars().collect();
        let left_stone = stone_string[0..stone_string.len() / 2]
            .iter()
            .collect::<String>()
            .parse::<u64>()
            .unwrap();
        let right_stone = stone_string[stone_string.len() / 2..]
            .iter()
            .collect::<String>()
            .parse::<u64>()
            .unwrap();
        vec![left_stone, right_stone]
    } else {
        vec![stone * 2024]
    }
}

fn p2(input: &Vec<u64>) -> usize {
    let mut cache: HashMap<u64, Vec<u64>> = HashMap::new();
    cache.insert(0, vec![1]);

    let mut stones = input.clone();
    for iteration in 0..75 {
        println!("iteration {}", iteration);
        (stones, cache) = blink182(&stones, cache.clone());
    }
    stones.len()
}
