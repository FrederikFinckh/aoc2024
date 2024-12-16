use std::{
    collections::{HashMap, HashSet},
    env::args,
    fs::read_to_string,
};

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

    // we have an equivalence relation, so we can just build up equivalence classes.
    // for each field:
    //  - continue if region exists,
    //  - otherwise build up region
    let mut mapped_fields: HashSet<(usize, usize)> = HashSet::new();
    let mut regions: Vec<Region> = Vec::new();
    for row in 0..input.len() {
        for col in 0..input[0].len() {
            if mapped_fields.contains(&(row, col)) {
                continue;
            }
            let region = build_region((row, col), &input, &mapped_fields);
            for field in &region.fields {
                assert!(mapped_fields.insert(*field)); // equivalence relations assures mutually disjoined
            }
            regions.push(region);
            let _ = region.plant;
        }
    }

    println!("{}", p1(&regions));
    println!("{}", p2(&regions));
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

#[derive(PartialEq)]
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

fn p1(regions: &Vec<Region>) -> usize {
    regions
        .iter()
        .map(|region| region.boundary * region.fields.len())
        .sum()
}

fn p2(regions: &Vec<Region>) -> usize {
    let mut region_by_field: HashMap<(usize, usize), &Region> = HashMap::new();
    for region in regions {
        for field in &region.fields {
            region_by_field.insert(*field, &region);
        }
    }

    // handle edges

    let mut cost = 0;

    for region in regions {
        let min_row = region.fields.iter().min_by_key(|point| point.0).unwrap().0;
        let min_col = region.fields.iter().min_by_key(|point| point.1).unwrap().1;
        let max_row = region.fields.iter().max_by_key(|point| point.0).unwrap().0;
        let max_col = region.fields.iter().max_by_key(|point| point.1).unwrap().1;

        let mut amount_boundaries = 0;

        for row in min_row..=max_row {
            let mut top_boundary_points = HashSet::new();
            let mut top_boundary_count = 0;
            let mut bot_boundary_points = HashSet::new();
            let mut bot_boundary_count = 0;
            for col in min_col..=max_col {
                let current_point = (row, col);
                let left_neighbour = (row, col as isize - 1);
                let up_neighbour = (row as isize - 1, col);
                let down_neighbour = (row + 1, col);

                if region_by_field.get(&current_point) == Some(&region)
                    && (up_neighbour.0 < 0
                        || Some(&region)
                            != region_by_field.get(&(up_neighbour.0 as usize, up_neighbour.1)))
                {
                    // top boundary
                    top_boundary_points.insert(current_point);
                    if left_neighbour.1 < 0
                        || !top_boundary_points
                            .contains(&(left_neighbour.0, left_neighbour.1 as usize))
                    {
                        top_boundary_count += 1;
                    }
                }

                if region_by_field.get(&current_point) == Some(&region)
                    && Some(&region) != region_by_field.get(&down_neighbour)
                {
                    bot_boundary_points.insert(current_point);
                    if left_neighbour.1 < 0
                        || !bot_boundary_points
                            .contains(&(left_neighbour.0, left_neighbour.1 as usize))
                    {
                        bot_boundary_count += 1;
                    }
                }
            }
            amount_boundaries += top_boundary_count + bot_boundary_count;
        }
        for col in min_col..=max_col {
            let mut left_boundary_points = HashSet::new();
            let mut left_boundary_count = 0;
            let mut right_boundary_points = HashSet::new();
            let mut right_boundary_count = 0;
            for row in min_row..=max_row {
                let current_point = (row, col);
                let left_neighbour = (row, col as isize - 1);
                let up_neighbour = (row as isize - 1, col);
                let right_neighbour = (row, col + 1);

                if region_by_field.get(&current_point) == Some(&region)
                    && (left_neighbour.1 < 0
                        || Some(&region)
                            != region_by_field.get(&(left_neighbour.0, left_neighbour.1 as usize)))
                {
                    // top boundary
                    left_boundary_points.insert(current_point);
                    if up_neighbour.0 < 0
                        || !left_boundary_points
                            .contains(&(up_neighbour.0 as usize, up_neighbour.1))
                    {
                        left_boundary_count += 1;
                    }
                }

                if region_by_field.get(&current_point) == Some(&region)
                    && Some(&region) != region_by_field.get(&right_neighbour)
                {
                    right_boundary_points.insert(current_point);
                    if up_neighbour.0 < 0
                        || !right_boundary_points
                            .contains(&(up_neighbour.0 as usize, up_neighbour.1))
                    {
                        right_boundary_count += 1;
                    }
                }
            }
            amount_boundaries += left_boundary_count + right_boundary_count;
        }

        cost += amount_boundaries * region.fields.len();
    }
    cost
}
