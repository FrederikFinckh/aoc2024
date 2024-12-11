use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let input: Vec<String> = read_to_string("inputs/day6")
        .unwrap()
        .to_string()
        .split('\n')
        .filter(|x| x.len() > 0)
        .map(|x| x.to_string())
        .collect();
    let starting_position = (0..input.len())
        .map(|row| (row, input[row].find('^')))
        .filter(|(_, col)| col.is_some())
        .map(|(row, col)| (row, col.unwrap()))
        .nth(0)
        .unwrap();
    let obstacles: Vec<(usize, usize)> = (0..input.len())
        .flat_map(|row| {
            input[row]
                .match_indices('#')
                .map(|(col, _)| (row, col))
                .collect::<Vec<(usize, usize)>>()
        })
        .collect();
    println!("{}", p1(&input, &starting_position, &obstacles));
    println!("{}", p2(&input, &starting_position, &obstacles));
}

fn p1(
    input: &Vec<String>,
    starting_position: &(usize, usize),
    obstacles: &Vec<(usize, usize)>,
) -> u64 {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut position = starting_position.clone();

    let mut direction = 0;
    // direction
    // 0 -> N
    // 1 -> E
    // 2 -> S
    // 3 -> W

    loop {
        let obstacle = find_obstacle(position, direction, &obstacles);
        if obstacle.is_none() {
            for field in
                get_fields_until_boundary(position, direction, (input.len(), input[0].len()))
            {
                visited.insert(field);
            }
            break;
        } else {
            let obstacle = obstacle.unwrap();
            for field in get_fields(position, direction, *obstacle) {
                visited.insert(field);
            }
            position = match direction {
                0 => (obstacle.0 + 1, position.1),
                1 => (position.0, obstacle.1 - 1),
                2 => (obstacle.0 - 1, position.1),
                3 => (position.0, obstacle.1 + 1),
                _ => panic!("invalid direction!"),
            };

            direction = next_direction(direction);
        }
    }

    // _visualize(input, &visited, obstacles);

    visited.len() as u64
}

fn _visualize(
    input: &Vec<String>,
    visited: &HashSet<(usize, usize)>,
    obstacles: Vec<(usize, usize)>,
) {
    //visualize
    println!();
    println!();
    for row in 0..input.len() {
        for col in 0..input[0].len() {
            if visited.contains(&(row, col)) {
                print!("X");
            } else if obstacles.contains(&(row, col)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!("{:?}", visited);
}

fn get_fields(
    position: (usize, usize),
    direction: u8,
    obstacle: (usize, usize),
) -> Vec<(usize, usize)> {
    match direction {
        0 => (obstacle.0 + 1..=position.0)
            .map(|x| (x, position.1))
            .collect(),
        1 => (position.1..obstacle.1).map(|x| (position.0, x)).collect(),
        2 => (position.0..obstacle.0).map(|x| (x, position.1)).collect(),
        3 => (obstacle.1 + 1..=position.1)
            .map(|x| (position.0, x))
            .collect(),
        _ => panic!("invalid direction!"),
    }
}

fn get_fields_until_boundary(
    position: (usize, usize),
    direction: u8,
    dimensions: (usize, usize),
) -> Vec<(usize, usize)> {
    match direction {
        0 => (0..=position.0).map(|x| (x, position.1)).collect(),
        1 => (position.1..dimensions.1)
            .map(|x| (position.0, x))
            .collect(),
        2 => (position.0..dimensions.0)
            .map(|x| (x, position.1))
            .collect(),
        3 => (0..=position.1).map(|x| (position.0, x)).collect(),
        _ => panic!("invalid direction!"),
    }
}

fn next_direction(direction: u8) -> u8 {
    (direction + 1) % 4
}

fn find_obstacle(
    position: (usize, usize),
    direction: u8,
    obstacles: &Vec<(usize, usize)>,
) -> Option<&(usize, usize)> {
    match direction {
        0 => obstacles
            .iter()
            .filter(|obstacle| obstacle.1 == position.1)
            .filter(|obstacle| obstacle.0 < position.0)
            .max_by_key(|obstacle| obstacle.0),
        1 => obstacles
            .iter()
            .filter(|obstacle| obstacle.0 == position.0)
            .filter(|obstacle| obstacle.1 > position.1)
            .min_by_key(|obstacle| obstacle.1),
        2 => obstacles
            .iter()
            .filter(|obstacle| obstacle.1 == position.1)
            .filter(|obstacle| obstacle.0 > position.0)
            .min_by_key(|obstacle| obstacle.0),
        3 => obstacles
            .iter()
            .filter(|obstacle| obstacle.0 == position.0)
            .filter(|obstacle| obstacle.1 < position.1)
            .max_by_key(|obstacle| obstacle.1),
        _ => panic!("invalid direction!"),
    }
}

fn p2(
    input: &Vec<String>,
    starting_position: &(usize, usize),
    obstacles: &Vec<(usize, usize)>,
) -> u64 {
    let mut possible_loops = 0;
    for row in 0..input.len() {
        println!("{}", row);
        for col in 0..input[0].len() {
            if (row == starting_position.0 && col == starting_position.1)
                || obstacles.contains(&(row, col))
            {
                continue;
            }

            let new_obstacle = (row, col);
            let mut new_obstacles = obstacles.clone();
            new_obstacles.push(new_obstacle);

            let mut hit_obstacle_from_direction: HashSet<((usize, usize), u8)> = HashSet::new();
            let mut position = starting_position.clone();

            let mut direction = 0;
            // let mut visited: HashSet<(usize, usize)> = HashSet::new();

            if loop {
                let obstacle = find_obstacle(position, direction, &new_obstacles);
                if obstacle.is_none() {
                    break false;
                } else {
                    let obstacle = obstacle.unwrap();
                    if hit_obstacle_from_direction.contains(&(*obstacle, direction)) {
                        break true;
                    }
                    // for field in get_fields(position, direction, *obstacle) {
                    //     visited.insert(field);
                    // }
                    hit_obstacle_from_direction.insert((*obstacle, direction));
                    position = match direction {
                        0 => (obstacle.0 + 1, position.1),
                        1 => (position.0, obstacle.1 - 1),
                        2 => (obstacle.0 - 1, position.1),
                        3 => (position.0, obstacle.1 + 1),
                        _ => panic!("invalid direction!"),
                    };

                    direction = next_direction(direction);
                }
            } {
                possible_loops += 1;
                // _visualize(input, &visited, new_obstacles);
            }
        }
    }
    possible_loops
}
