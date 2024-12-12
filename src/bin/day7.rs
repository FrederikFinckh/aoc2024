use std::fs::read_to_string;

fn main() {
    let input: Vec<(i64, Vec<i64>)> = read_to_string("inputs/day7")
        .unwrap()
        .to_string()
        .split('\n')
        .filter(|x| x.len() > 0)
        .map(|line| line.split_once(':').unwrap())
        .map(|(x, y)| {
            (
                x.parse::<i64>().unwrap(),
                y.split_ascii_whitespace()
                    .map(|x| x.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>(),
            )
        })
        .collect();
    println!("{}", p1(&input));
    println!("{}", p2(&input));
}

fn p1(input: &Vec<(i64, Vec<i64>)>) -> i64 {
    input
        .iter()
        .filter(|(test_sum, nums)| is_satisfiable(*test_sum, nums))
        .map(|(sum, _)| sum)
        .sum()
}

fn is_satisfiable(test_sum: i64, nums: &Vec<i64>) -> bool {
    let mut mask: Vec<bool> = vec![false; nums.len() - 1];
    loop {
        if get_value(nums, &mask) == test_sum {
            break true;
        }
        if !mask.contains(&false) {
            break false;
        }
        mask = tick_up(mask);
    }
}

fn get_value(nums: &Vec<i64>, mask: &Vec<bool>) -> i64 {
    let mut value = nums[0];
    for index in 0..mask.len() {
        if mask[index] {
            value += nums[index + 1];
        } else {
            value *= nums[index + 1];
        }
    }
    value
}

fn tick_up(mut mask: Vec<bool>) -> Vec<bool> {
    for x in (0..mask.len()).rev() {
        mask[x] = !mask[x];
        if mask[x] {
            break;
        }
    }
    mask
}

fn p2(input: &Vec<(i64, Vec<i64>)>) -> i64 {
    input
        .iter()
        .filter(|(test_sum, nums)| is_satisfiable2(*test_sum, nums))
        .map(|(sum, _)| sum)
        .sum()
}

fn is_satisfiable2(test_sum: i64, nums: &Vec<i64>) -> bool {
    println!("checking {:?}", nums);
    let mut mask: Vec<u8> = vec![0; nums.len() - 1];
    loop {
        if get_value2(nums, &mask) == test_sum {
            break true;
        }
        if !mask.contains(&0) && !mask.contains(&1) {
            break false;
        }
        mask = tick_up2(mask);
    }
}

fn get_value2(nums: &Vec<i64>, mask: &Vec<u8>) -> i64 {
    let mut value = nums[0];
    for index in 0..mask.len() {
        match mask[index] {
            0 => {
                value += nums[index + 1];
            }
            1 => {
                value *= nums[index + 1];
            }
            2 => {
                let mut x = value.to_string();
                x.push_str(&mut nums[index + 1].to_string());
                value = x.parse::<i64>().unwrap();
            }
            x => panic!("invalid operator {}", x),
        }
    }
    value
}

fn tick_up2(mut mask: Vec<u8>) -> Vec<u8> {
    for x in (0..mask.len()).rev() {
        mask[x] = (mask[x] + 1) % 3;
        if mask[x] > 0 {
            break;
        }
    }
    mask
}
