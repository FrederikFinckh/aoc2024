use std::{
    collections::{HashMap, HashSet},
    env::args,
    fs::read_to_string,
};

fn main() {
    let is_test = args().last() == Some("test".to_string());

    let input: Vec<(usize, usize)> = read_to_string(format!(
        "inputs/day18{}",
        if is_test { "_test" } else { "" }
    ))
    .unwrap()
    .split('\n')
    .filter(|x| x.len() > 0)
    .map(|x| x.split_once(',').unwrap())
    .map(|(r, c)| (c.parse::<usize>().unwrap(), r.parse::<usize>().unwrap()))
    .collect();

    let dimensions = if is_test { (7, 7) } else { (71, 71) };

    println!("{}", p1(&input, dimensions, is_test));
    println!("{:?}", p2(&input, dimensions));
}

fn p1(input: &Vec<(usize, usize)>, dimensions: (usize, usize), is_test: bool) -> usize {
    _vizualize_maze(input, dimensions);
    *shortest_path(
        &input[..if is_test { 12 } else { 1024 }].to_vec(),
        dimensions,
    )
    .unwrap()
    .get(&(dimensions.0 - 1, dimensions.1 - 1))
    .unwrap()
}

fn shortest_path(
    input: &Vec<(usize, usize)>,
    dimensions: (usize, usize),
) -> Option<HashMap<(usize, usize), usize>> {
    let start = (0, 0);
    let end = (dimensions.0 - 1, dimensions.1 - 1);

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut cost_by_field: HashMap<(usize, usize), usize> = HashMap::new();

    cost_by_field.insert(start, 0);

    loop {
        if cost_by_field.get(&end).is_some() {
            break;
        }

        let x = cost_by_field
            .iter()
            .filter(|(point, _)| !visited.contains(point))
            .min_by_key(|(_, cost)| *cost)
            .map(|(&p, &c)| (p, c));

        if x.is_none() {
            return None;
        }

        let (point, cost) = x.unwrap();

        visited.insert(point);

        let valid_neighbours = valid_neighbours(point, dimensions, input);

        valid_neighbours.into_iter().for_each(|neighbour| {
            if !cost_by_field
                .get(&neighbour)
                .is_some_and(|neighbour_cost| *neighbour_cost < cost + 1)
            {
                cost_by_field.insert(neighbour, cost + 1);
            }
        });
    }
    Some(cost_by_field)
}

fn valid_neighbours(
    (r, c): (usize, usize),
    dimensions: (usize, usize),
    input: &Vec<(usize, usize)>,
) -> Vec<(usize, usize)> {
    neighbours((r, c), dimensions)
        .into_iter()
        .filter(|p| !input.contains(p))
        .collect()
}

fn neighbours((r, c): (usize, usize), dimensions: (usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbours = vec![];
    if r > 0 {
        neighbours.push((r - 1, c));
    }
    if c > 0 {
        neighbours.push((r, c - 1));
    }
    if r < dimensions.0 - 1 {
        neighbours.push((r + 1, c));
    }
    if c < dimensions.1 - 1 {
        neighbours.push((r, c + 1));
    }

    neighbours
}

fn _vizualize_maze(input: &Vec<(usize, usize)>, dimensions: (usize, usize)) {
    println!();
    for row in 0..dimensions.0 {
        for col in 0..dimensions.1 {
            if input.contains(&(row, col)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn p2(input: &Vec<(usize, usize)>, dimensions: (usize, usize)) -> (usize, usize) {
    // binary search if there is a shortest path
    let mut test_index = input.len() / 2;

    let mut lo = 0;
    let mut hi = input.len();

    loop {
        if lo > hi - 2 {
            println!(
                "something something binary search successful, lo: {}, hi {}",
                lo, hi
            );
            break;
        }
        match shortest_path(&input[..test_index].to_vec(), dimensions) {
            Some(_) => {
                lo = test_index;
                test_index += (hi - test_index) / 2;
            }
            None => {
                hi = test_index;
                test_index -= (test_index - lo) / 2;
            }
        }
    }

    (input[lo].1, input[lo].0)
}
