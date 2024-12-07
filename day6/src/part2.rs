use std::collections::{HashMap, HashSet};

type GuardMap = Vec<Vec<CellState>>;
type Path = HashMap<Position, HashSet<CellState>>;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
enum CellState {
    Open,
    Blocked,
    Up,
    Down,
    Left,
    Right,
    StartingPoint,
}
use CellState::*;

struct Direction {
    map_marker: char,
    dx: i32,
    dy: i32,
    visited_state: CellState,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Position(usize, usize);

const DIRECTIONS: [Direction; 4] = [
    Direction {
        map_marker: '^',
        dx: 0,
        dy: -1,
        visited_state: Up,
    },
    Direction {
        map_marker: '>',
        dx: 1,
        dy: 0,
        visited_state: Right,
    },
    Direction {
        map_marker: 'v',
        dx: 0,
        dy: 1,
        visited_state: Down,
    },
    Direction {
        map_marker: '<',
        dx: -1,
        dy: 0,
        visited_state: Left,
    },
];

pub fn solve(input: &[String]) -> usize {
    let (input_map, start_pos, start_dir_idx) = load_input(input);
    let (_, path_traveled) = run_simulation(
        &input_map,
        &start_pos,
        start_dir_idx,
        &Position(input_map.len(), input_map[0].len()),
    );

    path_traveled
        .iter()
        .filter(|(obstacle_pos, _)| {
            let (has_cycle, _) =
                run_simulation(&input_map, &start_pos, start_dir_idx, obstacle_pos);
            has_cycle
        })
        .count()
}

fn run_simulation(
    map: &GuardMap,
    start_pos: &Position,
    start_dir_idx: usize,
    obstacle: &Position,
) -> (bool, Path) {
    let mut current_pos = *start_pos;
    let mut dir_idx = start_dir_idx;
    let max_col = map[0].len() as i32;
    let max_row = map.len() as i32;
    let mut path_traveled: Path = HashMap::new();

    loop {
        let current_dir = &DIRECTIONS[dir_idx];
        let (next_row, next_col) = (
            current_pos.0 as i32 + current_dir.dy,
            current_pos.1 as i32 + current_dir.dx,
        );

        if next_row < 0 || next_col < 0 || next_col >= max_col || next_row >= max_row {
            return (false, path_traveled);
        } // out of bounds

        let possible_next_pos = Position(next_row as usize, next_col as usize);

        if map[next_row as usize][next_col as usize] == Blocked || possible_next_pos == *obstacle {
            dir_idx = (dir_idx + 1) % DIRECTIONS.len()
        } else {
            current_pos = possible_next_pos;
            let path_to_update = path_traveled.entry(current_pos).or_insert(HashSet::new());
            // cycle detected, returning to a cell in the same direction previously visited
            if path_to_update.contains(&current_dir.visited_state) {
                return (true, path_traveled);
            } else {
                path_to_update.insert(current_dir.visited_state);
            }
        }
    }
}

fn load_input(input: &[String]) -> (GuardMap, Position, usize) {
    let mut start_dir_idx: usize = 0;
    let mut start_pos = Position(0, 0);

    let mut map: GuardMap = input
        .iter()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| match c {
                    '.' => Open,
                    '#' => Blocked,
                    '^' | '>' | 'v' | '<' => {
                        start_dir_idx = DIRECTIONS.iter().position(|d| d.map_marker == c).unwrap();
                        start_pos = Position(row, col);
                        DIRECTIONS[start_dir_idx].visited_state
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    map[start_pos.0][start_pos.1] = StartingPoint;

    return (map, start_pos, start_dir_idx);
}

// fn print_map(map: &GuardMap) {
//     let mut display_map = String::new();

//     for row in 0..map.len() {
//         for col in 0..map[row].len() {
//             let cell_state = &map[row][col];
//             let mut ch = '.';

//             if cell_state.contains(&Open) {
//                 ch = '.';
//             } else if cell_state.contains(&Blocked) {
//                 ch = '#';
//             } else if cell_state.contains(&Up) {
//                 ch = '|';
//             } else if cell_state.contains(&Down) {
//                 ch = '|';
//             } else if cell_state.contains(&Left) {
//                 ch = '-';
//             } else if cell_state.contains(&Right) {
//                 ch = '-';
//             } else if cell_state.contains(&NewObstacle) {
//                 ch = 'O';
//             } else if cell_state.contains(&StartingPoint) {
//                 ch = 'S'
//             }

//             if (ch == '|' && (cell_state.contains(&Right) || cell_state.contains(&Left)))
//                 || (ch == '-' && (cell_state.contains(&Up) || cell_state.contains(&Down)))
//             {
//                 ch = '+';
//             }

//             display_map.push(ch);
//         }
//         display_map.push('\n');
//     }
//     println!("{}\n", display_map);
// }
