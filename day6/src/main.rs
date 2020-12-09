use std::str::FromStr;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

#[derive(Debug, PartialEq)]
struct Group {
    persons: Vec<Person>,
}

impl Group {
    pub fn from(persons: Vec<Person>) -> Self {
        Group {persons}
    }

    pub fn get_unique_answers(&self) -> HashSet<char> {
        let ret = self.persons.iter().fold(HashSet::<char>::new(), |acc, x| acc.union(&x.yes_answers).cloned().collect::<HashSet::<char>>());
        ret
    }

    pub fn get_all_answers(&self) -> HashSet<char> {
        let ret = self.persons.iter().fold(HashSet::<char>::from(self.persons.first().unwrap().yes_answers.iter().cloned().collect()), |acc, x| acc.intersection(&x.yes_answers).cloned().collect::<HashSet::<char>>());
        ret
    }
}

#[derive(Debug, PartialEq)]
struct Person {
    pub yes_answers: HashSet<char>,
}

impl FromStr for Person {
    type Err = String;

    fn from_str(entry: &str) -> Result<Self, Self::Err> {
        let mut yes_answers = HashSet::<char>::new();
        for c in entry.chars() {
            yes_answers.insert(c);
        }

        Ok(Person {yes_answers})
    }
}
fn groups_from_file(filename: &str) -> Vec<Group> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut groups = Vec::<Group>::new();
    let mut current_group_data = Vec::<Person>::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            // end of passport entry, generate new passport
            if current_group_data.len() > 0 {
                let group = Group::from(current_group_data);
                groups.push(group);
                current_group_data = Vec::<Person>::new();
            }

            continue;
        }

        current_group_data.push(Person::from_str(line.trim()).unwrap());
    }

    if current_group_data.len() > 0 {
        let group = Group::from(current_group_data);
        groups.push(group);
    }

    groups
}

fn main() {
    let groups = groups_from_file("input.txt");
    let num_unique_answers = groups.iter().fold(0, |acc, x| acc + &x.get_unique_answers().len());
    println!("Answer part 1: {}", num_unique_answers);
    let num_all_answers = groups.iter().fold(0, |acc, x| acc + &x.get_all_answers().len());
    println!("Answer part 2: {}", num_all_answers);
    }

#[cfg(test)]
mod tests {
use crate::groups_from_file;
#[test]
    fn test_input() {
        let groups = groups_from_file("testinput.txt");
        assert_eq!(groups.len(), 5);
        let num_unique_answers = groups.iter().fold(0, |acc, x| acc + &x.get_unique_answers().len());
        assert_eq!(num_unique_answers, 11);
        let num_all_answers = groups.iter().fold(0, |acc, x| acc + &x.get_all_answers().len());
        assert_eq!(num_all_answers, 6);
    }
}
