const A_BUTTON_COST: usize = 3;
const B_BUTTON_COST: usize = 1;
const BUTTON_PRESS_MAX: usize = 100;

#[derive(Debug)]
struct ClawMachine {
    button_a: (usize, usize),
    button_b: (usize, usize),
    prize: (usize, usize),
}

pub fn solve(input: &[String]) -> usize {
    let machines = parse_input(input);
    machines
        .iter()
        .filter_map(|m| find_optimial_token_cost(m))
        .sum()
}

fn find_optimial_token_cost(machine: &ClawMachine) -> Option<usize> {
    let mut min_cost = usize::MAX;
    for a_presses in 0..BUTTON_PRESS_MAX {
        for b_presses in 0..BUTTON_PRESS_MAX {
            let result = calc_button_press_result(machine, a_presses, b_presses);
            if result.is_some() {
                min_cost = min_cost.min(result.unwrap());
            }
        }
    }
    if min_cost == usize::MAX {
        None
    } else {
        Some(min_cost)
    }
}

fn calc_button_press_result(
    machine: &ClawMachine,
    button_a_presses: usize,
    button_b_presses: usize,
) -> Option<usize> {
    let token_cost = button_a_presses * A_BUTTON_COST + button_b_presses * B_BUTTON_COST;
    let result_x = button_a_presses * machine.button_a.0 + button_b_presses * machine.button_b.0;
    let result_y = button_a_presses * machine.button_a.1 + button_b_presses * machine.button_b.1;
    if result_x == machine.prize.0 && result_y == machine.prize.1 {
        return Some(token_cost);
    }
    None
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

            machines.push(ClawMachine {
                button_a: parse_coords(button_a),
                button_b: parse_coords(button_b),
                prize: parse_coords(prize),
            });
        }
    }

    machines
}
