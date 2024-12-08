use std::fs::read_to_string;

fn main() {
    let input: Vec<String> = read_to_string("inputs/day4")
        .unwrap()
        .split('\n')
        .filter(|line| line.len() > 0)
        .map(|line| line.to_string())
        .collect();
    println!("{}", p1(&input));
    p2(&input);
}

fn p1(input: &Vec<String>) -> u64 {
    let directions: Vec<Vec<String>> = get_directions(input);

    directions
        .iter()
        .map(|direction| direction.iter().map(|line| count_xmas(line)).sum::<u64>())
        .sum()
}

fn get_directions(input: &Vec<String>) -> Vec<Vec<String>> {
    let columns = columns(input);
    let up_left_to_bottom_right = diagonals_ul_br(input);
    let up_right_to_bottom_left = diagonals_ur_bl(input);
    vec![
        input.clone(),
        columns,
        up_left_to_bottom_right,
        up_right_to_bottom_left,
    ]
}

fn columns(input: &Vec<String>) -> Vec<String> {
    (0..input[0].len()).map(|i| column(input, i)).collect()
}

fn column(input: &Vec<String>, i: usize) -> String {
    input
        .iter()
        .map(|line| line.chars().nth(i).unwrap())
        .collect()
}

fn diagonals_ul_br(input: &Vec<String>) -> Vec<String> {
    ((input.len() as i64 * -1) + 1..input[0].len() as i64)
        .map(|i| diagonal_ul_br(input, i))
        .collect()
}

fn diagonals_ur_bl(input: &Vec<String>) -> Vec<String> {
    ((input.len() as i64 * -1) + 1..input[0].len() as i64)
        .map(|i| diagonal_ur_bl(input, i))
        .collect()
}

fn diagonal_ul_br(input: &Vec<String>, i: i64) -> String {
    if i <= 0 {
        let start_row = (i * -1) as usize;
        let diag_length = (input.len() - start_row).min(input[0].len());
        (0..diag_length)
            .map(|index| {
                input
                    .get(start_row + index)
                    .unwrap()
                    .chars()
                    .nth(index)
                    .unwrap()
            })
            .collect()
    } else {
        let start_col = i as usize;
        let diag_length = (input.len()).min(input[0].len() - start_col);
        (0..diag_length)
            .map(|index| {
                input
                    .get(index)
                    .unwrap()
                    .chars()
                    .nth(index + start_col)
                    .unwrap()
            })
            .collect()
    }
}

fn diagonal_ur_bl(input: &Vec<String>, i: i64) -> String {
    if i <= 0 {
        let start_row = (i * -1) as usize;
        let line_length = input[0].len();
        let diag_length = (input.len() - start_row).min(line_length);
        (0..diag_length)
            .map(|index| {
                input
                    .get(start_row + index)
                    .unwrap()
                    .chars()
                    .nth(line_length - 1 - index)
                    .unwrap()
            })
            .collect()
    } else {
        let start_col = i as usize;
        let line_length = input[0].len();
        let diag_length = (input.len()).min(line_length - start_col);
        (0..diag_length)
            .map(|index| {
                input
                    .get(index)
                    .unwrap()
                    .chars()
                    .nth(line_length - 1 - (index + start_col))
                    .unwrap()
            })
            .collect()
    }
}

fn count_xmas(line: &String) -> u64 {
    line.matches("XMAS").count() as u64 + line.matches("SAMX").count() as u64
}

fn p2(_input: &Vec<String>) {}
