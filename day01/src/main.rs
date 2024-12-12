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
    let mut array1: Vec<u32> = Vec::new();
    let mut array2: Vec<u32> = Vec::new();
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            let nums = line
                .expect("Failed to read line")
                .split_whitespace()
                .map(|s| s.parse().expect("Invalid number"))
                .collect::<Vec<u32>>();
            array1.push(nums[0]);
            array2.push(nums[1]);
        }
    }
    array1.sort();
    array2.sort();
    let mut distance: u32 = 0;
    let mut similarity: u32 = 0;
    for (num1, num2) in array1.iter().zip(array2.iter()) {
        distance += num1.abs_diff(*num2);
        similarity += num1 * array2.iter().filter(|&x| x == num1).count() as u32;
    }
    println!("Total distance: {distance}");
    println!("Similarity score: {similarity}")
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
