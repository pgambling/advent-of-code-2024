use std::collections::HashMap;

type Warehouse = Vec<Vec<char>>;
type Moves = Vec<char>;

const BOX: char = 'O';
const ROBOT: char = '@';
const EMPTY: char = '.';
const WALL: char = '#';

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
    println!("Moves: {:?}", moves);
    let (mut robot_x, mut robot_y) = find_robot(&warehouse);
    let directions: HashMap<char, Direction> = HashMap::from([
        (UP.marker, UP),
        (DOWN.marker, DOWN),
        (LEFT.marker, LEFT),
        (RIGHT.marker, RIGHT),
    ]);

    println!("Initial warehouse:");
    print_warehouse(&warehouse);
    for mov in moves {
        println!("Move: {}", mov);
        let direction = directions.get(&mov).unwrap();
        let mut positions_to_update: Vec<(isize, isize)> =
            vec![(robot_x as isize, robot_y as isize)];
        let mut position: (isize, isize) = (robot_x as isize, robot_y as isize);
        loop {
            position = (position.0 + direction.dx, position.1 + direction.dy);
            let marker = warehouse[(position.1) as usize][(position.0) as usize];
            match marker {
                WALL => {
                    break;
                }
                BOX => {
                    positions_to_update.push((position.0, position.1));
                }
                EMPTY => {
                    positions_to_update.push((position.0, position.1));
                    positions_to_update.iter().for_each(|(x, y)| {
                        warehouse[*y as usize][*x as usize] = BOX;
                    });
                    let robot_original_position = positions_to_update[0];
                    let new_robot_position = positions_to_update[1];

                    warehouse[robot_original_position.1 as usize]
                        [robot_original_position.0 as usize] = EMPTY;
                    warehouse[new_robot_position.1 as usize][new_robot_position.0 as usize] = ROBOT;
                    robot_x = new_robot_position.0 as usize;
                    robot_y = new_robot_position.1 as usize;
                    break;
                }
                _ => unreachable!(),
            }
        }
        print_warehouse(&warehouse);
    }

    calculate_gps_total(&warehouse)
}

fn calculate_gps_total(warehouse: &Warehouse) -> usize {
    let mut total = 0;
    for (y, row) in warehouse.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == BOX {
                total += y * 100 + x;
            }
        }
    }
    total
}

fn find_robot(warehouse: &Warehouse) -> (usize, usize) {
    for (y, row) in warehouse.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == ROBOT {
                return (x, y);
            }
        }
    }
    (0, 0)
}

fn print_warehouse(warehouse: &Warehouse) {
    for row in warehouse {
        println!("{}", row.iter().collect::<String>());
    }
    println!();
}

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
