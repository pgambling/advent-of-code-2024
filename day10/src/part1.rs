use std::collections::{HashMap, HashSet};

const IMPASSABLE: i32 = -1;
const TRAILHEAD: i32 = 0;
const TRAILEND: i32 = 9;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Position(i32, i32);

type TrailMap = HashMap<Position, i32>;

pub fn solve(input: &[String]) -> usize {
    let map: TrailMap = load_input(input);

    map.iter()
        .map(|(pos, level)| {
            if *level != TRAILHEAD {
                return 0;
            }
            let mut trail_ends: HashSet<Position> = HashSet::new();
            find_trail_ends(&map, *pos, IMPASSABLE, &mut trail_ends);
            trail_ends.len()
        })
        .sum()
}

fn find_trail_ends(
    map: &TrailMap,
    pos: Position,
    previous_level: i32,
    trail_ends: &mut HashSet<Position>,
) {
    if trail_ends.contains(&pos) {
        return;
    }

    let current_level = map.get(&pos);
    if current_level.is_none() {
        return;
    }

    let current_level = *current_level.unwrap();
    if current_level == IMPASSABLE || current_level - previous_level != 1 {
        return;
    }

    if current_level == TRAILEND {
        trail_ends.insert(pos);
        return;
    }

    find_trail_ends(map, Position(pos.0 - 1, pos.1), current_level, trail_ends);
    find_trail_ends(map, Position(pos.0 + 1, pos.1), current_level, trail_ends);
    find_trail_ends(map, Position(pos.0, pos.1 - 1), current_level, trail_ends);
    find_trail_ends(map, Position(pos.0, pos.1 + 1), current_level, trail_ends);
}

fn load_input(input: &[String]) -> TrailMap {
    let mut trail_map: TrailMap = HashMap::new();

    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = Position(x as i32, y as i32);
            if c == '.' {
                trail_map.insert(pos, IMPASSABLE);
            } else {
                trail_map.insert(pos, c.to_digit(10).unwrap() as i32);
            }
        }
    }

    trail_map
}
