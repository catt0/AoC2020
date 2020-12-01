use std::fs::File;
use std::io::{BufRead, BufReader};

fn find_match_for_2(filename: &str) -> u64 {
    let filename = filename;
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut nums = Vec::<u64>::new();
    for line in reader.lines() {
        let num = line.unwrap().parse::<u64>().unwrap();
        nums.push(num);
    }

    for (outerindex, outernum) in nums.iter().enumerate() {
        for (innerindex, innernum) in nums.iter().enumerate() {
            if outernum + innernum == 2020 {
                println!("Found match at outer: {}: {}, inner: {}: {}. Result: {}", outerindex, outernum, innerindex, innernum, outernum*innernum);
                return outernum*innernum;
            }
        }
    }

    panic!("No matching pair found.");
}

fn find_match_for_3(filename: &str) -> u64 {
    let filename = filename;
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut nums = Vec::<u64>::new();
    for line in reader.lines() {
        let num = line.unwrap().parse::<u64>().unwrap();
        nums.push(num);
    }

    for (outerindex, outernum) in nums.iter().enumerate() {
        for (middleindex, middlenum) in nums.iter().enumerate() {
            for (innerindex, innernum) in nums.iter().enumerate() {
                if outernum + innernum + middlenum == 2020 {
                    println!("Found match at outer: {}: {}, middle: {}: {}, inner: {}: {}. Result: {}", outerindex, outernum, middleindex, middlenum, innerindex, innernum, outernum*innernum*middlenum);
                    return outernum*innernum*middlenum;
                }
            }
        }
    }

    panic!("No matching pair found.");
}

fn main() {
    let testmatch = find_match_for_2("testinput.txt");
    println!("Testmatch: {}", testmatch);

    let realmatch = find_match_for_2("input.txt");
    println!("Realmatch for 2: {}", realmatch);

    let realmatch = find_match_for_3("input.txt");
    println!("Realmatch for 3: {}", realmatch);
}

#[cfg(test)]
mod tests {
    use crate::find_match_for_2;
    use crate::find_match_for_3;
    #[test]
    fn it_works_2() {
        let testmatch = find_match_for_2("testinput.txt");
        assert_eq!(testmatch, 514579);
    }

    #[test]
    fn it_works_3() {
        let testmatch = find_match_for_3("testinput.txt");
        assert_eq!(testmatch, 241861950);
    }
}