use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

type Direction = (isize, isize);

const UP: Direction = (-1, 0);
const DOWN: Direction = (1, 0);
const LEFT: Direction = (0, -1);
const RIGHT: Direction = (0, 1);

const DIRECTIONS: [Direction; 4] = [UP, DOWN, LEFT, RIGHT];

type Garden = Vec<Vec<char>>;
type Visited = Vec<Vec<bool>>;

#[derive(Clone, Debug)]
struct Block {
    x: usize,
    y: usize,
    fences: HashSet<Direction>,
}

impl Block {
    fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            fences: HashSet::new(),
        }
    }

    fn add_fence(&mut self, direction: Direction) {
        self.fences.insert(direction);
    }
}

struct Region {
    blocks: Vec<Block>,
}

impl Region {
    fn new() -> Self {
        let blocks = vec![];
        Self { blocks }
    }

    fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }

    fn area(&self) -> usize {
        self.blocks.len()
    }

    fn count_sides_for_direction(&self, direction: Direction) -> usize {
        let mut sides = 0;
        let mut key_fn = vec![|b: &Block| b.x, |b: &Block| b.y];
        match direction {
            UP | DOWN => (),
            LEFT | RIGHT => key_fn.reverse(),
            _ => unreachable!(),
        };

        let group_key = key_fn[0];
        let sort_key = key_fn[1];
        self.blocks_with_fences()
            .into_iter()
            .filter(|b| b.fences.contains(&direction))
            .sorted_by_key(group_key)
            .chunk_by(group_key)
            .into_iter()
            .for_each(|(_, group)| {
                let mut group: Vec<Block> = group.collect();
                group.sort_by_key(sort_key);
                sides += 1;
                for window in group.windows(2) {
                    let block1 = &window[0];
                    let block2 = &window[1];
                    let is_gap = match direction {
                        UP | DOWN => block2.x != block1.x || (block2.y - block1.y) > 1,
                        LEFT | RIGHT => block2.y != block1.y || (block2.x - block1.x) > 1,
                        _ => unreachable!(),
                    };
                    if is_gap {
                        sides += 1;
                    }
                }
            });

        sides
    }

    fn blocks_with_fences(&self) -> Vec<Block> {
        self.blocks
            .iter()
            .filter(|b| b.fences.len() > 0)
            .map(|b| b.clone())
            .collect()
    }

    fn count_sides(&self) -> usize {
        let mut sides = 0;
        for direction in DIRECTIONS {
            sides += self.count_sides_for_direction(direction);
        }
        sides
    }

    fn fence_price(&self) -> usize {
        self.area() * self.count_sides()
    }
}

pub fn solve(input: &[String]) -> usize {
    let garden = parse_input(input);
    let mut regions: Vec<Region> = vec![];
    let mut visited = vec![vec![false; garden[0].len()]; garden.len()];

    for i in 0..garden.len() {
        for j in 0..garden[0].len() {
            if let Some(region) = bfs_find_region(&garden, &mut visited, i, j) {
                regions.push(region);
            }
        }
    }

    regions.iter().map(|r| r.fence_price()).sum()
}

fn bfs_find_region(
    grid: &Garden,
    visited: &mut Visited,
    start_x: usize,
    start_y: usize,
) -> Option<Region> {
    if visited[start_x][start_y] {
        return None;
    }

    let rows = grid.len();
    let cols = grid[0].len();
    let target = grid[start_x][start_y];
    let mut region = Region::new();
    let mut queue = VecDeque::new();

    queue.push_back((start_x, start_y));
    visited[start_x][start_y] = true;

    while let Some((x, y)) = queue.pop_front() {
        let mut block = Block::new(x, y);
        for &(dx, dy) in &DIRECTIONS {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0 && ny >= 0 && (nx as usize) < rows && (ny as usize) < cols {
                let nx = nx as usize;
                let ny = ny as usize;

                if grid[nx][ny] == target {
                    if !visited[nx][ny] {
                        visited[nx][ny] = true;
                        queue.push_back((nx, ny));
                    }
                } else {
                    block.add_fence((dx, dy));
                }
            } else {
                block.add_fence((dx, dy));
            }
        }
        region.add_block(block);
    }

    Some(region)
}

fn parse_input(input: &[String]) -> Garden {
    let (rows, cols) = (input.len(), input[0].len());
    let mut garden = vec![vec![' '; cols]; rows];
    for (i, row) in input.iter().enumerate() {
        for (j, c) in row.chars().enumerate() {
            garden[i][j] = c;
        }
    }
    garden
}
