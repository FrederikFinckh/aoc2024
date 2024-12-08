use std::fs::read_to_string;

fn main() {
    let input = read_to_string("inputs/day3").unwrap().to_string();
    println!("{}", p1(&input));
    println!("{}", p2(&input));
}

fn p1(input: &String) -> u64 {
    let result: u64 = input
        .split("mul(")
        .map(|mul_candidate: &str| parse_mul(mul_candidate))
        .filter(|x| x.is_some_and(|(a, b)| a > 0 && b > 0 && a < 1000 && b < 1000))
        .map(|x| x.unwrap())
        .map(|(a, b)| a * b)
        .sum();

    result
}

fn parse_mul(mul_candidate: &str) -> Option<(u64, u64)> {
    // takes in some string after 'mul('

    let mut return_val: Option<(u64, u64)> = None;
    if let Some((first, second)) = mul_candidate.split_once(',') {
        // first,second
        if let Ok(first_number) = first.parse::<u64>() {
            if let Some((second_number, _rest)) = second.split_once(')') {
                if let Ok(second_number) = second_number.parse::<u64>() {
                    return_val = Some((first_number, second_number));
                }
            }
        }
    }
    return_val
}

fn p2(input: &String) -> u64 {
    let mut manipulated_input = "do()".to_string();
    manipulated_input.push_str(input);

    manipulated_input
        .split("don't()")
        .map(|dont_block| {
            dont_block
                .split("do()")
                .skip(1)
                .map(|do_block| p1(&do_block.to_string()))
                .sum::<u64>()
        })
        .sum()
}
