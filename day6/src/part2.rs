use std::collections::HashSet;

type GuardMap = Vec<Vec<HashSet<CellState>>>;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
enum CellState {
    Open,
    Blocked,
    Up,
    Down,
    Left,
    Right,
    NewObstacle,
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
    let mut unaltered_result = input_map.clone();
    run_simulation(&mut unaltered_result, start_pos, start_dir_idx);
    let visited_states = [Up, Down, Left, Right];

    let mut positions_to_test: Vec<Position> = Vec::new();
    for row in 0..unaltered_result.len() {
        for col in 0..unaltered_result[row].len() {
            let cell_states = &unaltered_result[row][col];
            if visited_states
                .iter()
                .any(|s| cell_states.contains(s) && !cell_states.contains(&StartingPoint))
            {
                positions_to_test.push(Position(row, col));
            }
        }
    }

    positions_to_test
        .iter()
        .filter(|obstacle_pos| {
            let mut test_map = input_map.clone();
            test_map[obstacle_pos.0][obstacle_pos.1].insert(NewObstacle);
            let has_cycle = run_simulation(&mut test_map, start_pos, start_dir_idx);
            has_cycle
        })
        .count()
}

fn run_simulation(map: &mut GuardMap, start_pos: Position, start_dir_idx: usize) -> bool {
    let mut current_pos = start_pos;
    let mut dir_idx = start_dir_idx;
    let max_col = map[0].len() as i32;
    let max_row = map.len() as i32;

    loop {
        let current_dir = &DIRECTIONS[dir_idx];
        let (next_row, next_col) = (
            current_pos.0 as i32 + current_dir.dy,
            current_pos.1 as i32 + current_dir.dx,
        );

        if next_row < 0 || next_col < 0 || next_col >= max_col || next_row >= max_row {
            return false;
        } // out of bounds

        let possible_next_pos = Position(next_row as usize, next_col as usize);

        let next_cell_state_set = &mut map[possible_next_pos.0][possible_next_pos.1];

        // cycle detected, returning to a cell in the same direction previously visited
        if next_cell_state_set.contains(&current_dir.visited_state) {
            return true;
        } else if next_cell_state_set.contains(&Blocked)
            || next_cell_state_set.contains(&NewObstacle)
        {
            dir_idx = (dir_idx + 1) % DIRECTIONS.len()
        } else {
            current_pos = possible_next_pos;
            next_cell_state_set.remove(&Open);
            next_cell_state_set.insert(current_dir.visited_state);
        }
        // print_map(map);
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
                .map(|(col, c)| {
                    let state = match c {
                        '.' => Open,
                        '#' => Blocked,
                        '^' | '>' | 'v' | '<' => {
                            start_dir_idx =
                                DIRECTIONS.iter().position(|d| d.map_marker == c).unwrap();
                            start_pos = Position(row, col);
                            DIRECTIONS[start_dir_idx].visited_state
                        }
                        _ => unreachable!(),
                    };
                    let mut set = HashSet::new();
                    set.insert(state);
                    set
                })
                .collect()
        })
        .collect();

    map[start_pos.0][start_pos.1].insert(StartingPoint);

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
