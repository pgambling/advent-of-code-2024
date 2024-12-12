use std::collections::BTreeSet;

const FREE_SPACE: isize = -1;

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
struct Block {
    start: usize,
    length: usize,
    id: isize,
}

impl Block {
    fn new(id: isize, start: usize, length: usize) -> Self {
        Self { id, start, length }
    }
}

struct FreeMemoryManager<'a> {
    disk: &'a mut Disk,
    free_blocks: BTreeSet<Block>,
}

impl<'a> FreeMemoryManager<'a> {
    fn new(disk: &'a mut Disk) -> Self {
        let mut free_blocks = BTreeSet::new();
        let (mut start, mut length) = (0, 0);
        let mut is_new_block = true;
        for i in 0..disk.len() {
            if disk[i] == FREE_SPACE && i != disk.len() - 1 {
                length += 1;
                if is_new_block {
                    start = i;
                    is_new_block = false;
                }
            } else if length > 0 {
                free_blocks.insert(Block::new(FREE_SPACE, start, length));
                length = 0;
                is_new_block = true;
            }
        }

        Self { disk, free_blocks }
    }

    fn move_file_if_possible(&mut self, file_block: &Block) {
        let free_block = self
            .free_blocks
            .iter()
            .find(|block| block.start < file_block.start && block.length >= file_block.length);

        if free_block.is_none() {
            return;
        }

        let free_block = &free_block.unwrap().clone();

        for i in 0..file_block.length {
            self.disk[free_block.start + i] = file_block.id;
            self.disk[file_block.start + i] = FREE_SPACE;
        }

        if free_block.length > file_block.length {
            let new_start = free_block.start + file_block.length;
            let new_length = free_block.length - file_block.length;

            self.free_blocks
                .insert(Block::new(free_block.id, new_start, new_length));
        }

        self.free_blocks.remove(&free_block);
    }

    fn print_disk(&self) {
        for i in 0..self.disk.len() {
            print!(
                "{}",
                if self.disk[i] == FREE_SPACE {
                    '.'
                } else {
                    self.disk[i].to_string().chars().next().unwrap()
                }
            );
        }
        println!();
    }
}

type Disk = Vec<isize>;

pub fn solve(input: &[String]) -> usize {
    let mut disk: Disk = load_input(input);
    let file_blocks = list_file_blocks(&disk);
    let mut free_memory = FreeMemoryManager::new(&mut disk);

    // free_memory.print_disk();
    file_blocks.iter().rev().for_each(|file_block| {
        free_memory.move_file_if_possible(&file_block);
        // free_memory.print_disk();
    });

    // free_memory.print_disk();

    calc_checksum(&disk)
}

fn list_file_blocks(disk: &Disk) -> Vec<Block> {
    let mut file_blocks: Vec<Block> = Vec::new();
    let (mut start, mut prev_id) = (0, -1);
    for i in 0..disk.len() {
        let current_id = disk[i];

        if current_id != prev_id {
            if prev_id != FREE_SPACE {
                file_blocks.push(Block::new(prev_id, start, i - start));
            }

            if current_id != FREE_SPACE {
                start = i;
            }
        }

        prev_id = current_id;
    }
    if prev_id != FREE_SPACE {
        file_blocks.push(Block::new(prev_id, start, disk.len() - start));
    }
    file_blocks
}

fn calc_checksum(disk: &Disk) -> usize {
    let mut checksum = 0;
    for i in 0..disk.len() {
        if disk[i] != FREE_SPACE {
            checksum += i * disk[i] as usize;
        }
    }
    checksum
}

fn load_input(input: &[String]) -> Disk {
    let mut id = -1;
    let mut reading_file = true;

    input[0]
        .chars()
        .flat_map(|c| {
            let n: usize = c.to_digit(10).unwrap() as usize;
            if reading_file {
                reading_file = false;
                id += 1;
                vec![id; n]
            } else {
                reading_file = true;
                vec![FREE_SPACE; n]
            }
        })
        .collect()
}
