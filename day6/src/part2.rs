use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
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
struct Position(i32, i32);

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
    let (map, start_pos, start_dir_idx) = load_input(input);
    let invalid_obstacle_states = [Blocked, StartingPoint, NewObstacle];
    let total = map.len();
    let mut count = 0;
    map.iter()
        .filter(|(obstacle_pos, obstacle_cell_states)| {
            if invalid_obstacle_states
                .iter()
                .all(|s| !obstacle_cell_states.contains(s))
            {
                count += 1;
                println!("{} of {}", count, total);
                let mut test_map = map.clone();
                test_map.get_mut(obstacle_pos).unwrap().insert(NewObstacle);
                has_cycle(&mut test_map, start_pos, start_dir_idx)
            } else {
                false
            }
        })
        .count()
}

fn has_cycle(
    map: &mut HashMap<Position, HashSet<CellState>>,
    start_pos: Position,
    start_dir_idx: usize,
) -> bool {
    let mut current_pos = start_pos;
    let mut dir_idx = start_dir_idx;

    loop {
        let current_dir = &DIRECTIONS[dir_idx];
        // print_map(&map, &current_pos, DIRECTIONS[dir_idx].map_marker);
        let possible_next_pos = Position(
            current_pos.0 + current_dir.dx,
            current_pos.1 + current_dir.dy,
        );

        match map.get_mut(&possible_next_pos) {
            Some(next_cell_state_set) => {
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
            }
            _ => return false, // out of bounds
        }
    }
}

fn load_input(input: &[String]) -> (HashMap<Position, HashSet<CellState>>, Position, usize) {
    let mut map = HashMap::new();
    let mut start_dir_idx: usize = 0;
    let mut start_pos = Position(0, 0);

    input.iter().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            let state = match c {
                '.' => Open,
                '#' => Blocked,
                '^' | '>' | 'v' | '<' => {
                    start_dir_idx = DIRECTIONS.iter().position(|d| d.map_marker == c).unwrap();
                    start_pos = Position(x as i32, y as i32);
                    DIRECTIONS[start_dir_idx].visited_state
                }
                _ => unreachable!(),
            };
            let mut set = HashSet::new();
            set.insert(state);
            map.insert(Position(x as i32, y as i32), set);
        });
    });

    let start_pos_state_set = map.get_mut(&start_pos).unwrap();
    start_pos_state_set.insert(StartingPoint);

    return (map, start_pos, start_dir_idx);
}

// fn print_map(map: &HashMap<Position, CellState>, current_pos: &Position, current_map_marker: char) {
//     let mut sorted_map = map.iter().collect::<Vec<_>>();
//     sorted_map.sort_by(|a, b| {
//         if a.0 .1 != b.0 .1 {
//             a.0 .1.cmp(&b.0 .1)
//         } else {
//             a.0 .0.cmp(&b.0 .0)
//         }
//     });

//     let mut display_map = String::new();
//     let mut current_row = 0;
//     sorted_map.iter().for_each(|(pos, state)| {
//         if pos.1 != current_row {
//             display_map.push('\n');
//             current_row = pos.1;
//         }
//         let mut ch = match state {
//             Open => '.',
//             Blocked => '#',
//             UpDown => '|',
//             LeftRight => '-',
//             BothDirections => '+',
//             NewObstacle => 'O',
//         };

//         if *pos == current_pos {
//             ch = current_map_marker;
//         }
//         display_map.push(ch);
//     });
//     println!("{}\n", display_map);
// }
