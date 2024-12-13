use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let input: String = read_to_string("inputs/day8").unwrap().to_string();
    let occuring_chars = HashSet::from_iter(input.trim().replace(".", "").chars());

    let input_lines: Vec<String> = input
        .split('\n')
        .filter(|x| x.len() > 0)
        .map(|x| x.to_string())
        .collect();

    println!("{}", p1(&occuring_chars, &input_lines));
    println!("{}", p2(&occuring_chars, &input_lines));
}

fn p1(occuring_chars: &HashSet<char>, input_lines: &Vec<String>) -> usize {
    let dimensions = (input_lines.len(), input_lines[0].len());
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();
    for character in occuring_chars {
        let occurences = get_occuring_indices(*character, input_lines);
        for occ1 in &occurences {
            for occ2 in &occurences {
                if occ1 == occ2 {
                    continue;
                }
                if in_range(occ1, occ2, &dimensions) {
                    antinodes.insert((2 * occ1.0 - occ2.0, 2 * occ1.1 - occ2.1));
                }
                if in_range(occ2, occ1, &dimensions) {
                    antinodes.insert((2 * occ2.0 - occ1.0, 2 * occ2.1 - occ1.1));
                }
            }
        }
    }

    // _vizualize(input_lines, &antinodes);
    antinodes.len()
}

fn _vizualize(input_lines: &Vec<String>, antinodes: &HashSet<(usize, usize)>) {
    println!("");
    for row in 0..input_lines.len() {
        for col in 0..input_lines[row].len() {
            if antinodes.contains(&(row, col)) {
                print!("#");
            } else {
                print!("{}", input_lines[row].chars().nth(col).unwrap());
            }
        }
        println!("")
    }

    println!("");
}

fn in_range(a: &(usize, usize), b: &(usize, usize), dimensions: &(usize, usize)) -> bool {
    2 * a.0 >= b.0 && 2 * a.0 - b.0 < dimensions.0 && 2 * a.1 >= b.1 && 2 * a.1 - b.1 < dimensions.1
}

fn get_occuring_indices(character: char, input_lines: &Vec<String>) -> Vec<(usize, usize)> {
    (0..input_lines.len())
        .flat_map(|row| {
            (0..input_lines[0].len())
                .filter(|col| input_lines[row].chars().nth(*col).unwrap() == character)
                .map(|col| (row, col))
                .collect::<Vec<(usize, usize)>>()
        })
        .collect()
}

fn in_range2(
    a: &(usize, usize),
    b: &(usize, usize),
    k: usize,
    dimensions: &(usize, usize),
) -> bool {
    // check if a+ k(a-b)
    (1 + k) * a.0 >= k * b.0
        && (1 + k) * a.0 - k * b.0 < dimensions.0
        && (1 + k) * a.1 >= k * b.1
        && (1 + k) * a.1 - k * b.1 < dimensions.1
}

fn p2(occuring_chars: &HashSet<char>, input_lines: &Vec<String>) -> usize {
    let dimensions = (input_lines.len(), input_lines[0].len());

    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();
    for character in occuring_chars {
        let occurences = get_occuring_indices(*character, input_lines);
        for occ1 in &occurences {
            for occ2 in &occurences {
                if occ1 == occ2 {
                    continue;
                }

                let mut k = 0;
                loop {
                    if in_range2(occ1, occ2, k, &dimensions) {
                        antinodes
                            .insert(((1 + k) * occ1.0 - k * occ2.0, (1 + k) * occ1.1 - k * occ2.1));
                    } else {
                        break;
                    }
                    k += 1;
                }
                k = 0;
                loop {
                    if in_range2(occ2, occ1, k, &dimensions) {
                        antinodes
                            .insert(((1 + k) * occ2.0 - k * occ1.0, (1 + k) * occ2.1 - k * occ1.1));
                    } else {
                        break;
                    }
                    k += 1;
                }
            }
        }
    }

    // _vizualize(input_lines, &antinodes);
    antinodes.len()
}
