const A_BUTTON_COST: isize = 3;
const B_BUTTON_COST: isize = 1;
const PRIZE_CALIBRATION: isize = 10000000000000;

#[derive(Debug)]
struct ClawMachine {
    button_a: (isize, isize),
    button_b: (isize, isize),
    prize: (isize, isize),
}

pub fn solve(input: &[String]) -> isize {
    let machines = parse_input(input);
    machines
        .iter()
        .filter_map(|m| find_optimial_token_cost(m))
        .sum()
}

fn find_optimial_token_cost(machine: &ClawMachine) -> Option<isize> {
    let b_press_numerator =
        machine.prize.0 * machine.button_a.1 - machine.prize.1 * machine.button_a.0;
    let b_press_denominator =
        machine.button_b.0 * machine.button_a.1 - machine.button_b.1 * machine.button_a.0;
    if (b_press_numerator % b_press_denominator) != 0 {
        return None;
    }

    let b_presses = b_press_numerator / b_press_denominator;
    let a_press_numerator = machine.prize.0 - machine.button_b.0 * b_presses;

    if (a_press_numerator % machine.button_a.0) != 0 {
        return None;
    }

    let a_presses = a_press_numerator / machine.button_a.0;

    Some(a_presses * A_BUTTON_COST + b_presses * B_BUTTON_COST)
}

fn parse_input(input: &[String]) -> Vec<ClawMachine> {
    let mut machines: Vec<ClawMachine> = vec![];

    for lines in input.chunks(4) {
        if let [button_a, button_b, prize, _] = lines {
            let button_a = button_a.strip_prefix("Button A: ").unwrap();
            let button_b = button_b.strip_prefix("Button B: ").unwrap();
            let prize = prize.strip_prefix("Prize: ").unwrap();

            let parse_coords = |s: &str| {
                let (x, y) = s.split_once(", ").unwrap();
                let x = x
                    .strip_prefix("X+")
                    .or_else(|| x.strip_prefix("X="))
                    .unwrap();
                let y = y
                    .strip_prefix("Y+")
                    .or_else(|| y.strip_prefix("Y="))
                    .unwrap();
                (x.parse().unwrap(), y.parse().unwrap())
            };

            let mut machine = ClawMachine {
                button_a: parse_coords(button_a),
                button_b: parse_coords(button_b),
                prize: parse_coords(prize),
            };

            // fix that darn calibration
            machine.prize = (
                machine.prize.0 + PRIZE_CALIBRATION,
                machine.prize.1 + PRIZE_CALIBRATION,
            );
            machines.push(machine);
        }
    }
    machines
}
