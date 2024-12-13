use std::fs::read_to_string;

fn main() {
    let input: Vec<u32> = read_to_string("inputs/day9")
        .unwrap()
        .trim()
        .chars()
        .map(|character| character.to_digit(10).unwrap())
        .collect();
    println!("{}", p1(&input));
    println!("{}", p2(&input));
}

fn p1(input: &Vec<u32>) -> usize {
    let mut fs_vec: Vec<String> = (0..input.len())
        .flat_map(|index| {
            if index % 2 == 0 {
                vec![(index / 2).to_string(); input[index] as usize]
            } else {
                vec![".".to_string(); input[index] as usize]
            }
        })
        .collect();

    //let j go from end to start
    //let i go from start to end
    //increment i until it is free
    //decrement j until not free
    // swap i,j until i>=j
    let mut i = 0;
    let mut j = fs_vec.len() - 1;

    while i < j {
        if !(fs_vec[i] == ".") {
            i = i + 1;
            continue;
        }
        if fs_vec[j] == "." {
            j = j - 1;
            continue;
        }
        let tmp = fs_vec[j].clone();
        fs_vec[i] = tmp;
        i = i + 1;
        fs_vec[j] = ".".to_string();
        j = j - 1;
    }
    (0..fs_vec.len())
        .filter(|index| !(fs_vec[*index] == ".".to_string()))
        .map(|index| index * fs_vec[index].parse::<usize>().unwrap())
        .sum()
}

enum Block {
    File { id: usize, size: u32 },
    Space { size: u32 },
}

fn p2(input: &Vec<u32>) -> usize {
    let mut fs_vec: Vec<Block> = (0..input.len())
        .map(|index| {
            if index % 2 == 0 {
                Block::File {
                    id: index / 2,
                    size: input[index],
                }
            } else {
                Block::Space { size: input[index] }
            }
        })
        .collect();

    let mut j = fs_vec.len() - 1;
    while j > 0 {
        // _visualize(serialize(&fs_vec));
        if let Block::File { id, size } = fs_vec[j] {
            // find if there is space to the left
            if let Some((suitable_space_index, available_space)) = (0..j)
                .filter_map(|i| match fs_vec[i] {
                    Block::File { id: _, size: _ } => None,
                    Block::Space { size: empty_size } => Some((i, empty_size)),
                })
                .find(|(_index, empty_size)| *empty_size >= size)
            {
                // move file to empty space
                // -> there is one more block then, so increment j by one.
                fs_vec[suitable_space_index] = Block::Space {
                    size: available_space - size,
                };
                fs_vec[j] = Block::Space { size };
                fs_vec.insert(suitable_space_index, Block::File { id, size });
                j += 1;
            }
        }
        j -= 1;
    }

    let fs_vec = serialize(&fs_vec);

    (0..fs_vec.len())
        .filter(|index| !(fs_vec[*index] == ".".to_string()))
        .map(|index| index * fs_vec[index].parse::<usize>().unwrap())
        .sum()
}

fn serialize(fs_vec: &Vec<Block>) -> Vec<String> {
    (0..fs_vec.len())
        .flat_map(|index| match fs_vec[index] {
            Block::File { id, size } => vec![id.to_string(); size as usize],
            Block::Space { size } => vec![".".to_string(); size as usize],
        })
        .collect()
}

fn _visualize(fs_vec: Vec<String>) {
    println!();
    for x in fs_vec {
        print!("{}", x);
    }
}
