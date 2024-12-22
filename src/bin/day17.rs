use core::panic;
use std::{env::args, fs::read_to_string};

fn main() {
    let input_string = read_to_string(format!(
        "inputs/day17{}",
        match args().last() {
            Some(x) if x == "test".to_string() => "_test",
            _ => "",
        }
    ))
    .unwrap();
    let (registers, program) = input_string.split_once("\n\n").unwrap();

    let registers: Vec<u32> = registers
        .split('\n')
        .filter(|x| x.len() > 0)
        .map(|x| x.split(':').nth(1).unwrap())
        .map(|x| x.trim().parse::<u32>().unwrap())
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

fn p1(r_a: u32, r_b: u32, r_c: u32, program: &Vec<usize>) -> String {
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
                a = a / (2u32.pow(combo_operand(operand, a, b, c)));
            }
            1 => {
                b = b ^ operand as u32;
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
                b = a / (2u32.pow(combo_operand(operand, a, b, c)));
            }
            7 => {
                c = a / (2u32.pow(combo_operand(operand, a, b, c)));
            }
            x => {
                panic!("invalid operand {}", x);
            }
        }
        insruction_pointer += 2;
    }
    output.join(",")
}

fn combo_operand(operand: usize, a: u32, b: u32, c: u32) -> u32 {
    match operand {
        x if x < 4 => x as u32,
        4 => a,
        5 => b,
        6 => c,
        y => panic!("incvalid operand {}", y),
    }
}

fn p2(r_a: u32, r_b: u32, r_c: u32, program: &Vec<usize>) -> u32 {
    let _ = r_a;
    let _ = r_b;
    let _ = r_c;
    let _ = program;
    2
}
