use std::fs::read_to_string;

fn main() {
    let input: Vec<String> = read_to_string("inputs/day4")
        .unwrap()
        .split('\n')
        .filter(|line| line.len() > 0)
        .map(|line| line.to_string())
        .collect();
    println!("{}", p1(&input));
    println!("{}", p2(&input));
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

fn p2(input: &Vec<String>) -> usize {
    let diagonals_ul_br = diagonals_ul_br(input);

    let ul_br_diag_cartesian_indices: Vec<(usize, usize)> = (0..diagonals_ul_br.len())
        .flat_map(|diag_index| {
            let diag = diagonals_ul_br.get(diag_index).unwrap();
            let mut mas_matches: Vec<(usize, usize)> = diag
                .match_indices("MAS")
                .map(|(match_index, _)| (diag_index, match_index))
                .collect();
            mas_matches.append(
                &mut diag
                    .match_indices("SAM")
                    .map(|(match_index, _)| (diag_index, match_index))
                    .collect(),
            );
            mas_matches
        })
        .map(|(diag, index)| ul_br_to_cartesian(diag, index, input.len()))
        .collect();

    let diagonals_ur_bl = diagonals_ur_bl(input);
    let ur_bl_diag_cartesian_indices: Vec<(usize, usize)> = (0..diagonals_ur_bl.len())
        .flat_map(|diag_index| {
            let diag = diagonals_ur_bl.get(diag_index).unwrap();
            let mut mas_matches: Vec<(usize, usize)> = diag
                .match_indices("MAS")
                .map(|(match_index, _)| (diag_index, match_index))
                .collect();
            mas_matches.append(
                &mut diag
                    .match_indices("SAM")
                    .map(|(match_index, _)| (diag_index, match_index))
                    .collect(),
            );
            mas_matches
        })
        .map(|(diag, index)| ur_bl_to_cartesian(diag, index, input.len(), input[0].len()))
        .collect();

    ul_br_diag_cartesian_indices
        .iter()
        .filter(|(y, x)| ur_bl_diag_cartesian_indices.contains(&(*y, x + 2)))
        .count()
}

fn ul_br_to_cartesian(diag: usize, index: usize, height: usize) -> (usize, usize) {
    if diag < height {
        // start on left on row dimensions.0 - 1 - diag
        (height - 1 - diag + index, index)
    } else {
        // start on top in col dimensions.0 - diag
        (index, diag - (height - 1) + index)
    }
}

fn ur_bl_to_cartesian(diag: usize, index: usize, height: usize, width: usize) -> (usize, usize) {
    if diag < height {
        // start on right on row dimensions.0 - 1 - diag
        (height - 1 - diag + index, width - 1 - index)
    } else {
        // start on top in col dimensions.0 - diag
        (index, width - 1 - (diag - (height - 1) + index))
    }
}

#[test]
fn test_ul_br_to_cartesian() {
    let height = 10;
    let diag = 3;
    let index = 2;
    assert_eq!(ul_br_to_cartesian(diag, index, height), (8, 2));
    let height = 15;
    let diag = 5;
    let index = 1;
    assert_eq!(ul_br_to_cartesian(diag, index, height), (10, 1));
    let height = 15;
    let diag = 17;
    let index = 1;
    assert_eq!(ul_br_to_cartesian(diag, index, height), (1, 4));
}

#[test]
fn test_ur_bl_to_cartesian() {
    let height = 10;
    let width = 13;
    let diag = 3;
    let index = 2;
    assert_eq!(ur_bl_to_cartesian(diag, index, height, width), (8, 10));
    let height = 15;
    let diag = 5;
    let index = 1;
    let width = 19;
    assert_eq!(ur_bl_to_cartesian(diag, index, height, width), (10, 17));
    let height = 15;
    let diag = 17;
    let index = 1;
    let width = 19;
    assert_eq!(ur_bl_to_cartesian(diag, index, height, width), (1, 14));
}
