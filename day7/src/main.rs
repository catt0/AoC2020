use std::str::FromStr;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;
use lazy_static::lazy_static;
use const_format::formatcp;
use std::collections::HashSet;


#[derive(Debug, PartialEq, Clone, Eq, Hash, Ord, PartialOrd)]
struct Bag {
    color: String,
    modifier: String
}

impl Bag {
    const BAG_REGEX_STR: &'static str = r"(\w+) (\w+) bags?";
}


impl FromStr for Bag {
    type Err = String;

    fn from_str(entry: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref BAG_REGEX : Regex = Regex::new(Bag::BAG_REGEX_STR).unwrap();
        }
        let cap = BAG_REGEX.captures(entry).ok_or(format!("Regex not matched for {}", entry))?;
        let modifier = cap.get(1).unwrap().as_str();
        let color = cap.get(2).unwrap().as_str();

        let modifier = String::from(modifier);
        let color = String::from(color);


        Ok(Bag {color, modifier})
    }
}

#[derive(Debug, PartialEq)]
struct Rule {
    outer_bag: Bag,
    inner_bags: Vec<(u32, Bag)>,
}

impl FromStr for Rule {
    type Err = String;

    fn from_str(entry: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref BAG_REGEX : Regex = Regex::new(Rule::RULE_REGEX_STR).unwrap();
            static ref INNER_BAG_REGEX : Regex = Regex::new(Rule::INNER_RULE_REGEX_STR).unwrap();
        }
        let cap = BAG_REGEX.captures(entry).ok_or(format!("Regex not matched for {}", entry))?;
        let outer_bag = &cap["outer"];
        let outer_bag = Bag::from_str(outer_bag)?;
        let mut inner_bags = Vec::<(u32, Bag)>::new();
        let inner_str = &cap["inners"];
        for cap in INNER_BAG_REGEX.captures_iter(inner_str) {
            let count = u32::from_str(&cap["count"]).unwrap();
            let bag = Bag::from_str(&cap["bag"]).unwrap();
            inner_bags.push((count, bag));
        }
        Ok(Rule {outer_bag, inner_bags})
    }
}

impl Rule {
    const RULE_REGEX_STR: &'static str = formatcp!(r"^(?P<outer>{0}) contain (?P<inners>(?:\d {0}(?:, )?)*)|(?:no other bags)\.$", Bag::BAG_REGEX_STR);
    const INNER_RULE_REGEX_STR: &'static str = formatcp!(r"(?P<count>\d) (?P<bag>{0})", Bag::BAG_REGEX_STR);

    pub fn can_contain(&self, bag: &Bag) -> bool {
        return self.inner_bags.iter().any(|x| &x.1 == bag)
    }
}

fn get_rules_from_file(filename: &str) -> Vec::<Rule> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut entries = Vec::<Rule>::new();
    for line in reader.lines() {
        let entry = Rule::from_str(&line.unwrap()).unwrap();
        entries.push(entry);
    }

    entries 
}

// we use a wrapper to sort and dedup exactly once instead of during recursing
fn get_valid_bags<'a>(rules: &'a Vec<Rule>, bag: &Bag) -> Vec<&'a Bag> {
    let mut ret = get_valid_bags_inner(rules, bag);
    ret.sort();
    ret.dedup();
    ret
}

fn get_valid_bags_inner<'a>(rules: &'a Vec<Rule>, bag: &Bag) -> Vec<&'a Bag> {
    let mut ret = <Vec::<&Bag>>::new();
    let mut seen_bags = HashSet::<&Bag>::new();
    let mut queue = vec![bag];
    while queue.len() > 0 {
        let current = queue.pop().unwrap();
        if seen_bags.contains(current) {
            continue;
        }
        seen_bags.insert(current);

        // this gets all bags that can contain the current bag directly
        let valid_bags = rules.iter().filter_map(|x| 
            if x.can_contain(current) {
                Some(&x.outer_bag)
            } else {
                None
            }).collect::<Vec::<&Bag>>();

        // now loop all not already processed bags to find transitive valid bags
        for bag in valid_bags.iter() {
            if !seen_bags.contains(bag) {
                let valid_bags = get_valid_bags(rules, bag);
                ret.extend(valid_bags);
            }
        }
        ret.extend(valid_bags);
    }
    ret
}

fn get_required_bags(rules: &Vec<Rule>, bag: &Bag) -> u32 {
    let mut ret:u32 = 0;
    let rule = &rules.iter().find(|x| &x.outer_bag == bag).unwrap();
    for (count, bag) in rule.inner_bags.iter() {
        ret += count;
        ret += get_required_bags(rules, bag) * count;
    }
    ret
}

fn main() {
    let rules = get_rules_from_file("input.txt");
    let target_bag = Bag {color: String::from("gold"), modifier: String::from("shiny")};
    let valid_bags = get_valid_bags(&rules, &target_bag);
    println!("Answer for part 1: {}", valid_bags.len());
    let c = get_required_bags(&rules, &target_bag);
    println!("Answer for part 2: {}", c);
}

#[cfg(test)]
mod tests {
use std::str::FromStr;
use crate::{Bag, Rule, get_rules_from_file, get_valid_bags, get_required_bags};
    #[test]
    fn single_bag() {
        let bag = Bag::from_str("light red bags").unwrap();
        assert_eq!(bag.color, "red");
        assert_eq!(bag.modifier, "light");

        let bag = Bag::from_str("dark orange bag").unwrap();
        assert_eq!(bag.color, "orange");
        assert_eq!(bag.modifier, "dark");
    }

    #[test]
    fn single_rule() {
        let _ = Rule::from_str("faded blue bags contain no other bags.").unwrap();
        let _ = Rule::from_str("bright white bags contain 1 shiny gold bag.").unwrap();
        let _ = Rule::from_str("light red bags contain 1 bright white bag, 2 muted yellow bags.").unwrap();
    }

    #[test]
    fn multiple_rules() {
        let rules = get_rules_from_file("testinput.txt");
        assert_eq!(rules.len(), 9);
    }

    #[test]
    fn can_contain() {
        let rule = Rule::from_str("faded blue bags contain no other bags.").unwrap();
        assert_eq!(rule.can_contain(&Bag {color: String::from("red"), modifier: String::from("light")}), false);
        let rule = Rule::from_str("bright white bags contain 1 shiny gold bag.").unwrap();
        assert_eq!(rule.can_contain(&Bag {color: String::from("red"), modifier: String::from("light")}), false);
        assert_eq!(rule.can_contain(&Bag {color: String::from("gold"), modifier: String::from("shiny")}), true);
        let _ = Rule::from_str("light red bags contain 1 bright white bag, 2 muted yellow bags.").unwrap();
    }

    #[test]
    fn stacks() {
        let rules = get_rules_from_file("testinput.txt");
        assert_eq!(rules.len(), 9);
        let test_bag = Bag {color: String::from("gold"), modifier: String::from("shiny")};
        let valid_bags = get_valid_bags(&rules, &test_bag);
        assert_eq!(valid_bags.len(), 4);
    }

    #[test]
    fn bags() {
        let rules = get_rules_from_file("testinput.txt");
        assert_eq!(rules.len(), 9);
        let test_bag = Bag {color: String::from("gold"), modifier: String::from("shiny")};
        let c = get_required_bags(&rules, &test_bag);
        assert_eq!(c, 32);
    }
}
