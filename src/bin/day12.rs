use std::{collections::HashSet, env::args, fs::read_to_string};

fn main() {
    let input: Vec<String> = read_to_string(format!(
        "inputs/day12{}",
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

fn neighbours(point: (usize, usize), dimensions: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbours = vec![];
    if let Some(point) = up(point, dimensions) {
        neighbours.push(point);
    }
    if let Some(point) = right(point, dimensions) {
        neighbours.push(point);
    }
    if let Some(point) = down(point, dimensions) {
        neighbours.push(point);
    }
    if let Some(point) = left(point, dimensions) {
        neighbours.push(point);
    }
    neighbours
}

fn up(point: (usize, usize), dimensions: &(usize, usize)) -> Option<(usize, usize)> {
    if !in_bounds(
        (point.0 as isize - 1, point.1 as isize),
        &(dimensions.0 as isize, dimensions.1 as isize),
    ) {
        None
    } else {
        Some((point.0 - 1, point.1))
    }
}

fn right(point: (usize, usize), dimensions: &(usize, usize)) -> Option<(usize, usize)> {
    if !in_bounds(
        (point.0 as isize, point.1 as isize + 1),
        &(dimensions.0 as isize, dimensions.1 as isize),
    ) {
        None
    } else {
        Some((point.0, point.1 + 1))
    }
}

fn down(point: (usize, usize), dimensions: &(usize, usize)) -> Option<(usize, usize)> {
    if !in_bounds(
        (point.0 as isize + 1, point.1 as isize),
        &(dimensions.0 as isize, dimensions.1 as isize),
    ) {
        None
    } else {
        Some((point.0 + 1, point.1))
    }
}

fn left(point: (usize, usize), dimensions: &(usize, usize)) -> Option<(usize, usize)> {
    if !in_bounds(
        (point.0 as isize, point.1 as isize - 1),
        &(dimensions.0 as isize, dimensions.1 as isize),
    ) {
        None
    } else {
        Some((point.0, point.1 - 1))
    }
}

fn in_bounds(point: (isize, isize), dimensions: &(isize, isize)) -> bool {
    point.0 >= 0 && point.1 >= 0 && point.0 < dimensions.0 && point.1 < dimensions.1
}

struct Region {
    fields: HashSet<(usize, usize)>,
    plant: char,
    boundary: usize,
}

fn build_region(
    root_point: (usize, usize),
    input: &Vec<String>,
    mapped_fields: &HashSet<(usize, usize)>,
) -> Region {
    let plant = get_char_at(input, root_point);
    let mut boundary = 4;
    let mut fields: HashSet<(usize, usize)> = HashSet::new();

    let dimensions = (input.len(), input[0].len());
    fields.insert(root_point);

    loop {
        let neighbours_to_add: Vec<(usize, usize)> = fields
            .iter()
            .flat_map(|field| {
                neighbours(*field, &dimensions)
                    .into_iter()
                    .filter(|neighbour| get_char_at(input, *neighbour) == plant)
                    .filter(|neighbour| !fields.contains(neighbour))
                    .filter(|neighbour| !mapped_fields.contains(neighbour))
                    .collect::<Vec<(usize, usize)>>()
            })
            .collect();

        if neighbours_to_add.is_empty() {
            break;
        }
        for neighbour in neighbours_to_add {
            if fields.insert(neighbour) {
                // boundary increases by 4 - 2*neigbours already in set
                boundary = boundary
                    - 2 * neighbours(neighbour, &dimensions)
                        .into_iter()
                        .filter(|x| fields.contains(x))
                        .count()
                    + 4;
            }
        }
    }

    Region {
        fields,
        boundary,
        plant,
    }
}

fn get_char_at(input: &Vec<String>, point: (usize, usize)) -> char {
    input[point.0].chars().nth(point.1).unwrap()
}

// we have an equivalence relation, so we can just build up equivalence classes.
// for each field:
//  - continue if region exists,
//  - otherwise build up region
fn p1(input: &Vec<String>) -> usize {
    let mut mapped_fields: HashSet<(usize, usize)> = HashSet::new();
    let mut regions: Vec<Region> = Vec::new();
    for row in 0..input.len() {
        for col in 0..input[0].len() {
            if mapped_fields.contains(&(row, col)) {
                continue;
            }
            let region = build_region((row, col), input, &mapped_fields);
            for field in &region.fields {
                assert!(mapped_fields.insert(*field)); // equivalence relations assures mutually disjoined
            }
            regions.push(region);
            let _ = region.plant;
        }
    }

    regions
        .iter()
        .map(|region| region.boundary * region.fields.len())
        .sum()
}

fn p2(input: &Vec<String>) -> u64 {
    println!("{:?}", input.len());
    2
}
