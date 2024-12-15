#[derive(Clone)]
struct Robot {
    x: isize,
    y: isize,
    velocity: (isize, isize),
    max_x: isize,
    max_y: isize,
}

impl Robot {
    fn new(init: &String, max_x: isize, max_y: isize) -> Self {
        let parts: Vec<&str> = init.split(" ").collect();
        let pos_part = parts[0].strip_prefix("p=").unwrap();
        let vel_part = parts[1].strip_prefix("v=").unwrap();

        let pos: Vec<&str> = pos_part.split(",").collect();
        let vel: Vec<&str> = vel_part.split(",").collect();

        let x = pos[0].parse().unwrap();
        let y = pos[1].parse().unwrap();
        let velocity: (isize, isize) = (vel[0].parse().unwrap(), vel[1].parse().unwrap());

        Robot {
            x,
            y,
            velocity,
            max_x,
            max_y,
        }
    }

    fn advance(&mut self) {
        self.x += self.velocity.0;
        self.y += self.velocity.1;

        if self.x >= self.max_x {
            self.x -= self.max_x;
        }

        if self.y >= self.max_y {
            self.y -= self.max_y;
        }

        if self.x < 0 {
            self.x += self.max_x;
        }

        if self.y < 0 {
            self.y += self.max_y;
        }
    }
}

pub fn solve(input: &[String], max_x: isize, max_y: isize) -> isize {
    let mut robots: Vec<Robot> = parse_input(input, max_x, max_y);
    let mut min_safety_score = isize::MAX;
    let mut robots_snapshot = robots.clone();
    let mut seconds_snapshot = 0;
    for seconds in 1..=(max_x * max_y) {
        for robot in robots.iter_mut() {
            robot.advance();
        }

        let (quad1, quad2, quad3, quad4) = calculate_safety_score(&robots, max_x, max_y);
        let new_safety_score = quad1 * quad2 * quad3 * quad4;
        if seconds == 6532 {
            println!(
                "Quad1: {}, Quad2: {}, Quad3: {}, Quad4: {}",
                quad1, quad2, quad3, quad4
            );
            println!("New safety score: {}", new_safety_score);
        }
        if new_safety_score < min_safety_score {
            min_safety_score = new_safety_score;
            robots_snapshot = robots.clone();
            seconds_snapshot = seconds;
        }

        // WHATEVER, I just dumped all the output to file and CTRL+F for "11111111111111" in a text editor to find the answer
        print_grid(&robots, seconds, new_safety_score, max_x, max_y);
    }
    print_grid(
        &robots_snapshot,
        seconds_snapshot,
        min_safety_score,
        max_x,
        max_y,
    );
    seconds_snapshot
}

fn calculate_safety_score(
    robots: &[Robot],
    max_x: isize,
    max_y: isize,
) -> (isize, isize, isize, isize) {
    let quad1_min = (0 as isize, 0 as isize);
    let quad1_max = (max_x / 2, max_y / 2);

    let quad2_min = (max_x / 2 + max_x % 2, 0);
    let quad2_max = (max_x, max_y / 2);

    let quad3_min = (0 as isize, max_y / 2 + max_y % 2);
    let quad3_max = (max_x / 2, max_y);

    let quad4_min = (max_x / 2 + max_x % 2, max_y / 2 + max_y % 2);
    let quad4_max = (max_x, max_y);

    let mut quad1_count = 0;
    let mut quad2_count = 0;
    let mut quad3_count = 0;
    let mut quad4_count = 0;

    for robot in robots {
        if robot.x >= quad1_min.0
            && robot.x < quad1_max.0
            && robot.y >= quad1_min.1
            && robot.y < quad1_max.1
        {
            quad1_count += 1;
        } else if robot.x >= quad2_min.0
            && robot.x < quad2_max.0
            && robot.y >= quad2_min.1
            && robot.y < quad2_max.1
        {
            quad2_count += 1;
        } else if robot.x >= quad3_min.0
            && robot.x < quad3_max.0
            && robot.y >= quad3_min.1
            && robot.y < quad3_max.1
        {
            quad3_count += 1;
        } else if robot.x >= quad4_min.0
            && robot.x < quad4_max.0
            && robot.y >= quad4_min.1
            && robot.y < quad4_max.1
        {
            quad4_count += 1;
        }
    }

    (quad1_count, quad2_count, quad3_count, quad4_count)
}

fn print_grid(robots: &[Robot], seconds: isize, safety_score: isize, max_x: isize, max_y: isize) {
    for y in 0..max_y {
        for x in 0..max_x {
            let robot_count = robots.iter().filter(|r| r.x == x && r.y == y).count();
            let ch = if robot_count > 0 { '1' } else { '.' };
            print!("{}", ch);
        }
        println!();
    }
    println!("SECONDS: {}", seconds);
    println!("SAFETY SCORE: {}", safety_score);
}

fn parse_input(input: &[String], max_x: isize, max_y: isize) -> Vec<Robot> {
    input
        .iter()
        .map(|line| Robot::new(line, max_x, max_y))
        .collect()
}
