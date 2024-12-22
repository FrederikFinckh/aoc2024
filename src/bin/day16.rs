use std::{
    collections::{HashMap, HashSet},
    env::args,
    fs::read_to_string,
    // io::stdin,
};

fn main() {
    let input: Vec<Vec<char>> = read_to_string(format!(
        "inputs/day16{}",
        match args().last() {
            Some(x) if x == "test".to_string() => "_test",
            Some(x) if x == "test2".to_string() => "_test2",
            _ => "",
        }
    ))
    .unwrap()
    .split('\n')
    .filter(|x| x.len() > 0)
    .map(|x| x.chars().collect())
    .collect();

    let start_position = (0..input.len())
        .filter_map(|row| {
            (0..input[row].len())
                .filter(|col| input[row][*col] == 'S')
                .nth(0)
                .map(|col| (row, col))
        })
        .nth(0)
        .map(|(r, c)| (r, c, 'E'))
        .unwrap();
    let target_position1 = (0..input.len())
        .filter_map(|row| {
            (0..input[row].len())
                .filter(|col| input[row][*col] == 'E')
                .nth(0)
                .map(|col| (row, col))
        })
        .nth(0)
        .map(|(r, c)| (r, c, 'E'))
        .unwrap();
    let target_position2 = (target_position1.0, target_position1.1, 'N');
    let target_positions = vec![target_position1, target_position2];
    let cost_by_point = dijkstra(&input, start_position, &target_positions);

    println!("{}", p1(&cost_by_point, &target_positions));
    let points = calculate_points_on_best_paths(&cost_by_point, &target_positions);
    println!();
    print!(" ");
    for col in 0..input[0].len() {
        print!("{}", col % 10);
    }
    println!();
    for row in 0..input.len() {
        print!("{}", row % 10);
        for col in 0..input[row].len() {
            if points.contains(&(row, col)) {
                print!("O");
            } else {
                print!(".")
            }
        }
        println!()
    }
    println!();

    //p2

    println!("{}", points.len() + 1);
}

fn dijkstra(
    input: &Vec<Vec<char>>,
    start_position: (usize, usize, char),
    target_positions: &Vec<(usize, usize, char)>,
) -> HashMap<(usize, usize, char), (u64, Vec<(usize, usize, char)>)> {
    let mut cost_by_position: HashMap<(usize, usize, char), (u64, Vec<(usize, usize, char)>)> =
        HashMap::new();
    let mut visited: HashSet<(usize, usize, char)> = HashSet::new();
    cost_by_position.insert(start_position, (0, vec![]));
    let mut loop_var = 1;
    loop {
        if cost_by_position.get(&target_positions[0]).is_some()
            || cost_by_position.get(&target_positions[1]).is_some()
        {
            break;
        }
        let current_point = cost_by_position
            .iter()
            .filter(|(x, __)| !visited.contains(x))
            .min_by_key(|(_, cost)| cost.0)
            .map(|(&point, cost_and_paths)| (point, cost_and_paths.clone()))
            .unwrap()
            .clone();
        visited.insert(current_point.0);
        let neighbours = get_neighbours(input, current_point.0);
        for neighbour in neighbours {
            let maybe_entry = cost_by_position.get(&neighbour.0);

            if maybe_entry.is_none()
                || maybe_entry.is_some_and(|(cost, _)| *cost > current_point.1 .0 + neighbour.1)
            {
                cost_by_position.insert(
                    neighbour.0,
                    (current_point.1 .0 + neighbour.1, vec![current_point.0]),
                );
            } else if maybe_entry.is_some_and(|(cost, _)| *cost == current_point.1 .0 + neighbour.1)
            {
                let mut possible_predecessors = maybe_entry.unwrap().1.clone();
                possible_predecessors.push(current_point.0);
                cost_by_position.insert(
                    neighbour.0,
                    (current_point.1 .0 + neighbour.1, possible_predecessors),
                );
            }
        }
        // _debug(input, &cost_by_position, current_point);
        // let mut buffer = String::new();
        // stdin().read_line(&mut buffer).unwrap();
        if loop_var % 1000 == 0 {
            println!("{}", loop_var);
        }
        loop_var += 1;
    }
    cost_by_position
}

