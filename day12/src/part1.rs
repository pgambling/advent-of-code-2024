use std::collections::VecDeque;

type Garden = Vec<Vec<char>>;
type Visited = Vec<Vec<bool>>;

struct Block {
    fences: u8,
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

    fn perimeter(&self) -> usize {
        self.blocks.iter().map(|b| b.fences as usize).sum()
    }

    fn fence_price(&self) -> usize {
        self.area() * self.perimeter()
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

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let rows = grid.len();
    let cols = grid[0].len();
    let mut region = Region::new();
    let mut queue = VecDeque::new();

    let target = grid[start_x][start_y];
    queue.push_back((start_x, start_y));
    visited[start_x][start_y] = true;

    while let Some((x, y)) = queue.pop_front() {
        let mut fences = 4;
        for &(dx, dy) in &directions {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0 && ny >= 0 && (nx as usize) < rows && (ny as usize) < cols {
                let nx = nx as usize;
                let ny = ny as usize;

                if grid[nx][ny] == target {
                    fences -= 1;
                    if !visited[nx][ny] {
                        visited[nx][ny] = true;
                        queue.push_back((nx, ny));
                    }
                }
            }
        }
        region.add_block(Block { fences });
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
