use std::str::FromStr;
use regex::Regex;
use lazy_static::lazy_static;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq)]
struct PasswordPolicy {
    min: usize,
    max: usize,
    letter: char,
}

impl FromStr for PasswordPolicy {
    type Err = std::num::ParseIntError;

    fn from_str(policy: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref POLICY_REGEX : Regex = Regex::new(r"(\d+)-(\d+) (.)").unwrap();
        }
        let cap = POLICY_REGEX.captures(policy).unwrap();
        let min = usize::from_str(cap.get(1).unwrap().as_str())?;
        let max = usize::from_str(cap.get(2).unwrap().as_str())?;
        let letter = char::from_str(cap.get(3).unwrap().as_str()).unwrap();

        Ok(PasswordPolicy{min, max, letter})
    }

}

#[derive(Debug, PartialEq)]
struct PasswordEntry {
    policy: PasswordPolicy,
    password: String,
}

impl FromStr for PasswordEntry {
    type Err = std::num::ParseIntError;

    fn from_str(entry: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = entry.split(':').collect();
        assert_eq!(parts.len(), 2);
        let policy = PasswordPolicy::from_str(parts[0])?;
        let password = String::from(parts[1].trim());

        Ok(PasswordEntry{policy, password})
    }

}

impl PasswordEntry {
    fn is_valid(&self) -> bool {
        let count = self.password.matches(self.policy.letter).count();
        count >= self.policy.min && count <= self.policy.max
    }

    fn is_valid_two(&self) -> bool {
        let pos1 = self.policy.min;
        let pos2 = self.policy.max;

        let pos1contains = self.password.as_bytes()[pos1 - 1] == self.policy.letter as u8;
        let pos2contains = self.password.as_bytes()[pos2 - 1] == self.policy.letter as u8;

        pos1contains ^ pos2contains
    }
}

fn get_policies_from_file(filename: &str) -> Vec::<PasswordEntry> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut entries = Vec::<PasswordEntry>::new();
    for line in reader.lines() {
        let entry = PasswordEntry::from_str(&line.unwrap()).unwrap();
        entries.push(entry);
    }

    entries 
}

fn main() {
    let testpolicies = get_policies_from_file("testinput.txt");
    println!("Testinput has {} total policies.", testpolicies.len());
    let validcount = testpolicies.iter().filter(|x| x.is_valid()).count();
    println!("Testinput has {} valid policies.", validcount);

    let realpolicies = get_policies_from_file("input.txt");
    println!("Input has {} total policies.", testpolicies.len());
    let validcount = realpolicies.iter().filter(|x| x.is_valid()).count();
    println!("Input has {} valid policies.", validcount);

    let validcount = realpolicies.iter().filter(|x| x.is_valid_two()).count();
    println!("Input has {} valid policies for part two.", validcount);
}

#[cfg(test)]
mod tests {
use std::str::FromStr;
use crate::PasswordPolicy;
use crate::PasswordEntry;
    #[test]
    fn policy_parse() {
        let policy = PasswordPolicy::from_str("1-3 a").unwrap();
        assert_eq!(policy.min, 1);
        assert_eq!(policy.max, 3);
        assert_eq!(policy.letter, 'a');
    }

    #[test]
    fn entry_parse() {
        let entry = PasswordEntry::from_str("1-3 a: abcde").unwrap();
        assert_eq!(entry.policy.min, 1);
        assert_eq!(entry.policy.max, 3);
        assert_eq!(entry.policy.letter, 'a');
        assert_eq!(entry.password, "abcde");
    }

    #[test]
    fn entry_valid() {
        let entry = PasswordEntry::from_str("1-3 a: abcde").unwrap();
        assert_eq!(entry.is_valid(), true);
    }

    #[test]
    fn entry_invalid() {
        let entry = PasswordEntry::from_str("1-3 b: cdefg").unwrap();
        assert_eq!(entry.is_valid(), false);
    }

    #[test]
    fn entry_valid_two() {
        let entry = PasswordEntry::from_str("1-3 a: abcde").unwrap();
        assert_eq!(entry.is_valid_two(), true);
    }

    #[test]
    fn entry_invalid_two() {
        let entry = PasswordEntry::from_str("1-3 b: cdefg").unwrap();
        assert_eq!(entry.is_valid_two(), false);
    }

}

