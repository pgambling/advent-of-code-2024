mod part1;
mod part2;

use part1::count_xmas;
use part2::count_x_mas;
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
    let char_matrix: Vec<Vec<char>> = read_input(file_path)
        .map(|line| line.chars().collect())
        .collect();

    println!("Part1: {}", count_xmas(&char_matrix));
    println!("Part2: {}", count_x_mas(&char_matrix));
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
