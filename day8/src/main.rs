use crate::Instruction::{Acc, Jmp, Nop};
use std::collections::HashSet;
use std::str::FromStr;
use std::string::ParseError;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

impl Instruction {
    fn to_nop(&self) -> Self {
        match self {
            Jmp(arg) => Nop(*arg),
            Acc(_) => panic!("Not possible"),
            Nop(_) => panic!("Already NOP"),
        }
    }
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let string: Vec<&str> = s.split(' ').collect();

        let instruction = match string.as_slice() {
            ["acc", arg] => Acc(arg.parse().unwrap()),
            ["nop", arg] => Nop(arg.parse().unwrap()),
            ["jmp", arg] => Jmp(arg.parse().unwrap()),
            _ => unreachable!(),
        };

        Ok(instruction)
    }
}

fn main() {
    let instructions: Vec<Instruction> = include_str!("input")
        .lines()
        .map(FromStr::from_str)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    println!("{}", part1(&instructions));
    println!("{}", part2(instructions));
}

fn part1(instructions: &[Instruction]) -> i32 {
    run_code(instructions).0
}

fn part2(mut instructions: Vec<Instruction>) -> i32 {
    let mut prev_change = None;

    // in order to have a loop you need a backward jump
    let mut backward_jumps: Vec<(usize, Instruction)> = instructions
        .to_vec()
        .into_iter()
        .enumerate()
        .filter(|(_, instruction)| matches!(instruction, Jmp(arg) if arg < &0))
        .collect();

    // start with the longest jmp
    backward_jumps.sort();

    loop {
        let (accumulator, pc) = run_code(&instructions);

        if pc == instructions.len() {
            break accumulator;
        }

        // restore previous change
        if let Some((instruction, saved_pc)) = prev_change {
            instructions[saved_pc] = instruction;
        }

        // apply next jmp or nop
        if let Some((jmp_location, _)) = backward_jumps.pop() {
            // save counter
            prev_change = Some((instructions[jmp_location], jmp_location));
            instructions[jmp_location] = instructions[jmp_location].to_nop();
        }
    }
}

fn run_code(instructions: &[Instruction]) -> (i32, usize) {
    let mut accumulator = 0;
    let mut pc: i32 = 0;
    let mut pc_cache = HashSet::new();

    loop {
        // terminated
        if pc as usize == instructions.len() {
            break;
        }

        // found a loop
        if pc_cache.contains(&pc) {
            break;
        }

        pc_cache.insert(pc);

        match instructions[pc as usize] {
            Acc(arg) => {
                accumulator += arg;
                pc += 1;
            }
            Jmp(arg) => pc += arg,
            Nop(_) => pc += 1,
        }
    }

    (accumulator, pc as usize)
}
