use core::time;
use std::{collections::HashSet, env::args, fs::read_to_string, thread::sleep};

fn main() {
    let read_to_string = read_to_string(format!(
        "inputs/day15{}",
        match args().last() {
            Some(x) if x == "test".to_string() => "_test",
            _ => "",
        }
    ))
    .unwrap();
    let (maze, moves): (&str, &str) = read_to_string.split_once("\n\n").unwrap();

    let maze: Vec<String> = maze
        .split('\n')
        .filter(|x| x.len() > 0)
        .map(|x| x.to_string())
        .collect();

    let moves: Vec<char> = moves.chars().filter(|c| !c.is_ascii_whitespace()).collect();

    println!("{}", p1(&maze, &moves));
    println!("{}", p2(&maze, &moves));
}

fn p1(maze: &Vec<String>, moves: &Vec<char>) -> usize {
    let is_test = args().last().is_some_and(|arg| arg.eq("test"));
    let mut current_maze = maze.clone();
    let mut loop_var = 0;
    println!("there are {} steps to take", moves.len());
    for direction in moves {
        current_maze = update_maze(current_maze, direction);
        if is_test {
            print_maze(&current_maze);
            println!("{direction}");
            sleep(time::Duration::from_millis(10));
        }
        loop_var += 1;
        if loop_var % 100 == 0 {
            println!("{}", loop_var);
        }
    }
    score(&current_maze)
}

