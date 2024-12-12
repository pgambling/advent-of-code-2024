use cached::proc_macro::cached;

const BLINKS: usize = 75;

pub fn solve(input: &[String]) -> usize {
    load_input(input)
        .iter()
        .map(|stone| count_stones(*stone, 0, BLINKS))
        .sum()
}
#[cached]
fn count_stones(stone: usize, current_blink: usize, total_blinks: usize) -> usize {
    let mut count = 1;
    let mut stone = stone;
    let mut current_blink = current_blink;
    while current_blink < total_blinks {
        current_blink += 1;
        if stone == 0 {
            stone = 1;
        } else if !has_even_digits(stone) {
            stone = stone * 2024;
        } else {
            let (left, right) = split_even_digit_number(stone);
            stone = left;
            count += count_stones(right, current_blink, total_blinks);
        }
    }
    count
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
