use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq, Clone)]
enum Square {
    Empty,
    Tree,
    OpenVisisted,
    TreeVisisted,
}

impl Square {
    fn from_char(input: &char) -> Result<Square, ()> {
        match input {
            '.' => Ok(Square::Empty),
            '#' => Ok(Square::Tree),
            'O' => Ok(Square::OpenVisisted),
            'X' => Ok(Square::TreeVisisted),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let res = match self {
            Square::Empty => ".",
            Square::Tree => "#",
            Square::OpenVisisted => "O",
            Square::TreeVisisted => "X",
        };
        write!(f, "{}", res)
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Hill {
    squares: Vec<Vec<Square>>,
}

impl Hill {
    pub fn from_file(filename: &str) -> Self {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        // first determine dimensions lines x linelength
        let mut squares = Vec::<Vec::<Square>>::new();
        for line in reader.lines() {
            let line_vec : Vec<Square> = line.unwrap().chars().map(|c| Square::from_char(&c).unwrap()).collect();
            if squares.len() > 0 {
                assert!(line_vec.len() == squares[0].len(), "Line length mismatch.");
            }
            if line_vec.len() > 0 {
                squares.push(line_vec);
            }
        }

        Hill {squares}
    }
}

#[derive(Debug, PartialEq)]
struct Solution {
    hill: Hill,
    trees_hit: usize,
}

impl Solution {
    pub fn from_hill(mut hill: Hill, right: u32, down: u32) -> Self {
        let mut trees_hit: usize = 0;
        let mut current_right : usize = 0;
        let mut current_down : usize = 0;
        loop {
            current_down += down as usize;
            current_right += right as usize;
            current_right %= hill.squares[0].len();
            // hit the end or went over it, so we are done
            if current_down >= hill.squares.len() {
                break;
            }

            if hill.squares[current_down][current_right] == Square::Empty {
                hill.squares[current_down][current_right] = Square::OpenVisisted;
            } else if hill.squares[current_down][current_right] == Square::Tree {
                hill.squares[current_down][current_right] = Square::TreeVisisted;
                trees_hit += 1;
            } else {
                panic!("Unexpected type");
            }
        };

        Solution {hill, trees_hit}
    }
}

fn main() {
    println!("Hello, world!");

    let hill = Hill::from_file("input.txt");
    let solution = Solution::from_hill(hill.clone(), 3, 1);
    println!("Hit {} trees for the first task.", solution.trees_hit);

    let to_try = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut res = 1;
    for (right, down) in to_try {
        let sol = Solution::from_hill(hill.clone(), right, down);
        println!("For {}, {} hit {} trees.", right, down, sol.trees_hit);
        res *= sol.trees_hit;
    }

    println!("Part 2 res: {}", res);
}

#[cfg(test)]
mod tests {
    use crate::Hill;
    use crate::Square;
    use crate::Solution;
    #[test]
    fn gen_single_line() {
        let hill = Hill::from_file("singlelineinput.txt");
        assert_eq!(hill.squares.len(), 1);
        assert_eq!(hill.squares[0], vec![Square::Empty, Square::Empty, Square::Tree, Square::Empty, Square::Empty, Square::Tree, Square::Empty]);
    }

    #[test]
    fn test_input() {
        let hill = Hill::from_file("testinput.txt");
        let solution = Solution::from_hill(hill, 3, 1);
        assert_eq!(solution.trees_hit, 7);
    }


}