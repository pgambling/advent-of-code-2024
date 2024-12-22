use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::{HashMap, HashSet};

type Maze = Vec<Vec<char>>;
type Position = (usize, usize);

const OPEN: char = '.';
const START: char = 'S';
const END: char = 'E';

#[derive(Debug, Clone, Copy)]
struct Direction {
    dx: isize,
    dy: isize,
    marker: char,
    left: char,
    right: char,
    reverse: char,
}

static NORTH: Direction = Direction {
    dx: 0,
    dy: -1,
    marker: '^',
    left: '<',
    right: '>',
    reverse: 'v',
};

static EAST: Direction = Direction {
    dx: 1,
    dy: 0,
    marker: '>',
    left: '^',
    right: 'v',
    reverse: '<',
};

static SOUTH: Direction = Direction {
    dx: 0,
    dy: 1,
    marker: 'v',
    left: '>',
    right: '<',
    reverse: '^',
};

static WEST: Direction = Direction {
    dx: -1,
    dy: 0,
    marker: '<',
    left: 'v',
    right: '^',
    reverse: '>',
};

#[derive(Debug, Clone, Copy)]
enum Action {
    Move(usize, usize),
    Turn(char),
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Path {
    current_position: Position,
    current_dir: char,
    score: usize,
    visited_positions: Vec<Position>,
}

impl Path {
    fn new(current_position: Position, current_dir: char) -> Self {
        Self {
            current_position,
            current_dir,
            score: 0,
            visited_positions: vec![],
        }
    }

    fn add_action(&mut self, action: &Action) {
        match *action {
            Action::Move(x, y) => {
                self.current_position = (x, y);
                self.score += 1;
                self.visited_positions.push((x, y));
            }
            Action::Turn(dir) => {
                self.current_dir = dir;
                self.score += 1000;
            }
        }
    }

    fn add_actions(&mut self, actions: &Vec<Action>) {
        for action in actions {
            self.add_action(action);
        }
    }

    fn score(&self) -> usize {
        self.score
    }

    fn current_position(&self) -> Position {
        self.current_position.clone()
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn solve(input: &[String]) -> usize {
    let maze = parse_input(input);
    let start = find_start(&maze);
    run_maze(&maze, start, '>')
}

fn run_maze(maze: &Maze, start: (usize, usize), start_dir: char) -> usize {
    let dirs_map: HashMap<char, Direction> =
        HashMap::from([('^', NORTH), ('>', EAST), ('v', SOUTH), ('<', WEST)]);
    let mut stack: BinaryHeap<Path> = BinaryHeap::from([Path::new(start, start_dir)]);
    let mut visited: HashMap<(Position, char), usize> = HashMap::new();
    let mut min_score = usize::MAX;
    let mut min_score_path_positions: HashSet<Position> = HashSet::new();
    while let Some(path) = stack.pop() {
        let current_position = path.current_position();
        let current_dir = path.current_dir;

        if path.score() > min_score {
            continue;
        }

        if maze[current_position.1][current_position.0] == END {
            let score = path.score();
            if min_score_path_positions.is_empty() {
                min_score = score;
            }

            if score == min_score {
                min_score_path_positions.extend(path.visited_positions.clone());
            }
        }

        // Check each possible direction
        let current_dir = dirs_map.get(&current_dir).unwrap();
        let directions = [
            (current_dir.left, vec![Action::Turn(current_dir.left)]),
            (current_dir.right, vec![Action::Turn(current_dir.right)]),
            (current_dir.marker, vec![]),
            (
                current_dir.reverse,
                vec![
                    Action::Turn(current_dir.left),
                    Action::Turn(current_dir.left),
                ],
            ),
        ];

        for (dir_marker, turn_actions) in directions {
            let dir = dirs_map.get(&dir_marker).unwrap();
            let next_pos: Position = (
                current_position.0.wrapping_add(dir.dx as usize),
                current_position.1.wrapping_add(dir.dy as usize),
            );

            let next_pos_state = maze[next_pos.1][next_pos.0];

            if next_pos_state == OPEN || next_pos_state == END {
                let mut actions = turn_actions;
                actions.push(Action::Move(next_pos.0, next_pos.1));
                let mut new_path = path.clone();
                new_path.add_actions(&actions);

                let new_score = new_path.score();
                if let Some(score) = visited.get(&(next_pos, current_dir.marker)) {
                    if new_score <= *score {
                        stack.push(new_path);
                        visited.insert((next_pos, current_dir.marker), new_score);
                    }
                } else {
                    stack.push(new_path);
                    visited.insert((next_pos, current_dir.marker), new_score);
                }
            }
        }
    }

    min_score_path_positions.len() + 1
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
