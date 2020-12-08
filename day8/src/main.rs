use std::collections::HashSet;

fn main() {
    let instructions: Vec<(&str, i32)> = include_str!("input")
        .lines()
        .map(|x| {
            let v: Vec<&str> = x.split(' ').collect();
            (v[0], v[1].parse().unwrap())
        })
        .collect();

    println!("{}", part1(&instructions));
    println!("{}", part2(instructions));
}

fn part1(instructions: &[(&str, i32)]) -> i32 {
    run_code(instructions).0
}

fn part2(mut instructions: Vec<(&str, i32)>) -> i32 {
    let mut prev_change = None;

    let mut jmps_and_nops: Vec<(usize, i32)> = instructions
        .to_vec()
        .into_iter()
        .enumerate()
        .filter(|(_, (op, _))| *op == "jmp" || *op == "nop")
        .map(|(ic, (_op, arg))| (ic, arg))
        .collect();

    // try the one with the longest arg first
    jmps_and_nops.sort_by(|(_, a), (_, b)| b.cmp(a));

    loop {
        let (accumulator, ic) = run_code(&instructions);

        if ic == instructions.len() {
            break accumulator;
        }

        // restore previous change
        if let Some((op, arg, saved_ic)) = prev_change {
            instructions[saved_ic] = (op, arg);
        }

        // apply next jmp or nop
        if let Some((ic, _)) = jmps_and_nops.pop() {
            // save counter
            let (op, arg) = instructions[ic];
            prev_change = Some((op, arg, ic));

            // change instruction
            if op == "nop" {
                instructions[ic].0 = "jmp";
            } else {
                instructions[ic].0 = "nop";
            }
        }
    }
}

fn run_code(instructions: &[(&str, i32)]) -> (i32, usize) {
    let mut accumulator = 0;
    let mut instruction_counter: i32 = 0;
    let mut executed = HashSet::new();

    loop {
        // terminated
        if instruction_counter as usize == instructions.len() {
            break;
        }

        let (op, arg) = instructions[instruction_counter as usize];

        // found a loop
        if executed.contains(&instruction_counter) {
            break;
        }

        executed.insert(instruction_counter);

        match (op, arg) {
            ("acc", arg) => {
                accumulator += arg;
                instruction_counter += 1;
            }
            ("jmp", arg) => instruction_counter += arg,
            _ => instruction_counter += 1, // nop
        }
    }

    (accumulator, instruction_counter as usize)
}
