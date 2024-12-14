const FREE_SPACE: isize = -1;
type Disk = Vec<isize>;

pub fn solve(input: &[String]) -> usize {
    let mut disk: Disk = load_input(input);

    let mut head = 0;
    let mut tail = disk.len() - 1;
    // println!("{:?}", disk);
    while head <= tail {
        if disk[head] != FREE_SPACE {
            head += 1;
        } else if disk[tail] == FREE_SPACE {
            tail -= 1;
        } else {
            disk[head] = disk[tail];
            disk[tail] = FREE_SPACE;
            head += 1;
            tail -= 1;
        }
    }

    // println!("{:?}", disk);

    calc_checksum(&disk)
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
