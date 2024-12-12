use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide a file path");
        return;
    }
    let file_path = &args[1];
    let safe = read_input(file_path)
        .filter(|line| {
            let nums: Vec<i32> = line
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();

            let mut safe = is_safe(&nums);

            if !safe {
                for i in 0..nums.len() {
                    let mut nums_one_removed = nums.clone();
                    nums_one_removed.remove(i);
                    safe = is_safe(&nums_one_removed);
                    if safe {
                        break;
                    }
                }
            }

            safe
        })
        .count();

    println!("Num Safe: {safe}")
}

fn is_safe(nums: &[i32]) -> bool {
    if nums.len() < 2 {
        return true;
    }
    // Check if the numbers are increasing or decreasing
    let mut decreasing = true;
    let mut increasing = true;
    let mut safe_difference = true;
    for window in nums.windows(2) {
        let current = window[0];
        let next = window[1];
        if current < next {
            decreasing = false;
        }

        if current > next {
            increasing = false;
        }

        let diff = (current - next).abs();
        if diff < 1 || diff > 3 {
            safe_difference = false;
        }
    }

    (decreasing || increasing) && safe_difference
}

fn read_input<P>(filename: P) -> impl Iterator<Item = String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("Failed to read file");
    io::BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
}
