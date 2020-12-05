use std::str::FromStr;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq, Default)]
struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: String,
}

impl FromStr for Passport {
    type Err = String;

    fn from_str(entry: &str) -> Result<Self, Self::Err> {
        let mut ret = Passport::default();
        let parts: Vec<&str> = entry.trim().split(' ').collect();
        for part in parts.iter() {
            let key = &part[0..3];
            let val = &part[4..];
            match key {
                "byr" => ret.byr = String::from(val),
                "iyr" => ret.iyr = String::from(val),
                "eyr" => ret.eyr = String::from(val),
                "hgt" => ret.hgt = String::from(val),
                "hcl" => ret.hcl = String::from(val),
                "ecl" => ret.ecl = String::from(val),
                "pid" => ret.pid = String::from(val),
                "cid" => ret.cid = String::from(val),
                _ => return Err(format!("Unexpected value {}.", key)),
            }
        }

        Ok(ret)
    }

}

impl Passport {
    fn is_valid(&self) -> bool {
        self.byr.len() > 0
        && self.iyr.len() > 0
        && self.eyr.len() > 0
        && self.hgt.len() > 0
        && self.hcl.len() > 0
        && self.ecl.len() > 0
        && self.pid.len() > 0
        // && self.cid.len() > 0  // ignored for task 1
    }

    fn is_valid_and_sane(&self) -> bool {
        if !self.is_valid() {
            return false;
        }

        let byr = u32::from_str(&self.byr).unwrap_or(0);
        if byr < 1920 || byr > 2002 {
            return false;
        }

        let iyr = u32::from_str(&self.iyr).unwrap_or(0);
        if iyr < 2010 || iyr > 2020 {
            return false;
        }

        let eyr = u32::from_str(&self.eyr).unwrap_or(0);
        if eyr < 2020 || eyr > 2030 {
            return false;
        }

        let unit = &self.hgt[self.hgt.len() - 2..];
        if unit == "cm" {
            let hgt = u32::from_str(&self.hgt[0..3]).unwrap_or(0);
            if hgt < 150 || hgt > 193 {
                return false;
            }
        } else if unit == "in" {
            let hgt = u32::from_str(&self.hgt[0..2]).unwrap_or(0);
            if hgt < 59 || hgt > 76 {
                return false;
            }
        } else {
            return false;
        }

        if &self.hcl[0..1] != "#" {
            return false;
        }
        let hcl = u32::from_str_radix(&self.hcl[1..], 16);
        if hcl.is_err() {
            return false;
        }

        let valid_ecls = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        if !valid_ecls.iter().any(|x| x == &self.ecl) {
            return false;
        }

        if self.pid.len() != 9 || u32::from_str(&self.pid).is_err() {
            return false;
        }

        true
    }
}

fn passports_from_file(filename: &str) -> Vec<Passport> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut passports = Vec::<Passport>::new();
    let mut current_passport_data = String::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            // end of passport entry, generate new passport
            if current_passport_data.len() > 0 {
                let passport = Passport::from_str(&current_passport_data).unwrap();
                passports.push(passport);
                current_passport_data = String::new();
            }

            continue;
        }

        current_passport_data += line.trim();
        current_passport_data += " ";
    }

    if current_passport_data.len() > 0 {
        let passport = Passport::from_str(&current_passport_data).unwrap();
        passports.push(passport);
    }

    passports
}

fn main() {
    println!("Hello, world!");
    let passports = passports_from_file("input.txt");
    let valid_count = passports.iter().filter(|p| p.is_valid()).count();
    println!("Valid for part 1: {}", valid_count);

    let valid_count = passports.iter().filter(|p| p.is_valid_and_sane()).count();
    println!("Valid for part 2: {}", valid_count);
}


#[cfg(test)]
mod tests {
use crate::Passport;
use crate::passports_from_file;
    #[test]
    fn single_passport() {
        let passports = passports_from_file("singlepassport.txt");
        assert_eq!(passports.len(), 1);
        assert_eq!(passports[0], Passport {
            hcl: String::from("#ae17e1"),
            iyr: String::from("2013"),
            eyr: String::from("2024"),
            ecl: String::from("brn"),
            pid: String::from("760753108"),
            byr: String::from("1931"),
            hgt: String::from("179cm"),
            cid: String::from(""),
        })
    }

    #[test]
    fn test_input() {
        let passports = passports_from_file("testinput.txt");
        assert_eq!(passports.len(), 4);
        let valid_count = passports.iter().filter(|p| p.is_valid()).count();
        assert_eq!(valid_count, 2);
    }

    #[test]
    fn invalid_passports() {
        let passports = passports_from_file("invalidpassports.txt");
        assert_eq!(passports.len(), 4);
        let valid_count = passports.iter().filter(|p| p.is_valid_and_sane()).count();
        assert_eq!(valid_count, 0);
    }

    #[test]
    fn valid_passports() {
        let passports = passports_from_file("validpassports.txt");
        assert_eq!(passports.len(), 4);
        let valid_count = passports.iter().filter(|p| p.is_valid_and_sane()).count();
        assert_eq!(valid_count, 4);
    }


}
