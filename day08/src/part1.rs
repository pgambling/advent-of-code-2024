use itertools::Itertools;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Position(isize, isize);

enum PositionState {
    Open,
    Antinode,
    Antenna,
}
use PositionState::*;

type AntennaMap = HashMap<Position, PositionState>;
type FrequencyMap = HashMap<char, Vec<Position>>;

pub fn solve(input: &[String]) -> usize {
    let (antenna_map, frequency_map) = load_input(input);
    frequency_map
        .iter()
        .flat_map(|(_, positions)| calc_antinodes(positions))
        .filter(|antinode_pos| antenna_map.contains_key(antinode_pos))
        .unique()
        .count()
}

fn calc_antinodes(positions: &[Position]) -> Vec<Position> {
    let mut antinodes = vec![];
    for i in 0..positions.len() {
        for j in (i + 1)..positions.len() {
            let (pos1, pos2) = (positions[i], positions[j]);
            let (dx, dy) = (pos2.0 - pos1.0, pos2.1 - pos1.1);
            antinodes.push(Position(pos1.0 - dx, pos1.1 - dy));
            antinodes.push(Position(pos2.0 + dx, pos2.1 + dy));
        }
    }
    antinodes
}

fn load_input(input: &[String]) -> (AntennaMap, FrequencyMap) {
    let mut antenna_map: AntennaMap = HashMap::new();
    let mut frequency_map: HashMap<char, Vec<Position>> = HashMap::new();

    input.iter().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            let pos = Position(x as isize, y as isize);
            antenna_map.insert(
                pos,
                match c {
                    '.' => Open,
                    '#' => Antinode,
                    _ => {
                        frequency_map.entry(c).or_insert(vec![]).push(pos);
                        Antenna
                    }
                },
            );
        });
    });

    (antenna_map, frequency_map)
}
