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

fn blink182(amount_of_stones_by_value: HashMap<u64, usize>) -> HashMap<u64, usize> {
    let mut new_amounts_by_value = HashMap::new();
    for (stone, amount) in amount_of_stones_by_value {
        let resulting_stones = update_single_stone(stone);
        for new_stone in resulting_stones {
            match new_amounts_by_value.get(&new_stone) {
                None => {
                    new_amounts_by_value.insert(new_stone, amount);
                }
                Some(old_amount) => {
                    new_amounts_by_value.insert(new_stone, old_amount + amount);
                }
            }
        }
    }
    new_amounts_by_value
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
    let mut amount_of_stones_by_value: HashMap<u64, usize> = HashMap::new();
    for stone in input {
        match amount_of_stones_by_value.get(stone) {
            None => {
                amount_of_stones_by_value.insert(*stone, 1);
            }
            Some(amount) => {
                amount_of_stones_by_value.insert(*stone, amount + 1);
            }
        }
    }

    for _iteration in 0..75 {
        amount_of_stones_by_value = blink182(amount_of_stones_by_value);
    }
    amount_of_stones_by_value.values().sum()
}
