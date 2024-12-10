use std::collections::HashMap;

const TRAILEND: i32 = 9;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Position(i32, i32);

type TrailMap = HashMap<Position, i32>;

pub fn solve(input: &[String]) -> usize {
    let map: TrailMap = load_input(input);

    map.iter()
        .map(|(pos, _)| count_trails(&map, *pos, -1))
        .sum()
}

fn count_trails(map: &TrailMap, pos: Position, previous_level: i32) -> usize {
    let current_level = map.get(&pos);
    if current_level.is_none() {
        return 0;
    }

    let current_level = *current_level.unwrap();
    if current_level - previous_level != 1 {
        return 0;
    }

    if current_level == TRAILEND {
        return 1;
    }

    count_trails(map, Position(pos.0 - 1, pos.1), current_level)
        + count_trails(map, Position(pos.0 + 1, pos.1), current_level)
        + count_trails(map, Position(pos.0, pos.1 - 1), current_level)
        + count_trails(map, Position(pos.0, pos.1 + 1), current_level)
}

fn load_input(input: &[String]) -> TrailMap {
    let mut trail_map: TrailMap = HashMap::new();

    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = Position(x as i32, y as i32);
            if c != '.' {
                trail_map.insert(pos, c.to_digit(10).unwrap() as i32);
            }
        }
    }

    trail_map
}
