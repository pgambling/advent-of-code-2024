type Maze = Vec<Vec<char>>;

const OPEN: char = '.';
const START: char = 'S';
const END: char = 'E';

#[derive(Debug, Clone)]
struct Direction {
    dx: isize,
    dy: isize,
    left: fn() -> &'static Direction,
    right: fn() -> &'static Direction,
    reverse: fn() -> &'static Direction,
}

fn north() -> &'static Direction {
    &NORTH
}

fn east() -> &'static Direction {
    &EAST
}

fn south() -> &'static Direction {
    &SOUTH
}

fn west() -> &'static Direction {
    &WEST
}

static NORTH: Direction = Direction {
    dx: 0,
    dy: -1,
    left: west,
    right: east,
    reverse: south,
};

static EAST: Direction = Direction {
    dx: 1,
    dy: 0,
    left: north,
    right: south,
    reverse: west,
};

static SOUTH: Direction = Direction {
    dx: 0,
    dy: 1,
    left: east,
    right: west,
    reverse: north,
};

static WEST: Direction = Direction {
    dx: -1,
    dy: 0,
    left: south,
    right: north,
    reverse: east,
};

#[derive(Debug, Clone, Copy)]
enum Action {
    Start(usize, usize),
    Move(usize, usize),
    Turn(&'static Direction),
}

#[derive(Debug, Clone)]
struct Path {
    path: Vec<Action>,
    current_position: (usize, usize),
    current_dir: &'static Direction,
}

impl Path {
    fn new(start: (usize, usize)) -> Self {
        Self {
            path: vec![Action::Start(start.0, start.1)],
            current_position: start,
            current_dir: east(),
        }
    }

    fn add_action(&mut self, action: Action) {
        self.path.push(action);
        match action {
            Action::Move(x, y) => self.current_position = (x, y),
            Action::Turn(dir) => self.current_dir = dir,
            Action::Start(x, y) => self.current_position = (x, y),
        }
    }

    fn add_actions(&mut self, actions: Vec<Action>) {
        for action in actions {
            self.add_action(action);
        }
    }

    fn score(&self) -> usize {
        self.path
            .iter()
            .map(|a| match a {
                Action::Start(_, _) => 0,
                Action::Move(_, _) => 1,
                Action::Turn(_) => 1000,
            })
            .sum()
    }

    fn current_position(&self) -> (usize, usize) {
        self.current_position.clone()
    }

    fn current_dir(&self) -> &'static Direction {
        self.current_dir
    }

    fn has_visited_position(&self, position: (usize, usize)) -> bool {
        self.path.iter().any(|a| match a {
            Action::Move(x, y) | Action::Start(x, y) => *x == position.0 && *y == position.1,
            _ => false,
        })
    }
}

pub fn solve(input: &[String]) -> usize {
    let maze = parse_input(input);
    let start = find_start(&maze);
    let mut path = Path::new(start);
    run_maze(&maze, &mut path).unwrap_or(0)
}

fn run_maze(maze: &Maze, path: &mut Path) -> Option<usize> {
    loop {
        let current_position = path.current_position();
        if maze[current_position.1][current_position.0] == END {
            return Some(path.score());
        }

        let current_dir = path.current_dir();
        let mut possible_next_actions = vec![];

        // Check each possible direction
        let directions = [
            (
                (current_dir.left)(),
                vec![Action::Turn((current_dir.left)())],
            ),
            (
                (current_dir.right)(),
                vec![Action::Turn((current_dir.right)())],
            ),
            (current_dir, vec![]),
            (
                (current_dir.reverse)(),
                vec![
                    Action::Turn((current_dir.left)()),
                    Action::Turn((current_dir.left)()),
                ],
            ),
        ];

        for (dir, turn_actions) in directions {
            let next_pos = (
                current_position.0.wrapping_add(dir.dx as usize),
                current_position.1.wrapping_add(dir.dy as usize),
            );

            if is_open_position(maze, path, next_pos) {
                let mut actions = turn_actions;
                actions.push(Action::Move(next_pos.0, next_pos.1));
                possible_next_actions.push(actions);
            }
        }

        match possible_next_actions.len() {
            // Dead end
            0 => return None,
            // Single path
            1 => path.add_actions(possible_next_actions.remove(0)),
            // Multiple paths - try each branch
            _ => {
                return possible_next_actions
                    .into_iter()
                    .filter_map(|actions| {
                        let mut new_path = path.clone();
                        new_path.add_actions(actions);
                        run_maze(maze, &mut new_path)
                    })
                    .min();
            }
        }
    }
}

fn is_open_position(maze: &Maze, path: &Path, position: (usize, usize)) -> bool {
    let (x, y) = position;
    let position_state = maze[y][x];
    (position_state == OPEN || position_state == END) && !path.has_visited_position(position)
}

fn parse_input(input: &[String]) -> Maze {
    input.iter().map(|s| s.chars().collect()).collect()
}

fn find_start(maze: &Maze) -> (usize, usize) {
    for (y, row) in maze.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == START {
                return (x, y);
            }
        }
    }
    (0, 0)
}

// fn print_maze(maze: &Maze, path: &Path) {
//     for (y, row) in maze.iter().enumerate() {
//         for (x, &c) in row.iter().enumerate() {
//             if path.has_visited_position((x, y)) {
//                 print!("{}", '@');
//             } else {
//                 print!("{}", c);
//             }
//         }
//         println!();
//     }
//     println!();
// }
