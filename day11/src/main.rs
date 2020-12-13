use std::str::FromStr;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashSet, HashMap};
use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
#[allow(non_snake_case)]
enum Square {
    Floor,
    EmptySeat,
    TakenSeat,
}


impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Square::Floor => {write!(f, ".")}
            Square::EmptySeat => {write!(f, "L")}
            Square::TakenSeat => {write!(f, "#")}
        }
    }
}

impl Square {
    fn from_char(s: &char) -> Result<Square, String> {
        match s {
            'L' => Ok(Square::EmptySeat),
            '#' => Ok(Square::TakenSeat),
            '.' => Ok(Square::Floor),
            _ => Err(format!("Unrecognized state {}", s)),
        }
    }
}

#[derive(Debug, PartialEq)]
enum IteratorState {
    None,
    UpLeft,
    Up,
    UpRight,
    Left,
    Right,
    DownLeft,
    Down,
    DownRight
}

impl IteratorState {
    pub fn next(&self) -> Option<Self> {
        match self {
            IteratorState::None => Some(IteratorState::UpLeft),
            IteratorState::UpLeft => Some(IteratorState::Up),
            IteratorState::Up => Some(IteratorState::UpRight),
            IteratorState::UpRight => Some(IteratorState::Left),
            IteratorState::Left => Some(IteratorState::Right),
            IteratorState::Right => Some(IteratorState::DownLeft),
            IteratorState::DownLeft => Some(IteratorState::Down),
            IteratorState::Down => Some(IteratorState::DownRight),
            IteratorState::DownRight => None,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[allow(non_snake_case)]
fn Point(x: usize, y: usize) -> Point {
    Point {x: x as i32, y: y as i32}
}

#[allow(non_snake_case)]
fn PointS(x: i32, y: i32) -> Point {
    Point {x, y}
}

#[derive(Debug, PartialEq)]
struct NeighborIterator {
    boundx: usize,
    boundy: usize,
    current_state: IteratorState,
    reference_pos: Point,
}

impl NeighborIterator {
    pub fn new(boundx: usize, boundy: usize, reference_pos: Point) -> Self {
        NeighborIterator{boundx, boundy, current_state: IteratorState::None, reference_pos}
    }

    pub fn pos_for(&self, pos: &IteratorState) -> Option<Point> {
        match pos {
            IteratorState::None => {None}
            IteratorState::UpLeft => {self.check_valid(PointS(self.reference_pos.x - 1, self.reference_pos.y - 1))}
            IteratorState::Up => {self.check_valid(PointS(self.reference_pos.x, self.reference_pos.y - 1))}
            IteratorState::UpRight => {self.check_valid(PointS(self.reference_pos.x + 1, self.reference_pos.y - 1))}
            IteratorState::Left => {self.check_valid(PointS(self.reference_pos.x - 1, self.reference_pos.y))}
            IteratorState::Right => {self.check_valid(PointS(self.reference_pos.x + 1, self.reference_pos.y))}
            IteratorState::DownLeft => {self.check_valid(PointS(self.reference_pos.x - 1, self.reference_pos.y + 1))}
            IteratorState::Down => {self.check_valid(PointS(self.reference_pos.x, self.reference_pos.y + 1))}
            IteratorState::DownRight => {self.check_valid(PointS(self.reference_pos.x + 1, self.reference_pos.y + 1))}
        }
    }

    pub fn check_valid(&self, pos: Point) -> Option<Point> {
        if pos.x >= 0 && pos.x < self.boundx as i32
            && pos.y >= 0 && pos.y < self.boundy as i32 {
                Some(pos)
            } else {
                None
            }
    }
}

impl Iterator for NeighborIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Point> {
        while let Some(next) = self.current_state.next() {
            self.current_state = next;
            let t = self.pos_for(&self.current_state);
            if t.is_some() {
                return t;
            }
        };

        None
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
struct State {
    seats: Vec<Vec<Square>>,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.seats {
            for seat in row {
                write!(f, "{}", seat)?;
            }
            write!(f, "\n")?;
        }
        write!(f, "")
    }
}

impl State {
    pub fn from_file(filename: &str) -> Self {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        // first determine dimensions lines x linelength
        let mut seats = Vec::<Vec::<Square>>::new();
        for line in reader.lines() {
            let line_vec : Vec<Square> = line.unwrap().chars().map(|c| Square::from_char(&c).unwrap()).collect();
            if seats.len() > 0 {
                assert!(line_vec.len() == seats[0].len(), "Line length mismatch.");
            }
            if line_vec.len() > 0 {
                seats.push(line_vec);
            }
        }

        State {seats}
    }

    pub fn step(&self) -> Self {
        let mut ret = self.clone();

        for y in 0..self.seats.len() {
            for x in 0..self.seats[y].len() {
                let seat = self.seats[y][x];
                if seat == Square::Floor {
                    continue;
                }
                let neighbors = self.get_neighbors(Point(x, y));
                let occupied_count = neighbors.iter().filter(|x| **x == Square::TakenSeat).count();
                if seat == Square::TakenSeat && occupied_count >= 4 {
                    ret.seats[y][x] = Square::EmptySeat;
                } else if seat == Square::EmptySeat && occupied_count == 0 {
                    ret.seats[y][x] = Square::TakenSeat;
                };
            }
        }

        ret
    }

    pub fn step2(&self) -> Self {
        let mut ret = self.clone();

        for y in 0..self.seats.len() {
            for x in 0..self.seats[y].len() {
                let seat = self.seats[y][x];
                if seat == Square::Floor {
                    continue;
                }
                let neighbors = self.get_visible_seats(Point(x, y));
                let occupied_count = neighbors.iter().filter(|x| **x == Square::TakenSeat).count();
                if seat == Square::TakenSeat && occupied_count >= 5 {
                    ret.seats[y][x] = Square::EmptySeat;
                } else if seat == Square::EmptySeat && occupied_count == 0 {
                    ret.seats[y][x] = Square::TakenSeat;
                };
            }
        }

        ret
    }

    pub fn get_neighbors(&self, pos: Point) -> Vec<Square> {
        let mut it = NeighborIterator::new(self.seats.first().unwrap().len(), self.seats.len(), pos);
        let ret = it.into_iter().map(|pos| &self.seats[pos.y as usize][pos.x as usize]).copied().collect();
        ret
    }

    pub fn count_occupied(&self) -> u32 {
        let mut ret: u32 = 0;
        for row in &self.seats {
            for seat in row {
                if seat == &Square::TakenSeat {
                    ret += 1;
                }
            }
        }

        ret
    }

    pub fn run_to_stable(&self) -> Self {
        let mut seats = self.clone();
        let mut old_seats = State::default();
        loop {
            old_seats = seats;
            seats = old_seats.step();
            if old_seats == seats {
                break;
            }
        }

        seats
    }

    pub fn run_to_stable2(&self) -> Self {
        let mut seats = self.clone();
        let mut old_seats = State::default();
        loop {
            old_seats = seats;
            seats = old_seats.step2();
            if old_seats == seats {
                break;
            }
        }

        seats
    }

    pub fn raytrace_for_seat(&self, start_pos: Point, direction: &IteratorState) -> Option<Square> {
        let mut current_pos = start_pos;
        while let Some(next_pos) = NeighborIterator::new(self.seats.first().unwrap().len(), self.seats.len(), current_pos).pos_for(direction) {
            let square = self.seats[next_pos.y as usize][next_pos.x as usize];
            if square != Square::Floor {
                return Some(square);
            }
            current_pos = next_pos;
        };
        None
    }

    pub fn get_visible_seats(&self, pos: Point) -> Vec<Square> {
        let mut ret: Vec<Square> = Vec::new();
        let mut it_state = IteratorState::None;
        while let Some(next_it) = it_state.next() {
            if let Some(seat) = self.raytrace_for_seat(pos, &next_it) {
                ret.push(seat);
            }
            it_state = next_it;
        }

        ret
    }
}

fn main() {
    let testseats = State::from_file("testinput.txt");
    println!("Initial:\n{}", testseats);
    let mut tstate = testseats.clone();
    for i in 1..5 {
        tstate = tstate.step();
        println!("Step {}:\n{}", i, tstate);
    }
    
    let seats = State::from_file("input.txt");
    let finished = seats.run_to_stable();
    println!("Solution part 1: {}", finished.count_occupied());

    let seats = State::from_file("input.txt");
    let finished = seats.run_to_stable2();
    println!("Solution part 2: {}", finished.count_occupied());
}

#[cfg(test)]
mod tests {
    use crate::State;
    #[test]
    fn test_input() {
        let seats = State::from_file("testinput.txt");
        let finished = seats.run_to_stable();
        assert_eq!(finished.count_occupied(), 37);
    }

    #[test]
    fn test_input2() {
        let seats = State::from_file("testinput.txt");
        let finished = seats.run_to_stable2();
        assert_eq!(finished.count_occupied(), 26);
    }



}
