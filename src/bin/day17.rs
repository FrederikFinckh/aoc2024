use core::panic;
use std::{env::args, fs::read_to_string};

fn main() {
    let input_string = read_to_string(format!(
        "inputs/day17{}",
        match args().last() {
            Some(x) if x == "test".to_string() => "_test",
            Some(x) if x == "test2".to_string() => "_test2",
            _ => "",
        }
    ))
    .unwrap();
    let (registers, program) = input_string.split_once("\n\n").unwrap();

    let registers: Vec<u64> = registers
        .split('\n')
        .filter(|x| x.len() > 0)
        .map(|x| x.split(':').nth(1).unwrap())
        .map(|x| x.trim().parse::<u64>().unwrap())
        .collect();

    let r_a = registers[0];
    let r_b = registers[1];
    let r_c = registers[2];

    let program: Vec<usize> = program
        .split(':')
        .nth(1)
        .unwrap()
        .split(',')
        .map(|x| x.trim().parse::<usize>().unwrap())
        .collect();

    println!("{}", p1(r_a, r_b, r_c, &program));
    println!("{}", p2(r_a, r_b, r_c, &program));
}

fn p1(r_a: u64, r_b: u64, r_c: u64, program: &Vec<usize>) -> String {
    let mut insruction_pointer = 0;
    let mut a = r_a;
    let mut b = r_b;
    let mut c = r_c;
    let mut output = vec![];
    loop {
        if insruction_pointer + 1 >= program.len() {
            break;
        }
        let opcode = program[insruction_pointer];
        let operand = program[insruction_pointer + 1];
        match opcode {
            0 => {
                a = a / (2u32.pow(combo_operand(operand, a, b, c) as u32)) as u64;
            }
            1 => {
                b = b ^ operand as u64;
            }
            2 => {
                b = combo_operand(operand, a, b, c) % 8;
            }
            3 => {
                if a == 0 {
                    break;
                }
                insruction_pointer = operand;
                continue;
            }
            4 => {
                b = b ^ c;
            }
            5 => {
                output.push((combo_operand(operand, a, b, c) % 8).to_string());
            }
            6 => {
                b = a / (2u32.pow(combo_operand(operand, a, b, c) as u32)) as u64;
            }
            7 => {
                c = a / (2u32.pow(combo_operand(operand, a, b, c) as u32)) as u64;
            }
            x => {
                panic!("invalid operand {}", x);
            }
        }
        insruction_pointer += 2;
    }
    output.join(",")
}

fn combo_operand(operand: usize, a: u64, b: u64, c: u64) -> u64 {
    match operand {
        x if x < 4 => x as u64,
        4 => a,
        5 => b,
        6 => c,
        y => panic!("incvalid operand {}", y),
    }
}

fn p2(_: u64, r_b: u64, r_c: u64, program: &Vec<usize>) -> u64 {
    let mut prev = 0;
    for output_number in 0..program.len() {
        for x in 0..1024 * 1024 {
            let test_num = 8 * prev + x;
            let out = p1(test_num, r_b, r_c, program);
            println!("x = {}, 8*prev + x = {} output: {}", x, test_num, out);
            println!(
                "{:?}",
                program[program.len() - 1 - output_number..].to_vec()
            );
            if program[program.len() - 1 - output_number..]
                == out
                    .split(",")
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>()
            {
                prev = test_num;
                break;
            }
        }
    }
    prev
}

// 2,4 -> B = A % 8
// 1,7 -> B = B ^ 111
// 7,5 -> C = A / (2**B)
// 0,3 -> A = A / 8
// 4,4 -> B = B ^ C
// 1,7 -> B = B ^ 111
// 5,5 -> print out B%8
// 3,0 -> jump to start (or break)

// first round : 2 -> B%8 = 2
// secon round : 4 -> B%8 = 4
// ...

// A floor divided by 8 every time -> bitshift 3 to the right

// B is last three bits of A

// XOR with 7 => flip all bits of B

// shift A by B bits to the right, set c to this value
// B XOR (last three bits of C)

// -> copy A, throw away B bits, take next three bits and XOR with B

// flip bits of B

// print B

// throw away 3 bits of A

// repeat

// example in base 2

// A = 101010111110111

// B = 111 (last 3 bits)
// B = 000 (flip bits)
// B = 111 (mod 3 last bits of A after removing B bits from right)
// B = 000 (mod 111)
// -> print 7

// A = 101010111110

// B = 110
// B = 001
// B = 110
// B = 001
// -> print 1

// inverse question:
// when is B = 010

// flip bits =>
// 101

// 101 = old_b XOR A bitshifted old_b_times_to_right
// old_b = flipped last 3 bits of A

// out = flip(old_b XOR A >> old_b) = flip(flip(A%8) XOR A >> flip(A%8))

// flip(x) = x^111

// (111^ (111^(A%8) ^ (A >> (111^A%8))))%8
// = (A%8 ^ (A >> (111^A%8)))%8

// A has a length of 48, 47 or 46 bits.

// fuck it, let's just calculate one step at a time and brute force
