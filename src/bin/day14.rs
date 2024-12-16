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

fn p1(input: &Vec<((i64, i64), (i64, i64))>, field_size: (i64, i64)) -> u64 {
    let seconds = 100;
    let positions_after_seconds = update_positions(input, field_size, seconds);
    println!("{:?}", positions_after_seconds);
    visualize(field_size, positions_after_seconds);
    1
}

fn update_positions(
    input: &Vec<((i64, i64), (i64, i64))>,
    field_size: (i64, i64),
    seconds: i64,
) -> Vec<(i64, i64)> {
    let positions_after_seconds: Vec<(i64, i64)> = input
        .iter()
        .map(|(position, velocity)| {
            (
                (position.0 + seconds * velocity.0)
                    - field_size.0 * ((position.0 + seconds * velocity.0) / field_size.0),
                (position.1 + seconds * velocity.1)
                    - field_size.1 * ((position.1 + seconds * velocity.1) / field_size.1),
            )
        })
        .map(|(x, y)| {
            (
                if x >= 0 { x } else { x + field_size.0 },
                if y >= 0 { y } else { y + field_size.1 },
            )
        })
        .collect();
    positions_after_seconds
}

fn visualize(field_size: (i64, i64), positions_after_100s: Vec<(i64, i64)>) {
    println!();
    for y in 0..field_size.1 {
        for x in 0..field_size.0 {
            if positions_after_100s.contains(&(x, y)) {
                print!("X");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn p2(input: &Vec<((i64, i64), (i64, i64))>, field_size: (i64, i64)) -> u64 {
    println!("{:?}, {:?}", input, field_size);
    2
}
