use std::{env::args, fs::read_to_string};

fn main() {
    let input: Vec<String> = read_to_string(format!(
        "inputs/dayN{}",
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

fn p1(input: &Vec<String>) -> u64 {
    println!("{:?}", input);
    1
}

fn p2(input: &Vec<String>) -> u64 {
    println!("{:?}", input);
    2
}
