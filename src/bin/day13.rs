use std::{env::args, fs::read_to_string};

fn main() {
    let input: Vec<((u64, u64), (u64, u64), (u64, u64))> = read_to_string(format!(
        "inputs/day13{}",
        match args().last() {
            Some(x) if x == "test".to_string() => "_test",
            _ => "",
        }
    ))
    .unwrap()
    .to_string()
    .split("\n\n")
    .filter(|x| x.len() > 0)
    .map(|x| {
        let a_b_target: Vec<(u64, u64)> = x
            .split('\n')
            .filter(|x| x.len() > 0)
            .map(|line| line.split(':').nth(1).unwrap().split_once(',').unwrap())
            .map(|(x, y)| {
                (
                    x.trim()
                        .chars()
                        .skip(2)
                        .collect::<String>()
                        .parse::<u64>()
                        .unwrap(),
                    y.trim()
                        .chars()
                        .skip(2)
                        .collect::<String>()
                        .parse::<u64>()
                        .unwrap(),
                )
            })
            .collect();
        (a_b_target[0], a_b_target[1], a_b_target[2])
    })
    .collect();
    println!("{}", p1(&input));
    println!("{}", p2(&input));
}

fn get_feasible_solutions(a: (u64, u64), b: (u64, u64), target: (u64, u64)) -> Vec<(u64, u64)> {
    let mut brute_force = vec![];

    for alpha in 0..100 {
        for beta in 0..100 {
            if (alpha * a.0 + beta * b.0 == target.0) && alpha * a.1 + beta * b.1 == target.1 {
                brute_force.push((alpha, beta));
            }
        }
    }
    brute_force
}

fn linear_algebra_regular_matrix(
    a: (u64, u64),
    b: (u64, u64),
    target: (u64, u64),
) -> Vec<(u64, u64)> {
    //x_1 * a.0 + x_2 * b.0 = target.0 &&
    //x_1 * a.1 + x_2 * b.1 = target.1
    // iff (suppose [a, b] regular)
    // x_2 = (target.1*a.0-target.0*a.1)/(b.1*1.0-a.1*b.0)
    // x_1 = (target.0 - x_2*b.0)/a.0

    let a_2_2 = (b.1 * a.0) as i64 - (a.1 * b.0) as i64;

    if a_2_2 == 0 {
        panic!("matrix not regular!")
    }

    let b_2 = (target.1 * a.0) as i64 - (target.0 * a.1) as i64;

    let x_2 = b_2 / a_2_2;
    let x_1 = (target.0 as i64 - x_2 * b.0 as i64) / a.0 as i64;

    if b_2 % a_2_2 == 0
        && (target.0 as i64 - x_2 * b.0 as i64) % a.0 as i64 == 0
        && x_1 >= 0
        && x_2 >= 0
    {
        vec![(x_1 as u64, x_2 as u64)]
    } else {
        vec![]
    }
}

fn p1(input: &Vec<((u64, u64), (u64, u64), (u64, u64))>) -> u64 {
    input
        .iter()
        .map(|(a, b, target)| get_feasible_solutions(*a, *b, *target))
        .map(|solutions| {
            solutions
                .into_iter()
                .min_by_key(|(alpha, beta)| 3 * alpha + beta)
        })
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .map(|(a_presses, b_presses)| 3 * a_presses + b_presses)
        .sum()
}

fn p2(input: &Vec<((u64, u64), (u64, u64), (u64, u64))>) -> u64 {
    input
        .iter()
        .map(|(a, b, target)| {
            (
                a,
                b,
                (target.0 + 10_000_000_000_000, target.1 + 10_000_000_000_000),
            )
        })
        .map(|(a, b, target)| linear_algebra_regular_matrix(*a, *b, target))
        .map(|solutions| {
            solutions
                .into_iter()
                .min_by_key(|(alpha, beta)| 3 * alpha + beta)
        })
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .map(|(a_presses, b_presses)| 3 * a_presses + b_presses)
        .sum()
}
