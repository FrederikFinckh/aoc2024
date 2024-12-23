use std::{collections::HashMap, env::args, fs::read_to_string};

fn main() {
    let read_to_string = read_to_string(format!(
        "inputs/day19{}",
        match args().last() {
            Some(x) if x == "test".to_string() => "_test",
            _ => "",
        }
    ))
    .unwrap();
    let (towels, patterns): (&str, &str) = read_to_string.split_once("\n\n").unwrap();

    let towels: Vec<String> = towels
        .split(',')
        .filter(|x| x.len() > 0)
        .map(|x| x.trim().to_string())
        .collect();

    let patterns: Vec<String> = patterns
        .split('\n')
        .map(|x| x.trim().to_string())
        .filter(|x| x.len() > 0)
        .collect();
    println!("{}", p1(&towels, &patterns));
    println!("{}", p2(&towels, &patterns));
}

fn p1(towels: &Vec<String>, patterns: &Vec<String>) -> usize {
    patterns
        .iter()
        .filter(|pattern| satisfiable(pattern, towels))
        .count()
}

fn satisfiable(pattern: &String, towels: &Vec<String>) -> bool {
    pattern.is_empty()
        || towels
            .iter()
            .filter_map(|towel| match pattern.starts_with(towel) {
                true => pattern.strip_prefix(towel),
                false => None,
            })
            .map(|x| x.to_string())
            .any(|reduced_pattern| satisfiable(&reduced_pattern, towels))
}

fn satisfiable_ways(
    pattern: &String,
    towels: &Vec<String>,
    cache: &mut HashMap<String, usize>,
) -> usize {
    if pattern.is_empty() {
        return 1;
    }
    if cache.get(pattern).is_some() {
        return *cache.get(pattern).unwrap();
    }

    towels
        .iter()
        .filter_map(|towel| match pattern.starts_with(towel) {
            true => pattern.strip_prefix(towel),
            false => None,
        })
        .map(|x| x.to_string())
        .map(|reduced_pattern| {
            let satisfiable_ways = satisfiable_ways(&reduced_pattern, towels, cache);
            cache.insert(reduced_pattern, satisfiable_ways);
            satisfiable_ways
        })
        .sum()
}

fn p2(towels: &Vec<String>, patterns: &Vec<String>) -> usize {
    let mut cache: HashMap<String, usize> = HashMap::new();
    patterns
        .iter()
        .map(|pattern| {
            println!("finding ways for {}", pattern);
            satisfiable_ways(pattern, towels, &mut cache)
        })
        .sum()
}
