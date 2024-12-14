use itertools::Itertools;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Position(isize, isize);

type FrequencyMap = HashMap<char, Vec<Position>>;

pub fn solve(input: &[String]) -> usize {
    let (frequency_map, max_x, max_y) = load_input(input);
    frequency_map
        .iter()
        .flat_map(|(_, positions)| calc_antinodes(&positions, max_x, max_y))
        .unique()
        .count()
}

fn calc_antinodes(positions: &[Position], max_x: isize, max_y: isize) -> Vec<Position> {
    let mut antinodes = vec![];
    for i in 0..positions.len() {
        for j in (i + 1)..positions.len() {
            let (pos1, pos2) = (positions[i], positions[j]);
            let (dx, dy) = (pos2.0 - pos1.0, pos2.1 - pos1.1);
            antinodes.extend(get_antinodes_in_range(&pos1, -dx, -dy, max_x, max_y));
            antinodes.extend(get_antinodes_in_range(&pos2, dx, dy, max_x, max_y));
        }
    }
    antinodes
}

fn get_antinodes_in_range(
    pos: &Position,
    dx: isize,
    dy: isize,
    max_x: isize,
    max_y: isize,
) -> Vec<Position> {
    let mut antinodes = vec![];
    let (mut x, mut y) = (pos.0, pos.1);
    while x >= 0 && y >= 0 && x < max_x && y < max_y {
        antinodes.push(Position(x, y));
        x += dx;
        y += dy;
    }
    antinodes
}

fn load_input(input: &[String]) -> (FrequencyMap, isize, isize) {
    let mut frequency_map: HashMap<char, Vec<Position>> = HashMap::new();

    let max_y = input.len();
    let max_x = input[0].len();

    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = Position(x as isize, y as isize);
            if c != '.' && c != '#' {
                frequency_map.entry(c).or_insert(vec![]).push(pos);
            }
        }
    }

    (frequency_map, max_x as isize, max_y as isize)
}
