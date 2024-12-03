use regex::Regex;
use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide a file path");
        return;
    }
    let file_path = &args[1];

    let input: String = fs::read_to_string(file_path).expect("Failed to read file");

    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let result: i32 = re
        .captures_iter(&input)
        .map(|caps| {
            let (_, [multiplicand, multiplier]) = caps.extract();
            let multiplicand: i32 = multiplicand.parse().unwrap();
            let multiplier: i32 = multiplier.parse().unwrap();
            multiplicand * multiplier
        })
        .sum();

    println!("Result: {result}")
}
