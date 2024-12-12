use std::collections::HashMap;

#[derive(PartialEq)]
enum CellState {
    Open,
    Blocked,
    Visited,
}

struct Direction {
    map_marker: char,
    dx: i32,
    dy: i32,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Position(i32, i32);

const DIRECTIONS: [Direction; 4] = [
    Direction {
        map_marker: '^',
        dx: 0,
        dy: -1,
    },
    Direction {
        map_marker: '>',
        dx: 1,
        dy: 0,
    },
    Direction {
        map_marker: 'v',
        dx: 0,
        dy: 1,
    },
    Direction {
        map_marker: '<',
        dx: -1,
        dy: 0,
    },
];

pub fn solve(input: &[String]) -> usize {
    let (mut map, start_pos, start_dir_idx) = load_input(input);

    let mut current_pos = start_pos;
    let mut dir_idx = start_dir_idx;
    loop {
        // print_map(&map, &current_pos, DIRECTIONS[dir_idx].map_marker);
        let possible_next_pos = Position(
            current_pos.0 + DIRECTIONS[dir_idx].dx,
            current_pos.1 + DIRECTIONS[dir_idx].dy,
        );
        match map.get(&possible_next_pos) {
            Some(CellState::Blocked) => dir_idx = (dir_idx + 1) % DIRECTIONS.len(),
            Some(CellState::Open) | Some(CellState::Visited) => {
                current_pos = possible_next_pos;
                map.insert(current_pos, CellState::Visited);
            }
            _ => break, // out of bounds
        }
    }

    map.iter()
        .filter(|(_, state)| **state == CellState::Visited)
        .count()
}

fn load_input(input: &[String]) -> (HashMap<Position, CellState>, Position, usize) {
    let mut map = HashMap::new();
    let mut start_dir_idx: usize = 0;
    let mut start_pos = Position(0, 0);

    input.iter().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            map.insert(
                Position(x as i32, y as i32),
                match c {
                    '.' => CellState::Open,
                    '#' => CellState::Blocked,
                    'X' => CellState::Visited,
                    '^' | '>' | 'v' | '<' => {
                        start_dir_idx = DIRECTIONS.iter().position(|d| d.map_marker == c).unwrap();
                        start_pos = Position(x as i32, y as i32);
                        CellState::Visited
                    }
                    _ => unreachable!(),
                },
            );
        });
    });

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
//             CellState::Open => '.',
//             CellState::Blocked => '#',
//             CellState::Visited => 'X',
//         };

//         if *pos == current_pos {
//             ch = current_map_marker;
//         }
//         display_map.push(ch);
//     });
//     println!("{}\n", display_map);
// }