fn score(maze: &Vec<String>) -> usize {
    (0..maze.len())
        .map(|row| {
            (0..maze[0].len())
                .map(|col| {
                    if maze[row].chars().nth(col) == Some('O') {
                        100 * row + col
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum()
}

//consume maze here
fn update_maze(maze: Vec<String>, direction: &char) -> Vec<String> {
    let position = (0..maze.len())
        .filter_map(|row| {
            (0..maze[row].len())
                .filter(|col| maze[row].chars().nth(*col) == Some('@'))
                .map(|col| (row, col))
                .nth(0)
        })
        .nth(0)
        .unwrap();
    match direction {
        '>' => push_right(position, maze),
        '<' => push_left(position, maze),
        'v' => push_down(position, maze),
        '^' => push_up(position, maze),
        x => panic!("invalid direction {}", x),
    }
}

fn push_right(position: (usize, usize), maze: Vec<String>) -> Vec<String> {
    let row = &maze[position.0];
    let next_wall = row
        .match_indices('#')
        .filter(|(index, _)| *index > position.1)
        .map(|x| x.0)
        .nth(0)
        .unwrap();

    let next_space = row
        .match_indices('.')
        .filter(|(index, _)| *index > position.1 && *index < next_wall)
        .map(|x| x.0)
        .nth(0);

    if next_space.is_none() {
        return maze;
    }
    let next_space = next_space.unwrap();

    let stones_to_shift: String = row
        .match_indices('O')
        .filter(|(index, _)| *index > position.1 && *index < next_space)
        .map(|_| 'O')
        .collect();
    let new_row = format!(
        "{}.@{}{}",
        row[..position.1].to_string(),
        stones_to_shift,
        row[next_space + 1..].to_string()
    );

    (0..maze.len())
        .map(|r| {
            if r != position.0 {
                maze[r].clone()
            } else {
                new_row.clone()
            }
        })
        .collect()
}

fn push_left((row, col): (usize, usize), maze: Vec<String>) -> Vec<String> {
    flip(push_right((row, maze[0].len() - 1 - col), flip(maze)))
}

fn push_down((row, col): (usize, usize), maze: Vec<String>) -> Vec<String> {
    transpose(push_right((col, row), transpose(maze)))
}

fn push_up((row, col): (usize, usize), maze: Vec<String>) -> Vec<String> {
    transpose(flip(push_right(
        (col, maze[0].len() - 1 - row),
        flip(transpose(maze)),
    )))
}

fn transpose(maze: Vec<String>) -> Vec<String> {
    (0..maze[0].len())
        .map(|col| {
            (0..maze.len())
                .map(|row| maze[row].chars().nth(col).unwrap())
                .collect()
        })
        .collect()
}

fn flip(maze: Vec<String>) -> Vec<String> {
    (0..maze.len())
        .map(|row| {
            (0..maze[row].len())
                .rev()
                .map(|col| maze[row].chars().nth(col).unwrap())
                .collect()
        })
        .collect()
}

fn print_maze(maze: &Vec<String>) {
    for line in maze {
        println!("{}", line);
    }
}

fn p2(maze: &Vec<String>, moves: &Vec<char>) -> usize {
    let is_test = args().last().is_some_and(|arg| arg.eq("test"));
    let _ = moves;
    let mut fat_maze = make_fat(maze);

    if is_test {
        print_maze(&fat_maze);
    }

    let mut loop_var = 0;
    println!("there are {} steps to take", moves.len());
    for direction in moves {
        update_fat_maze(&mut fat_maze, direction);
        if is_test {
            println!("{}", loop_var);
            print_maze(&fat_maze);
            println!("{direction}");
            sleep(time::Duration::from_millis(10));
        }
        loop_var += 1;
        if loop_var % 100 == 0 {
            println!("{}", loop_var);
        }
    }
    score_fat(&fat_maze)
}

fn score_fat(maze: &Vec<String>) -> usize {
    (0..maze.len())
        .map(|row| {
            (0..maze[0].len())
                .map(|col| {
                    if maze[row].chars().nth(col) == Some('[') {
                        100 * row + col
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum()
}

//mutate maze here, probably performance is better that way
fn update_fat_maze(mut maze: &mut Vec<String>, direction: &char) {
    let position = (0..maze.len())
        .filter_map(|row| {
            (0..maze[row].len())
                .filter(|col| maze[row].chars().nth(*col) == Some('@'))
                .map(|col| (row, col))
                .nth(0)
        })
        .nth(0)
        .unwrap();
    match direction {
        '>' => push_fat_right(position, &mut maze),
        '<' => push_fat_left(position, &mut maze),
        '^' => push_fat_up(position, &mut maze),
        'v' => push_fat_down(position, &mut maze),
        x => panic!("invalid direction {}", x),
    };
}

fn push_fat_right(position: (usize, usize), maze: &mut Vec<String>) {
    let row = &maze[position.0];
    let next_wall = row
        .match_indices('#')
        .filter(|(index, _)| *index > position.1)
        .map(|x| x.0)
        .nth(0)
        .unwrap();

    let next_space = row
        .match_indices('.')
        .filter(|(index, _)| *index > position.1 && *index < next_wall)
        .map(|x| x.0)
        .nth(0);

    if next_space.is_none() {
        return;
    }
    let next_space = next_space.unwrap();

    let stones_to_shift: String = row
        .match_indices(|x| x == '[' || x == ']')
        .filter(|(index, _)| *index > position.1 && *index < next_space)
        .map(|(_, x)| x)
        .collect();
    let new_row = format!(
        "{}.@{}{}",
        row[..position.1].to_string(),
        stones_to_shift,
        row[next_space + 1..].to_string()
    );

    maze[position.0] = new_row;
}

fn push_fat_left(position: (usize, usize), maze: &mut Vec<String>) {
    let row = &maze[position.0];
    let next_wall = row
        .match_indices('#')
        .filter(|(index, _)| *index < position.1)
        .map(|x| x.0)
        .last()
        .unwrap();

    let next_space = row
        .match_indices('.')
        .filter(|(index, _)| *index < position.1 && *index > next_wall)
        .map(|x| x.0)
        .last();

    if next_space.is_none() {
        return;
    }
    let next_space = next_space.unwrap();

    let stones_to_shift: String = row
        .match_indices(|x| x == '[' || x == ']')
        .filter(|(index, _)| *index < position.1 && *index > next_space)
        .map(|(_, x)| x)
        .collect();
    let new_row = format!(
        "{}{}@.{}",
        row[..next_space].to_string(),
        stones_to_shift,
        row[position.1 + 1..].to_string()
    );

    maze[position.0] = new_row;
}

fn push_fat_up((row, col): (usize, usize), maze: &mut Vec<String>) {
    let mut space_to_check = vec![(row - 1, col)];
    let mut coords_to_move_up: HashSet<(usize, usize)> = HashSet::new();
    coords_to_move_up.insert((row, col));
    loop {
        let mut next_places_to_check = vec![];
        for (r, c) in space_to_check {
            let thing = maze[r].chars().nth(c);
            if thing == Some('#') {
                // hit wall, there is nothing to do
                return;
            }
            if thing == Some('[') {
                coords_to_move_up.insert((r, c));
                coords_to_move_up.insert((r, c + 1));
                next_places_to_check.push((r - 1, c));
                next_places_to_check.push((r - 1, c + 1));
            }
            if thing == Some(']') {
                coords_to_move_up.insert((r, c - 1));
                coords_to_move_up.insert((r, c));
                next_places_to_check.push((r - 1, c - 1));
                next_places_to_check.push((r - 1, c));
            }
        }
        if next_places_to_check.is_empty() {
            break;
        }
        space_to_check = next_places_to_check;
    }
    // if we're here, we can push up!
    let min_height = coords_to_move_up.iter().min_by_key(|(r, _)| r).unwrap().0;
    for row_to_push in min_height..=row {
        let mut prev_row: Vec<char> = maze[row_to_push - 1].chars().collect();
        let mut current_row: Vec<char> = maze[row_to_push].chars().collect();
        let things_to_move_up: Vec<&(usize, usize)> = coords_to_move_up
            .iter()
            .filter(|(r, _)| *r == row_to_push)
            .collect();

        for thing in things_to_move_up {
            let tmp = prev_row[thing.1];
            prev_row[thing.1] = current_row[thing.1];
            current_row[thing.1] = tmp;
        }
        maze[row_to_push - 1] = prev_row.iter().collect();
        maze[row_to_push] = current_row.iter().collect();
    }
}

fn push_fat_down((row, col): (usize, usize), maze: &mut Vec<String>) {
    let mut space_to_check = vec![(row + 1, col)];
    let mut coords_to_move_down: HashSet<(usize, usize)> = HashSet::new();
    coords_to_move_down.insert((row, col));
    loop {
        let mut next_places_to_check = vec![];
        for (r, c) in space_to_check {
            let thing = maze[r].chars().nth(c);
            if thing == Some('#') {
                // hit wall, there is nothing to do
                return;
            }
            if thing == Some('[') {
                coords_to_move_down.insert((r, c));
                coords_to_move_down.insert((r, c + 1));
                next_places_to_check.push((r + 1, c));
                next_places_to_check.push((r + 1, c + 1));
            }
            if thing == Some(']') {
                coords_to_move_down.insert((r, c - 1));
                coords_to_move_down.insert((r, c));
                next_places_to_check.push((r + 1, c - 1));
                next_places_to_check.push((r + 1, c));
            }
        }
        if next_places_to_check.is_empty() {
            break;
        }
        space_to_check = next_places_to_check;
    }
    // if we're here, we can push down!
    let max_height = coords_to_move_down.iter().max_by_key(|(r, _)| r).unwrap().0;
    for row_to_push in (row..=max_height).rev() {
        let mut prev_row: Vec<char> = maze[row_to_push + 1].chars().collect();
        let mut current_row: Vec<char> = maze[row_to_push].chars().collect();
        let things_to_move_down: Vec<&(usize, usize)> = coords_to_move_down
            .iter()
            .filter(|(r, _)| *r == row_to_push)
            .collect();

        for thing in things_to_move_down {
            let tmp = prev_row[thing.1];
            prev_row[thing.1] = current_row[thing.1];
            current_row[thing.1] = tmp;
        }
        maze[row_to_push + 1] = prev_row.iter().collect();
        maze[row_to_push] = current_row.iter().collect();
    }
}

// fn push_fat_up((row, col): (usize, usize), maze: Vec<String>) {
//     todo!();
// }

fn make_fat(maze: &Vec<String>) -> Vec<String> {
    (0..maze.len())
        .map(|row| {
            (0..maze[row].len())
                .map(|col| match maze[row].chars().nth(col) {
                    Some('.') => "..",
                    Some('@') => "@.",
                    Some('O') => "[]",
                    Some('#') => "##",
                    x => panic!("unexpected char on map: {:?}", x),
                })
                .collect::<String>()
        })
        .collect()
}
