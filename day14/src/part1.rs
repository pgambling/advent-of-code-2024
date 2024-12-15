const NUM_SECONDS: isize = 100;

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

    for _ in 0..NUM_SECONDS {
        for robot in robots.iter_mut() {
            robot.advance();
        }
    }

    calculate_safety_score(&robots, max_x, max_y)
}

fn calculate_safety_score(robots: &[Robot], max_x: isize, max_y: isize) -> isize {
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

    quad1_count * quad2_count * quad3_count * quad4_count
}

fn parse_input(input: &[String], max_x: isize, max_y: isize) -> Vec<Robot> {
    input
        .iter()
        .map(|line| Robot::new(line, max_x, max_y))
        .collect()
}
