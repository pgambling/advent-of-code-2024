mod part1;
mod part2;

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

    let start = std::time::Instant::now();
    let part1_result = part1::solve(&input);
    let part1_duration = start.elapsed();
    println!("Part1: {} (took {:?})", part1_result, part1_duration);

    let start = std::time::Instant::now();
    let part2_result = part2::solve(&input);
    let part2_duration = start.elapsed();
    println!("Part2: {} (took {:?})", part2_result, part2_duration);
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

#[cfg(test)]
mod tests {
    use super::*;

    mod part1_tests {
        use super::*;

        #[test]
        fn example1() {
            let input = read_input("example1.txt");
            assert_eq!(part1::solve(&input), 140);
        }

        #[test]
        fn example2() {
            let input = read_input("example2.txt");
            assert_eq!(part1::solve(&input), 772);
        }

        #[test]
        fn example3() {
            let input = read_input("example3.txt");
            assert_eq!(part1::solve(&input), 1930);
        }
    }

    mod part2_tests {
        use super::*;

        #[test]
        fn example1() {
            let input = read_input("example1.txt");
            assert_eq!(part2::solve(&input), 80);
        }

        #[test]
        fn example2() {
            let input = read_input("example2.txt");
            assert_eq!(part2::solve(&input), 436);
        }

        #[test]
        fn example3() {
            let input = read_input("example3.txt");
            assert_eq!(part2::solve(&input), 1206);
        }

        #[test]
        fn example4() {
            let input = read_input("example4.txt");
            assert_eq!(part2::solve(&input), 236);
        }

        #[test]
        fn example5() {
            let input = read_input("example5.txt");
            assert_eq!(part2::solve(&input), 368);
        }
    }
}
