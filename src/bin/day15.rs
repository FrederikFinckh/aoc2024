use std::{env::args, fs::read_to_string};

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

fn p1(maze: &Vec<String>, moves: &Vec<char>) -> u64 {
    let _ = moves;
    for line in maze {
        println!("{}", line);
    }
    1
}

fn p2(maze: &Vec<String>, moves: &Vec<char>) -> u64 {
    let _ = moves;
    for line in maze {
        println!("{}", line);
    }
    2
}
