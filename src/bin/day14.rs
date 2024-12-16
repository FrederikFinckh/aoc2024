use std::{env::args, fs::read_to_string};

fn main() {
    let field_size = match args().last() {
        Some(x) if x == "test".to_string() => (11, 7),
        _ => (101, 103),
    };

    let input: Vec<((i64, i64), (i64, i64))> = read_to_string(format!(
        "inputs/day14{}",
        match args().last() {
            Some(x) if x == "test".to_string() => "_test",
            _ => "",
        }
    ))
    .unwrap()
    .split('\n')
    .filter(|x| x.len() > 0)
    .map(|line| line.trim().split_once(' ').unwrap())
    .map(|(position, velocity)| {
        (
            position.split_once('=').unwrap().1,
            velocity.split_once('=').unwrap().1,
        )
    })
    .map(|(position, velocity)| {
        (
            position.split_once(',').unwrap(),
            velocity.split_once(',').unwrap(),
        )
    })
    .map(|((x, y), (v_x, v_y))| {
        (
            (
                i64::from_str_radix(x, 10).unwrap(),
                i64::from_str_radix(y, 10).unwrap(),
            ),
            (
                i64::from_str_radix(v_x, 10).unwrap(),
                i64::from_str_radix(v_y, 10).unwrap(),
            ),
        )
    })
    .collect();
    println!("{}", p1(&input, field_size));
    println!("{}", p2(&input, field_size));
}

fn p1(input: &Vec<((i64, i64), (i64, i64))>, field_size: (isize, isize)) -> u64 {
    println!("{:?}, {:?}", input, field_size);
    1
}

fn p2(input: &Vec<((i64, i64), (i64, i64))>, field_size: (isize, isize)) -> u64 {
    println!("{:?}, {:?}", input, field_size);
    2
}
