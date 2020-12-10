use std::str::FromStr;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashSet, HashMap};
use strum_macros::Display;
use std::error::Error;

#[macro_use]
extern crate simple_error;

#[derive(Debug, PartialEq, Display, Clone)]
#[allow(non_camel_case_types)]
enum Instruction {
    acc(i32),
    jmp(i32),
    nop(i32),
}

impl FromStr for Instruction {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> ::std::result::Result<Instruction, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<&str>>();
        match parts[0] {
            "acc" => Ok(Instruction::acc(i32::from_str(parts[1])?)),
            "jmp" => Ok(Instruction::jmp(i32::from_str(parts[1])?)),
            "nop" => if parts.len() > 1 {
                Ok(Instruction::nop(i32::from_str(parts[1])?))
            } else {
                Ok(Instruction::nop(0))
            },
            _ => bail!("Unknown instruction {}", s),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Program {
    instructions: HashMap<u32, Instruction>,
}

impl Program {
    fn from_file(filename: &str) -> Self {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let mut instructions = HashMap::<u32, Instruction>::new();
        let mut position = 0;
        for line in reader.lines() {
            let instruction = Instruction::from_str(&line.unwrap()).unwrap();
            instructions.insert(position, instruction);
            position += 1;
        }
    
        Program {instructions}
    }
}

#[derive(Debug, PartialEq, Default)]
struct State {
    accumulator: i32,
    pc: u32,
}

fn step(mut state: State, program: &Program) -> State {
    let inst = &program.instructions[&state.pc];
    match inst {
        Instruction::acc(val) => {
            state.accumulator += val;
            state.pc += 1;
        },
        Instruction::jmp(val) => {
            let newpc = state.pc as i32 + val;
            assert!(newpc >= 0);
            state.pc = newpc as u32;
        },
        Instruction::nop(_) => {
            state.pc += 1;
        }
    };
    state
}

fn run_to_repeat(program: &Program) -> State {
    let mut state = State::default();
    let mut executed = HashSet::<u32>::new();
    while !executed.contains(&state.pc) {
        executed.insert(state.pc);
        state = step(state, program);
    }

    state
}

fn mutate(mut program: Program, last_mutation: i32, jmp2nop: bool) -> Option<(Program, i32)> {
    let mut indicies = program.instructions.keys().copied().collect::<Vec::<u32>>();
    // let mut instructions = program.instructions;
    indicies.sort();
    for i in indicies {
        if last_mutation >= 0 && i <= last_mutation as u32 {
            continue;
        }
        let inst = &program.instructions[&i];
        if jmp2nop {
            if let Instruction::jmp(val) = inst {
                *program.instructions.get_mut(&i).unwrap() = Instruction::nop(val.to_owned());
                return Some((program, i.to_owned() as i32));
            }
        } else {
            if let Instruction::nop(val) = inst {
                *program.instructions.get_mut(&i).unwrap() = Instruction::jmp(val.to_owned());
                return Some((program, i.to_owned() as i32));
            }
        }
    }

    None
}

fn terminates(program: &Program) -> Option<State> {
    let mut state = State::default();
    let mut executed = HashSet::<u32>::new();
    while !executed.contains(&state.pc) {
        if state.pc as usize == program.instructions.len() {
            return Some(state);
        }
        executed.insert(state.pc);
        state = step(state, program);
    }

    None
}

fn try_terminate(original_program: &Program) -> State {
    // try program as is, maybe we are lucky
    if let Some(state) = terminates(original_program) {
        return state;
    }
    let mut last_mutation = -1;
    while let Some((program, mutated)) = mutate(original_program.clone(), last_mutation, true) {
        last_mutation = mutated;
        if let Some(state) = terminates(&program) {
            return state;
        }
    }

    let mut last_mutation = -1;
    while let Some((program, mutated)) = mutate(original_program.clone(), last_mutation, false) {
        last_mutation = mutated;
        if let Some(state) = terminates(&program) {
            return state;
        }
    }
    panic!("No solution found.");
}

fn main() {
    let program = Program::from_file("input.txt");
    let end_state = run_to_repeat(&program);
    println!("End state part 1: {}", end_state.accumulator);
    let end_state = try_terminate(&program);
    println!("End state part 2: {}", end_state.accumulator);
}

#[cfg(test)]
mod tests {
use std::str::FromStr;
use crate::{Program, Instruction, run_to_repeat, try_terminate};
    #[test]
    fn read_instructions() {
        let program = Program::from_file("testinput.txt");
        assert_eq!(program.instructions.len(), 9);
        assert_eq!(program.instructions[&1], Instruction::acc(1));
        assert_eq!(program.instructions[&4], Instruction::jmp(-3));
    }

    #[test]
    fn test_run() {
        let program = Program::from_file("testinput.txt");
        let end_state = run_to_repeat(&program);
        assert_eq!(end_state.pc, 1);
        assert_eq!(end_state.accumulator, 5);
    }

    #[test]
    fn test_mutate() {
        let program = Program::from_file("testinput.txt");
        let end_state = try_terminate(&program);
        assert_eq!(end_state.pc, 9);
        assert_eq!(end_state.accumulator, 8);
    }

}
