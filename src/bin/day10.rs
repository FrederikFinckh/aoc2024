use std::{collections::HashSet, env::args, fs::read_to_string};

fn main() {
    let input: Vec<String> = read_to_string(format!(
        "inputs/day10{}",
        match args().last() {
            Some(x) if x == "test".to_string() => "_test",
            _ => "",
        }
    ))
    .unwrap()
    .to_string()
    .split('\n')
    .filter(|x| x.len() > 0)
    .map(|x| x.to_string())
    .collect();

    println!("{}", p1(&input));
    println!("{}", p2(&input));
}

fn neighbours(point: (usize, usize), dimensions: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbours = vec![];
    if let Some(point) = up(point, dimensions) {
        neighbours.push(point);
    }
    if let Some(point) = right(point, dimensions) {
        neighbours.push(point);
    }
    if let Some(point) = down(point, dimensions) {
        neighbours.push(point);
    }
    if let Some(point) = left(point, dimensions) {
        neighbours.push(point);
    }
    neighbours
}

fn up(point: (usize, usize), dimensions: &(usize, usize)) -> Option<(usize, usize)> {
    if !in_bounds(
        (point.0 as isize - 1, point.1 as isize),
        &(dimensions.0 as isize, dimensions.1 as isize),
    ) {
        None
    } else {
        Some((point.0 - 1, point.1))
    }
}

fn right(point: (usize, usize), dimensions: &(usize, usize)) -> Option<(usize, usize)> {
    if !in_bounds(
        (point.0 as isize, point.1 as isize + 1),
        &(dimensions.0 as isize, dimensions.1 as isize),
    ) {
        None
    } else {
        Some((point.0, point.1 + 1))
    }
}

fn down(point: (usize, usize), dimensions: &(usize, usize)) -> Option<(usize, usize)> {
    if !in_bounds(
        (point.0 as isize + 1, point.1 as isize),
        &(dimensions.0 as isize, dimensions.1 as isize),
    ) {
        None
    } else {
        Some((point.0 + 1, point.1))
    }
}

fn left(point: (usize, usize), dimensions: &(usize, usize)) -> Option<(usize, usize)> {
    if !in_bounds(
        (point.0 as isize, point.1 as isize - 1),
        &(dimensions.0 as isize, dimensions.1 as isize),
    ) {
        None
    } else {
        Some((point.0, point.1 - 1))
    }
}

fn in_bounds(point: (isize, isize), dimensions: &(isize, isize)) -> bool {
    point.0 >= 0 && point.1 >= 0 && point.0 < dimensions.0 && point.1 < dimensions.1
}

fn get_reachable_hilltops(point: (usize, usize), input: &[String]) -> HashSet<(usize, usize)> {
    let dimensions = (input[0].len(), input[0].len());
    let mut current_neighbours: HashSet<(usize, usize)> = neighbours(point, &dimensions)
        .iter()
        .filter(|(row, col)| input[*row].chars().nth(*col).unwrap() == '1')
        .map(|x| *x)
        .collect();
    for height in 2..=9 {
        current_neighbours = current_neighbours
            .iter()
            .flat_map(|point| neighbours(*point, &dimensions))
            .filter(|(row, col)| input[*row].chars().nth(*col) == height.to_string().chars().nth(0))
            .collect();
    }
    current_neighbours
}

fn p1(input: &Vec<String>) -> usize {
    let mut sum = 0;
    for row in 0..input.len() {
        for col in 0..input[0].len() {
            if input[row].chars().nth(col).unwrap() == '0' {
                sum += get_reachable_hilltops((row, col), input).len();
            }
        }
    }
    sum
}

fn get_distinct_paths(point: (usize, usize), input: &[String]) -> HashSet<Vec<(usize, usize)>> {
    let dimensions = (input[0].len(), input[0].len());

    let mut current_paths: HashSet<Vec<(usize, usize)>> = HashSet::new();

    current_paths.insert(vec![point]);

    for height in 1..=9 {
        for path in &current_paths.clone() {
            let current_point = path.last().unwrap().clone();
            current_paths.remove(path);
            let paths: HashSet<Vec<(usize, usize)>> = neighbours(current_point, &dimensions)
                .iter()
                .filter(|(row, col)| {
                    input[*row].chars().nth(*col) == height.to_string().chars().nth(0)
                })
                .map(|next_pos| {
                    let mut continuation = path.clone();
                    continuation.push(*next_pos);
                    continuation
                })
                .collect();

            for path in paths {
                current_paths.insert(path);
            }
        }
    }

    current_paths
}

fn p2(input: &Vec<String>) -> usize {
    let mut sum = 0;
    for row in 0..input.len() {
        for col in 0..input[0].len() {
            if input[row].chars().nth(col).unwrap() == '0' {
                sum += get_distinct_paths((row, col), input).len();
            }
        }
    }
    sum
}
