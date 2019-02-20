use std::collections::HashMap;
use std::io::{self, prelude::*};

// Given a list of integers,
// use a vector and return the mean (the average value),
// median (when sorted, the value in the middle position),
// and mode (the value that occurs most often;
// a hash map will be helpful here) of the list.
fn main() {
    let stdin = io::stdin();

    'mainloop: loop {
        print!("Enter space-separated numbers (q for quit)> ");
        io::stdout().flush().expect("flush failed");

        let mut line = String::new();
        stdin.read_line(&mut line).expect("Failed to read line");

        if line == "q\n" {
            break 'mainloop;
        }

        // TODO: handling ParseIntError
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        let mean = nums.iter().sum::<i32>() as f32 / nums.len() as f32;

        println!(
            "numbers: {:?}, mean: {}, median: {}, mode: {}",
            nums,
            mean,
            get_median(&nums),
            get_mode(&nums)
        );
    }
}

fn get_median(v: &Vec<i32>) -> f32 {
    if v.len() < 1 {
        return 0.0;
    }

    let mut vec = v.clone();
    vec.sort();
    if vec.len() % 2 == 1 {
        return *vec.get(vec.len() / 2).unwrap() as f32;
    }
    return (*vec.get(vec.len() / 2 - 1).unwrap() + *vec.get(vec.len() / 2).unwrap()) as f32 / 2.0;
}

fn get_mode(v: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    for num in v {
        let count = map.entry(num).or_insert(0);
        *count += 1;
    }
    return **map.iter().max_by_key(|(_, v)| *v).unwrap().0;
}
