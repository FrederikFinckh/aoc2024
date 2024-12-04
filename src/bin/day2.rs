use std::fs::read_to_string;

fn main() {
    let input: Vec<Vec<i64>> = read_to_string("inputs/day2_test")
        .unwrap()
        .split('\n')
        .filter(|l| l.len() > 0)
        .map(|l| {
            l.split(' ')
                .map(|n| n.trim().parse::<i64>().unwrap())
                .collect()
        })
        .collect();
    p1(&input);
    p2(input);
}

fn p1(input: &Vec<Vec<i64>>) {
    let safe_rows = input.iter().filter(|levels| is_safe(levels)).count();
    println!("{}", safe_rows);
}

fn is_safe(levels: &Vec<i64>) -> bool {
    if let Some(mut prev) = levels.get(0) {
        let first = prev.clone();
        //check safe increasing
        let mut safe_increasing = true;
        for level in levels.iter().skip(1) {
            if level <= prev || *level > prev + 3 {
                safe_increasing = false;
                break;
            }
            prev = level;
        }
        if !safe_increasing {
            prev = &first;
            for level in levels.iter().skip(1) {
                if level >= prev || *level < prev - 3 {
                    return false;
                }
                prev = level;
            }
        }
    }
    true
}

fn p2(input: Vec<Vec<i64>>) {
    let safe_rows = input
        .iter()
        .filter(|levels| is_safe_with_dampening(levels))
        .count();
    println!("{}", safe_rows);
}

fn is_safe_with_dampening(levels: &Vec<i64>) -> bool {
    println!();
    println!();
    println!();
    println!("=====");
    if let Some(mut prev_prev) = levels.get(0) {
        if let Some(mut prev) = levels.get(1) {
            println!("checking {:?} for increasing", levels);
            let second = prev.clone();
            let first = prev_prev.clone();
            let mut dampener_used = false;

            let mut safe_increasing = true;

            for level in levels.iter().skip(2) {
                if level <= prev || *level > prev + 3 {
                    if dampener_used {
                        safe_increasing = false;
                        break;
                    } else {
                        println!("using dampener, {:?}", levels);
                        dampener_used = true;
                        // if skipping prev is ok continue with prev_prev=prev_prev, level=prev
                        if level > prev_prev && *level <= prev_prev + 3 {
                            //skip prev
                            println!(
                                "skipping prev {} at {}, {}, {}",
                                prev, prev_prev, prev, level
                            );
                            prev = level;
                        } else {
                            //skip level
                            println!(
                                "skipping level {} at {}, {}, {}",
                                level, prev_prev, prev, level
                            );
                            continue;
                        }
                    }
                } else {
                    prev_prev = prev;
                    prev = level;
                }
            }

            if !safe_increasing {
                println!("checking {:?} for decreasing", levels);
                prev_prev = &first;
                prev = &second;
                dampener_used = false;
                for level in levels.iter().skip(2) {
                    if level >= prev || *level < prev - 3 {
                        if dampener_used {
                            return false;
                        } else {
                            println!("using dampener, {:?}", levels);
                            dampener_used = true;
                            // if skipping prev is ok continue with prev_prev=prev_prev, level=prev
                            // otherwise skip level
                            if level < prev_prev && *level >= prev_prev - 3 {
                                println!(
                                    "skipping prev {} at {}, {}, {}",
                                    prev, prev_prev, prev, level
                                );
                                // skip prev
                                prev = level;
                            } else {
                                println!(
                                    "skipping level {} at {}, {}, {}",
                                    level, prev_prev, prev, level
                                );
                                // skip level
                                continue;
                            }
                        }
                    } else {
                        prev_prev = prev;
                        prev = level;
                    }
                }
            }
        }
    }

    println!("safe!");
    true
}
