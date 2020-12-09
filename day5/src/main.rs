use std::str::FromStr;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp::Ordering;

#[derive(Debug, Eq)]
struct Seat {
    row: u32,
    column: u32,
}

impl Ord for Seat {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_id().cmp(&other.get_id())
    }
}

impl PartialOrd for Seat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.get_id().cmp(&other.get_id()))
    }
}

impl PartialEq for Seat {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

fn binary_search(bounds: (u32, u32), lower_symbol: char, upper_symbol: char, spec: &str) -> u32 {
    let mut current_bounds = bounds;
    
    for c in spec.chars() {
        let (lower, upper) = current_bounds;
        assert!(upper > lower, "Upper bound is somehow lower than lower bound. Lower: {}, upper: {}", lower, upper);

        let diff = upper - lower;
        let step = diff / 2 + diff % 2; // round up
        if c == lower_symbol {
            current_bounds = (lower, upper - step);
        } else if c == upper_symbol {
            current_bounds = (lower + step, upper);
        } else {
            panic!("Found invalid char {}.", c);
        }
    }

    let (lower, upper) = current_bounds;
    if upper == lower {
        return lower;
    }
    panic!("Did not terminat for spec {}.", spec);
}

impl FromStr for Seat {
    type Err = String;

    fn from_str(entry: &str) -> Result<Self, Self::Err> {
        if entry.len() != 10 {
            return Err(format!("Invalid seat specification {}.", entry));
        };

        let row = &entry[0..7];
        let column = &entry[7..10];
        let row = binary_search((0, 127), 'F', 'B', row);
        let column = binary_search((0, 7), 'L', 'R', column);
        
        Ok(Seat {row, column})
    }

}

impl Seat {
    pub fn get_id(&self) -> u32 {
        self.row * 8 + self.column
    }
}

fn seats_from_file(filename: &str) -> Vec<Seat> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    // first determine dimensions lines x linelength
    let mut seats = Vec::<Seat>::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if line.len() < 1 {
            continue;
        }
        let seat = Seat::from_str(&line).unwrap();
        seats.push(seat);
    }

    seats
}

fn main() {
    let mut seats = seats_from_file("input.txt");
    seats.sort();
    let max = &seats.last().unwrap();
    println!("Max for 1: {}", max.get_id());
    let min = &seats.first().unwrap();
    let mut last = min.get_id();
    for seat in seats {
        if seat.get_id() - last > 1 {
            println!("Your seat is {}", seat.get_id() - 1);
        }
        last = seat.get_id();
    }
}

#[cfg(test)]
mod tests {
use crate::Seat;
use std::str::FromStr;
    #[test]
    fn single_seats() {
        let seats = [("FBFBBFFRLR", 44, 5, 357), ("BFFFBBFRRR", 70, 7, 567), ("FFFBBBFRRR", 14, 7, 119), ("BBFFBBFRLL", 102, 4, 820)];
        for &(seat_str, row, column, id) in seats.iter() {
            let seat = Seat::from_str(seat_str).unwrap();
            assert_eq!(seat.row, row);
            assert_eq!(seat.column, column);
            assert_eq!(seat.get_id(), id);
        }
    }
}
