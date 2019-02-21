use std::collections::HashMap;
use std::io::{self, Write};
use std::num::ParseIntError;

// Given a list of integers,
// use a vector and return the mean (the average value),
// median (when sorted, the value in the middle position),
// and mode (the value that occurs most often;
// a hash map will be helpful here) of the list.
fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut line = String::new();

    loop {
        print!("Enter space-separated numbers (q for quit)> ");
        stdout.flush().unwrap();

        line.clear();
        stdin.read_line(&mut line).unwrap();

        if line == "q\n" {
            break;
        }

        let mut nums = match line
            .split_whitespace()
            .map(|s| s.parse::<i32>())
            .collect::<Result<Vec<i32>, ParseIntError>>()
        {
            Ok(vec) => {
                if vec.is_empty() {
                    continue;
                }
                vec
            }
            Err(_) => {
                println!("Invalid input. Only integer values are accepted. Please try again.");
                continue;
            }
        };

        let mean = nums.iter().sum::<i32>() as f32 / nums.len() as f32;
        let median = get_median(&mut nums);
        let mode = get_mode(&nums);

        println!(
            "numbers: {:?}, mean: {}, median: {}, mode: {:?}",
            &nums, mean, median, mode,
        );
    }
}

fn get_median(vec: &mut Vec<i32>) -> f32 {
    if vec.is_empty() {
        return 0.0;
    }

    vec.sort();

    let mid_idx = vec.len() / 2;
    if vec.len() % 2 == 1 {
        vec[mid_idx] as f32
    } else {
        (vec[mid_idx - 1] + vec[mid_idx]) as f32 / 2.0
    }
}

fn get_mode(slice: &[i32]) -> Vec<i32> {
    if slice.is_empty() {
        return vec![];
    }

    let mut map = HashMap::with_capacity(slice.len());
    for num in slice {
        let count = map.entry(num).or_insert(0);
        *count += 1;
    }

    let max_val = map.values().map(|v| *v).max().unwrap();

    let mut vec = map
        .into_iter()
        .filter_map(|(k, v)| if v == max_val { Some(*k) } else { None })
        .collect::<Vec<i32>>();

    vec.sort();

    vec
}
