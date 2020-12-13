use std::str::FromStr;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashSet, HashMap};
use std::error::Error;


#[macro_use]
extern crate simple_error;

#[derive(Debug, PartialEq, Clone)]
enum Instruction {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

impl FromStr for Instruction {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> ::std::result::Result<Instruction, Self::Err> {
        let instr = s.chars().next().unwrap();
        let rest = &s[1..];
        match instr {
            'N' => Ok(Instruction::North(i32::from_str(rest)?)),
            'S' => Ok(Instruction::South(i32::from_str(rest)?)),
            'E' => Ok(Instruction::East(i32::from_str(rest)?)),
            'W' => Ok(Instruction::West(i32::from_str(rest)?)),
            'L' => Ok(Instruction::Left(i32::from_str(rest)?)),
            'R' => Ok(Instruction::Right(i32::from_str(rest)?)),
            'F' => Ok(Instruction::Forward(i32::from_str(rest)?)),
            _ => bail!("Unknown instruction {}", s),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Position {
    north: i32,
    east: i32,
}

#[derive(Debug, PartialEq)]
struct State {
    current_pos: Position,
    heading: u32,
    waypoint_pos: Position,
}

impl State {
    fn update_heading(&mut self, heading: i32) {
        if heading < 0 {
            let heading = 360 + heading;
            self.heading = self.heading + heading as u32;

        } else {
            self.heading = self.heading + heading as u32;
        }
        self.heading %= 360;
    }

    fn update_waypoint(&mut self, heading: i32) {
        let mut heading = heading;
        if heading < 0 {
            heading = 360 + heading;
        }
        let heading = heading as f64;

        let east = self.waypoint_pos.east as f64 * heading.to_radians().cos() - self.waypoint_pos.north as f64 * heading.to_radians().sin();
        let north = self.waypoint_pos.north as f64 * heading.to_radians().cos() + self.waypoint_pos.east as f64 * heading.to_radians().sin();
        self.waypoint_pos = Position {east: east.round() as i32, north: north.round() as i32};
    }

    fn forward(&mut self, distance: i32) {
        match self.heading {
            0 => self.current_pos = Position { east: self.current_pos.east, north: self.current_pos.north + distance },
            90 => self.current_pos = Position { east: self.current_pos.east + distance, north: self.current_pos.north },
            180 => self.current_pos = Position { east: self.current_pos.east, north: self.current_pos.north - distance },
            270 => self.current_pos = Position { east: self.current_pos.east - distance, north: self.current_pos.north },
            _ => panic!("Unsupported heading {}", self.heading),
        }
    }

    pub fn step2(&mut self, instr: &Instruction) {
        match instr {
            Instruction::North(distance) => self.waypoint_pos = Position { east: self.waypoint_pos.east, north: self.waypoint_pos.north + distance },
            Instruction::South(distance) => self.waypoint_pos = Position { east: self.waypoint_pos.east, north: self.waypoint_pos.north - distance },
            Instruction::East(distance) => self.waypoint_pos = Position { east: self.waypoint_pos.east + distance, north: self.waypoint_pos.north },
            Instruction::West(distance) => self.waypoint_pos = Position { east: self.waypoint_pos.east - distance, north: self.waypoint_pos.north },
            Instruction::Left(degrees) => self.update_waypoint(*degrees),
            Instruction::Right(degrees) => self.update_waypoint(-1 * degrees),
            Instruction::Forward(distance) => self.current_pos = Position {
                east : self.current_pos.east + self.waypoint_pos.east * distance,
                north : self.current_pos.north + self.waypoint_pos.north * distance,
            },
        }
    }

    pub fn step(&mut self, instr: &Instruction) {
        match instr {
            Instruction::North(distance) => self.current_pos = Position { east: self.current_pos.east, north: self.current_pos.north + distance },
            Instruction::South(distance) => self.current_pos = Position { east: self.current_pos.east, north: self.current_pos.north - distance },
            Instruction::East(distance) => self.current_pos = Position { east: self.current_pos.east + distance, north: self.current_pos.north },
            Instruction::West(distance) => self.current_pos = Position { east: self.current_pos.east - distance, north: self.current_pos.north },
            Instruction::Left(degrees) => self.update_heading(-1 * degrees),
            Instruction::Right(degrees) => self.update_heading(*degrees),
            Instruction::Forward(distance) => self.forward(*distance),
        }
    }

    pub fn get_distance(&self) -> i32 {
        self.current_pos.north.abs() + self.current_pos.east.abs()
    }

    pub fn new() -> Self {
        State {current_pos: Position {north: 0, east: 0}, heading: 90, waypoint_pos: Position {east: 10, north: 1} }
    }

}

fn instructions_from_file(filename: &str) -> Vec<Instruction> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut instructions = Vec::<Instruction>::new();
    for line in reader.lines() {
        let instruction = Instruction::from_str(&line.unwrap()).unwrap();
        instructions.push(instruction);
    }

    instructions
}

fn execute(instructions: &Vec<Instruction>) -> State {
    let mut state = State::new();
    for instr in instructions {
        state.step(instr);
    }
    state
}

fn execute2(instructions: &Vec<Instruction>) -> State {
    let mut state = State::new();
    for instr in instructions {
        state.step2(instr);
    }
    state
}

fn main() {
    let instructions = instructions_from_file("input.txt");
    let state = execute(&instructions);
    println!("First part 1: {}", state.get_distance());

    let state = execute2(&instructions);
    println!("First part 2: {}", state.get_distance());
}

#[cfg(test)]
mod tests {
use crate::{instructions_from_file, execute, execute2};
    #[test]
    fn test_input() {
        let instructions = instructions_from_file("testinput.txt");
        let state = execute(&instructions);
        assert_eq!(state.get_distance(), 25);
    }

    #[test]
    fn test_input2() {
        let instructions = instructions_from_file("testinput.txt");
        let state = execute2(&instructions);
        assert_eq!(state.get_distance(), 286);
    }

}
