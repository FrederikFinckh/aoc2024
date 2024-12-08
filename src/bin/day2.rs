use std::fs::read_to_string;

fn main() {
    let input: Vec<Vec<i64>> = read_to_string("inputs/day2")
        .unwrap()
        .split('\n')
        .filter(|l| l.len() > 0)
        .map(|l| {
            l.split(' ')
                .map(|n| n.trim().parse::<i64>().unwrap())
                .collect()
        })
        .collect();
    p1(&input);
    p2(input);
}

fn p1(input: &Vec<Vec<i64>>) {
    let safe_rows = input.iter().filter(|levels| is_safe(levels)).count();
    println!("{}", safe_rows);
}

fn is_safe(levels: &Vec<i64>) -> bool {
    is_safe_increasing(levels) || is_safe_decreasing(levels)
}

fn is_safe_increasing(levels: &Vec<i64>) -> bool {
    if let Some(mut prev) = levels.get(0) {
        for level in levels.iter().skip(1) {
            if !is_safe_increase(level, prev) {
                return false;
            }
            prev = level;
        }
    }
    true
}

fn is_safe_increase(level: &i64, prev: &i64) -> bool {
    level > prev && *level <= prev + 3
}

fn is_safe_decreasing(levels: &Vec<i64>) -> bool {
    if let Some(mut prev) = levels.get(0) {
        for level in levels.iter().skip(1) {
            if !is_safe_decrese(level, prev) {
                return false;
            }
            prev = level;
        }
    }
    true
}

fn is_safe_decrese(level: &i64, prev: &i64) -> bool {
    level < prev && *level >= prev - 3
}

fn p2(input: Vec<Vec<i64>>) {
    let safe_rows = input
        .iter()
        .filter(|levels| is_safe_with_dampening(levels))
        .count();
    println!("{}", safe_rows);
}

fn is_safe_with_dampening(levels: &Vec<i64>) -> bool {
    if let Some(_first) = levels.get(0) {
        let mut safe_increasing = true;

        for index in 1..levels.len() {
            if !is_safe_increase(&levels[index], &levels[index - 1]) {
                let mut levels_without_current = levels[..index].to_vec();
                levels_without_current.append(&mut levels[index + 1..].to_vec());

                let mut levels_without_prev = levels[..index - 1].to_vec();
                levels_without_prev.append(&mut levels[index..].to_vec());

                safe_increasing = is_safe_increasing(&levels_without_current)
                    || is_safe_increasing(&levels_without_prev);
                break;
            }
        }
        if !safe_increasing {
            for index in 1..levels.len() {
                if !is_safe_decrese(&levels[index], &levels[index - 1]) {
                    let mut levels_without_current = levels[..index].to_vec();
                    levels_without_current.append(&mut levels[index + 1..].to_vec());

                    let mut levels_without_prev = levels[..index - 1].to_vec();
                    levels_without_prev.append(&mut levels[index..].to_vec());

                    return is_safe_decreasing(&levels_without_current)
                        || is_safe_decreasing(&levels_without_prev);
                }
            }
        }
    }
    true
}
