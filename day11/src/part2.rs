// use cached::proc_macro::cached;

const BLINKS: usize = 25;

pub fn solve(input: &[String]) -> usize {
    load_input(input)
        .iter()
        .map(|stone| count_stones(*stone, BLINKS))
        .sum()
}
// #[cached]
fn count_stones(stone: usize, blinks: usize) -> usize {
    let mut stones = vec![stone];
    for _ in 0..blinks {
        let len = stones.len();
        for index in 0..len {
            let stone = stones[index];
            if stone == 0 {
                stones[index] = 1;
            } else if has_even_digits(stone) {
                let (left, right) = split_even_digit_number(stone);
                stones[index] = left;
                stones.push(right);
            } else {
                stones[index] = stone * 2024;
            }
        }
    }
    stones.len()
}

fn has_even_digits(n: usize) -> bool {
    ((n as f64).log10().floor() as usize + 1) % 2 == 0
}

fn split_even_digit_number(n: usize) -> (usize, usize) {
    let num_digits = (n as f64).log10().floor() as u32 + 1;
    let divisor = 10_usize.pow(num_digits / 2);

    (n / divisor, n % divisor)
}

fn load_input(input: &[String]) -> Vec<usize> {
    input[0]
        .split_whitespace()
        .filter_map(|n| n.parse().ok())
        .collect()
}
