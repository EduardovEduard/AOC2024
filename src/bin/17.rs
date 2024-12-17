use std::collections::{HashSet, VecDeque};
use itertools::Itertools;

advent_of_code::solution!(17);

#[derive(Debug)]
struct Interpreter {
    registers: [u64; 3],
    program: Vec<u8>,
    ip: usize,
}

impl Interpreter {
    fn run(&mut self) -> Vec<u64> {
        let mut output = vec![];
        while self.ip < self.program.len() {
            let instruction = self.program[self.ip];
            let operand = self.program[self.ip + 1] as u64;
            self.run_instruction(&mut output, instruction, operand);
        }
        output
    }

    fn reset(&mut self, a: u64, b: u64, c: u64) {
        self.ip = 0;
        self.registers[0] = a;
        self.registers[1] = b;
        self.registers[2] = c;
    }

    fn run_instruction(&mut self, output: &mut Vec<u64>, instruction: u8, operand: u64) {
        match instruction {
            0 => { // adv
                let op = self.combo(operand);
                let a = self.registers[0];
                self.registers[0] = a >> op;
                self.ip += 2;
            },
            1 => { // bxl
                self.registers[1] = self.registers[1] ^ operand;
                self.ip += 2;
            },
            2 => { // bst
                self.registers[1] = self.combo(operand) % 8;
                self.ip += 2;
            },
            3 => { // jnz
                if self.registers[0] != 0 {
                    self.ip = operand as usize;
                } else {
                    self.ip += 2;
                }
            },
            4 => { // bxc
                self.registers[1] = self.registers[1] ^ self.registers[2];
                self.ip += 2
            },
            5 => { // out
                let value = self.combo(operand) % 8;
                output.push(value);
                self.ip += 2
            },
            6 => { // bdv
                let a = self.registers[0];
                let op = self.combo(operand);
                self.registers[1] = a >> op;
                self.ip += 2;
            },
            7 => { // cdv
                let a = self.registers[0];
                let op = self.combo(operand);
                self.registers[2] = a >> op;
                self.ip += 2;
            },
            _ => {
                panic!("Unknown instruction: {}", instruction)
            }
        }
    }

    fn combo(&mut self, operand: u64) -> u64 {
        match operand {
            0..=3 => operand,
            4 => self.registers[0],
            5 => self.registers[1],
            6 => self.registers[2],
            _ => panic!("Unknown operand: {}", operand)
        }
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let mut interpreter= parse_input(input);
    let mut result = interpreter.run();
    Some(result.iter().join(","))
}

fn dfs(a: u64, target_i: usize, target: &Vec<u8>, interpreter: &mut Interpreter, out: &mut Vec<u64>) {
    interpreter.reset(a, 0, 0);
    let res = interpreter.run();
    let result = res[0];

    if result == target[target_i] as u64 {
        if target_i == target.len() - 1 {
            out.push(a);
            return;
        }
        for i in 0x0..0x8 {
            dfs(a << 3 | i, target_i + 1, target, interpreter, out);
        }
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut interpreter= parse_input(input);
    let mut target = interpreter.program.clone();

    target.reverse();
    let mut res = vec![];
    for a in 0x0..0x8 {
        dfs(a, 0, &target, &mut interpreter, &mut res);
    }

    Some(*res.iter().min().unwrap())
}

fn parse_input(input: &str) -> Interpreter {
    let number = regex::Regex::new(r"(\d+)").unwrap();
    let mut registers = [0; 3];
    let lines = input.lines().collect_vec();

    registers[0] = number.captures(lines[0]).unwrap()
        .get(1).unwrap().as_str().parse::<u64>().unwrap();
    registers[1] = number.captures(lines[1]).unwrap()
        .get(1).unwrap().as_str().parse::<u64>().unwrap();
    registers[2] = number.captures(lines[2]).unwrap()
        .get(1).unwrap().as_str().parse::<u64>().unwrap();

    let program = lines[4].strip_prefix("Program: ").unwrap()
        .split(",")
        .map(|x| x.parse::<u8>().unwrap())
        .collect_vec();

    Interpreter {
        registers,
        program,
        ip: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(String::from("4,6,3,5,6,3,5,2,1,0")));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(216584205979245));
    }
}
