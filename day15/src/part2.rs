use std::collections::HashMap;

type Warehouse = Vec<Vec<char>>;
type Moves = Vec<char>;

#[derive(Debug)]
struct PositionToAdvance {
    x: isize,
    y: isize,
    marker: char,
}

impl PositionToAdvance {
    fn new(x: isize, y: isize, marker: char) -> Self {
        Self { x, y, marker }
    }
}

const BOX_LEFT: char = '[';
const BOX_RIGHT: char = ']';
const ROBOT: char = '@';
const EMPTY: char = '.';
const WALL: char = '#';

#[derive(Debug)]
struct Direction {
    dx: isize,
    dy: isize,
    marker: char,
}

const UP: Direction = Direction {
    dx: 0,
    dy: -1,
    marker: '^',
};
const DOWN: Direction = Direction {
    dx: 0,
    dy: 1,
    marker: 'v',
};
const LEFT: Direction = Direction {
    dx: -1,
    dy: 0,
    marker: '<',
};
const RIGHT: Direction = Direction {
    dx: 1,
    dy: 0,
    marker: '>',
};

pub fn solve(input: &[String]) -> usize {
    let (mut warehouse, moves) = parse_input(input);
    let (mut robot_x, mut robot_y) = find_robot(&warehouse);
    let directions: HashMap<char, Direction> = HashMap::from([
        (UP.marker, UP),
        (DOWN.marker, DOWN),
        (LEFT.marker, LEFT),
        (RIGHT.marker, RIGHT),
    ]);

    for mov in moves {
        let direction = directions.get(&mov).unwrap();
        let (new_robot_x, new_robot_y) = process_move(&mut warehouse, robot_x, robot_y, direction);
        robot_x = new_robot_x;
        robot_y = new_robot_y;
    }

    calculate_gps_total(&warehouse)
}

fn process_move(
    warehouse: &mut Warehouse,
    current_robot_x: isize,
    current_robot_y: isize,
    direction: &Direction,
) -> (isize, isize) {
    let mut positions_to_check: Vec<(isize, isize)> =
        vec![(current_robot_x as isize, current_robot_y as isize)];
    let mut positions_to_update: Vec<PositionToAdvance> = vec![PositionToAdvance::new(
        current_robot_x,
        current_robot_y,
        ROBOT,
    )];

    loop {
        let mut next_positions_to_check: Vec<(isize, isize)> = vec![];
        let mut can_move_forward = true;

        for (x, y) in positions_to_check.iter() {
            let position = (x + direction.dx, y + direction.dy);
            let marker = warehouse[position.1 as usize][position.0 as usize];
            match marker {
                WALL => {
                    return (current_robot_x, current_robot_y); // Nothing moved
                }
                BOX_LEFT | BOX_RIGHT => {
                    can_move_forward = false;
                    next_positions_to_check.push((position.0, position.1));
                    positions_to_update
                        .push(PositionToAdvance::new(position.0, position.1, marker));
                    if direction.dy != 0 {
                        let x_offset = if marker == BOX_LEFT { 1 } else { -1 };
                        let box_other_side = (position.0 + x_offset, position.1);
                        positions_to_update.push(PositionToAdvance::new(
                            box_other_side.0,
                            box_other_side.1,
                            if marker == BOX_LEFT {
                                BOX_RIGHT
                            } else {
                                BOX_LEFT
                            },
                        ));
                        next_positions_to_check.push(box_other_side);
                    }
                }
                EMPTY => {}
                _ => {
                    unreachable!()
                }
            }
        }
        if can_move_forward {
            break;
        }
        positions_to_check = next_positions_to_check;
    }

    // move the boxes
    let mut vacated_positions: std::collections::HashSet<_> =
        positions_to_update.iter().map(|p| (p.x, p.y)).collect();
    positions_to_update.iter().for_each(|update| {
        let new_x = update.x + direction.dx;
        let new_y = update.y + direction.dy;
        warehouse[new_y as usize][new_x as usize] = update.marker;
        vacated_positions.remove(&(new_x, new_y));
    });

    vacated_positions.iter().for_each(|&(x, y)| {
        warehouse[y as usize][x as usize] = EMPTY;
    });

    let new_robot_x = current_robot_x + direction.dx;
    let new_robot_y = current_robot_y + direction.dy;
    warehouse[new_robot_y as usize][new_robot_x as usize] = ROBOT;
    warehouse[current_robot_y as usize][current_robot_x as usize] = EMPTY;

    (new_robot_x, new_robot_y)
}

fn calculate_gps_total(warehouse: &Warehouse) -> usize {
    let mut total = 0;
    for (y, row) in warehouse.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == BOX_LEFT {
                total += y * 100 + x;
            }
        }
    }
    total
}

fn find_robot(warehouse: &Warehouse) -> (isize, isize) {
    for (y, row) in warehouse.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == ROBOT {
                return (x as isize, y as isize);
            }
        }
    }
    (0, 0)
}

// fn print_warehouse(warehouse: &Warehouse) {
//     let mut error = false;
//     let numbers = (0..warehouse[0].len())
//         .map(|i| (i % 10).to_string())
//         .collect::<String>();
//     println!(" {}", numbers);
//     for (y, row) in warehouse.iter().enumerate() {
//         let line = row.iter().collect::<String>();
//         if line.contains("[[") || line.contains("]]") || line.contains("[.") || line.contains(".]")
//         {
//             error = true;
//         }
//         println!("{}{}{}", y, line, y);
//     }
//     println!(" {}", numbers);
//     println!();
//     if error {
//         panic!("Error in warehouse");
//     }
// }

fn parse_input(input: &[String]) -> (Warehouse, Moves) {
    let mut warehouse = vec![];
    let mut moves = vec![];
    let mut loading_warehouse = true;

    for row in input.iter() {
        if row.is_empty() {
            loading_warehouse = false;
            continue;
        }
        if loading_warehouse {
            let modified_row = row
                .replace('#', "##")
                .replace('O', "[]")
                .replace('.', "..")
                .replace('@', "@.");
            warehouse.push(modified_row.chars().collect());
        } else {
            for c in row.chars() {
                moves.push(c);
            }
        }
    }
    (warehouse, moves)
}
