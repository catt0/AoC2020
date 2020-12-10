use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn get_numbers_from_file(filename: &str) -> Vec::<i64> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut entries = Vec::<i64>::new();
    for line in reader.lines() {
        let num = i64::from_str(&line.unwrap()).unwrap();
        entries.push(num);
    }

    entries 
}

fn find_first_not_matching(numbers: &Vec<i64>, window_length: u32) -> i64 {
    'outer: for i in 0..(numbers.len() - window_length as usize - 1) {
        let part = &numbers[i..i+window_length as usize];
        let target = numbers[i+window_length as usize];
        for (xpos, x) in part.iter().enumerate() {
            for (ypos, y) in part.iter().enumerate() {
                if xpos == ypos {
                    continue;
                }
                if x + y == target {
                    continue 'outer;
                }
            }
        }
        return target;
    }
    panic!("No solution.");
}

fn find_range_sum(numbers: &Vec<i64>, target: i64) -> i64 {
    for lowerpos in 0..numbers.len() {
        let mut sum = numbers[lowerpos];
        for upperpos in lowerpos+1..numbers.len() {
            sum += numbers[upperpos];
            if sum == target {
                let part = &numbers[lowerpos..upperpos+1];
                let min = part.iter().min().unwrap();
                let max = part.iter().max().unwrap();
                return min + max;
            }
        }
    }
    panic!("No solution!");
}

fn main() {
    let nums = get_numbers_from_file("input.txt");
    let res = find_first_not_matching(&nums, 25);
    println!("Result part 1: {}", res);
    let sum = find_range_sum(&nums, res);
    println!("Result part 2: {}", sum);
}

#[cfg(test)]
mod tests {
    use crate::{find_first_not_matching, get_numbers_from_file, find_range_sum};
    #[test]
    fn test_data() {
        let nums = get_numbers_from_file("testinput.txt");
        let res = find_first_not_matching(&nums, 5);
        assert_eq!(res, 127);
        let sum = find_range_sum(&nums, res);
        assert_eq!(sum, 62)
    }

}