fn _debug(
    input: &Vec<Vec<char>>,
    cost_by_position: &HashMap<(usize, usize, char), (u64, Vec<(usize, usize, char)>)>,
    current_point: ((usize, usize, char), (u64, Vec<(usize, usize, char)>)),
) {
    println!("just visited {:?}", current_point);
    for entry in cost_by_position {
        println!("{:?}", entry);
    }
    println!();
    for row in 0..input.len() {
        for col in 0..input[row].len() {
            if input[row][col] == '#' {
                print!("##### ")
            } else {
                print!(
                    "{}",
                    cost_by_position
                        .get(&(row, col, 'N'))
                        .map(|x| format!("N{:4} ", x.0))
                        .unwrap_or("N.... ".to_string())
                );
            }
        }
        println!();
        for col in 0..input[row].len() {
            if input[row][col] == '#' {
                print!("##### ")
            } else {
                print!(
                    "{}",
                    cost_by_position
                        .get(&(row, col, 'E'))
                        .map(|x| format!("E{:4} ", x.0))
                        .unwrap_or("E.... ".to_string())
                );
            }
        }
        println!();
        for col in 0..input[row].len() {
            if input[row][col] == '#' {
                print!("##### ")
            } else {
                print!(
                    "{}",
                    cost_by_position
                        .get(&(row, col, 'S'))
                        .map(|x| format!("S{:4} ", x.0))
                        .unwrap_or("S.... ".to_string())
                );
            }
        }
        println!();
        for col in 0..input[row].len() {
            if input[row][col] == '#' {
                print!("##### ")
            } else {
                print!(
                    "{}",
                    cost_by_position
                        .get(&(row, col, 'W'))
                        .map(|x| format!("W{:4} ", x.0))
                        .unwrap_or("W.... ".to_string())
                );
            }
        }
        println!();

        println!();
    }
    println!("current point {:?}", current_point.0);
    println!("{:?}", current_point.1 .1);
}

fn get_neighbours(
    input: &Vec<Vec<char>>,
    (row, col, direction): (usize, usize, char),
) -> Vec<((usize, usize, char), u64)> {
    match direction {
        'N' => vec![
            ((row - 1, col, 'N'), 1),
            ((row, col, 'E'), 1000),
            ((row, col, 'W'), 1000),
        ],
        'E' => vec![
            ((row, col + 1, 'E'), 1),
            ((row, col, 'S'), 1000),
            ((row, col, 'N'), 1000),
        ],
        'S' => vec![
            ((row + 1, col, 'S'), 1),
            ((row, col, 'E'), 1000),
            ((row, col, 'W'), 1000),
        ],
        'W' => vec![
            ((row, col - 1, 'W'), 1),
            ((row, col, 'S'), 1000),
            ((row, col, 'N'), 1000),
        ],
        x => panic!("invalid direction {}", x),
    }
    .into_iter()
    .filter(|((r, c, _), _)| input[*r][*c] != '#')
    .collect()
}

fn p1(
    cost_by_point: &HashMap<(usize, usize, char), (u64, Vec<(usize, usize, char)>)>,
    target_positions: &Vec<(usize, usize, char)>,
) -> u64 {
    cost_by_point
        .get(&target_positions[0])
        .map(|x| x.0)
        .unwrap_or_else(|| {
            cost_by_point
                .get(&target_positions[1])
                .map(|x| x.0)
                .unwrap()
        })
}

fn calculate_points_on_best_paths(
    cost_by_point: &HashMap<(usize, usize, char), (u64, Vec<(usize, usize, char)>)>,
    target_positions: &Vec<(usize, usize, char)>,
) -> HashSet<(usize, usize)> {
    let mut new_points: Vec<(usize, usize, char)> = cost_by_point
        .get(&target_positions[0])
        .map(|x| x.1.clone())
        .unwrap_or_else(|| {
            cost_by_point
                .get(&target_positions[1])
                .map(|x| x.1.clone())
                .unwrap()
        });

    let mut points = HashSet::new();

    loop {
        for x in new_points.clone() {
            points.insert(x);
        }
        new_points = new_points
            .into_iter()
            .flat_map(|x| cost_by_point.get(&x).unwrap().1.clone())
            .collect();
        if new_points.is_empty() {
            break;
        }
    }

    points.iter().map(|x| (x.0, x.1)).collect()
}
