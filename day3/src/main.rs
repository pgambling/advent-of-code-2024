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

    let re = Regex::new(r"(mul)\(([0-9]{1,3}),([0-9]{1,3})\)|do\(\)|don't\(\)").unwrap();
    let mut mul_enabled = true;
    let result: i32 = re
        .captures_iter(&input)
        .map(|caps| {
            return match caps.get(1) {
                Some(_) => {
                    let multiplicand: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
                    let multiplier: i32 = caps.get(3).unwrap().as_str().parse().unwrap();
                    if mul_enabled {
                        multiplicand * multiplier
                    } else {
                        0
                    }
                }
                None => {
                    match caps.get(0).unwrap().as_str() {
                        "do()" => mul_enabled = true,
                        "don't()" => mul_enabled = false,
                        _ => unreachable!(),
                    }
                    0
                }
            };
        })
        .sum();

    println!("Result: {result}")
}
