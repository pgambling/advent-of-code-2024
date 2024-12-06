mod part1;

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
    let input = read_input(file_path);

    println!("Part1: {}", part1::solve(&input));
}

fn read_input<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("Failed to read file");
    io::BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .collect()
}
