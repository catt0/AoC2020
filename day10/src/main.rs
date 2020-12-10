use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::collections::HashMap;
use std::cmp::min;
use num_bigint::BigUint;
use num_traits::{Zero, One};

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

fn count_jumps(mut numbers: Vec<i64>) -> (u32, u32) {
    numbers.sort();
    let mut last = 0;
    let mut single_jumps: u32 = 0;
    let mut triple_jumps: u32 = 1;
    for num in numbers {
        if num - last == 1 {
            single_jumps += 1;
        } else if num - last == 3 {
            triple_jumps += 1;
        } else {
            panic!("Nope");
        }
        last = num;
    }

    (single_jumps, triple_jumps)
}

type Cache = HashMap<i64, BigUint>;

fn get_combos(mut numbers: Vec<i64>) -> BigUint {
    let cache = Cache::new();
    numbers.push(0);
    numbers.sort();
    numbers.push(numbers.iter().max().unwrap()+3);
    get_combos_inner(&numbers, cache).0
}

fn get_combos_partitioned(mut numbers: Vec<i64>) -> BigUint {
    let mut cache = Cache::new();
    numbers.push(0);
    numbers.sort();
    numbers.push(numbers.iter().max().unwrap()+3);
    let chunk_size: usize = 10000;
    let mut counter = numbers.len() / chunk_size;
    loop {
        // let top = min((counter + 1) * chunk_size, numbers.len());
        let bottom = counter * chunk_size;
        let (_, u) = get_combos_inner(&numbers[bottom..], cache);
        cache = u;
        if counter == 0 {
            break;
        }
        counter -= 1;
    }
    get_combos_inner(&numbers, cache).0
}

fn get_combos_inner(numbers: &[i64], cache: Cache) -> (BigUint, Cache) {
    if numbers.len() == 0 {
        return (One::one(), cache);
    }
    if numbers.len() == 1 {
        return (One::one(), cache);
    }
    if let Some(v) = cache.get(&numbers[0]) {
        return (v.clone(), cache);
    }

    // our data has no duplicates, so we can just test the first three numbers in the vector to check if they match
    // we then cut off the tested values and pass the slice to the next iteration
    
    // first entry is always valid
    let (mut ret, mut cache) = get_combos_inner(&numbers[1..], cache);
    // check if the next number is also valid
    if numbers.len() >= 3 && numbers[2] - numbers[0] <= 3 {
        let (t, u) = get_combos_inner(&numbers[2..], cache);
        ret += t;
        cache = u;
    }
    if numbers.len() >= 4 && numbers[3] - numbers[0] <= 3 {
        let (t, u) = get_combos_inner(&numbers[3..], cache);
        ret += t;
        cache = u;
    }

    // update the cache
    cache.insert(numbers[0], ret.clone());

    (ret, cache)
}

fn main() {
    let nums = get_numbers_from_file("input.txt");
    let (single, triple) = count_jumps(nums.clone());
    println!("Result for part 1: {}", single * triple);
    let combos = get_combos_partitioned(nums);
    println!("Result for part 2: {}", combos);

    let nums = get_numbers_from_file("more.txt");
    let combos = get_combos_partitioned(nums);
    println!("Result for part 3: {}", combos);
}

#[cfg(test)]
mod tests {
    use crate::{get_numbers_from_file, count_jumps, get_combos};
    #[test]
    fn test_data() {
        let nums = get_numbers_from_file("testinput.txt");
        let (single, triple) = count_jumps(nums.clone());
        assert_eq!(single, 22);
        assert_eq!(triple, 10);
        let combos = get_combos(nums);
        // assert_eq!(combos, 19208);
    }

}