pub fn solve(input: &[String]) -> isize {
    input
        .iter()
        .filter_map(|line| {
            let (answer, numbers) = parse_line(&line);
            find_solution(answer, &numbers)
        })
        .sum()
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Operator {
    Add,
    Multiply,
}

fn find_solution(answer: isize, numbers: &[isize]) -> Option<isize> {
    let mut possible_ops_combos = Vec::new();
    generate_operator_combinations(answer, &numbers, &[], &mut possible_ops_combos);
    println!("Testing {}: {:?}", answer, numbers);
    println!("Total possible ops combos: {:?}", possible_ops_combos);
    if possible_ops_combos
        .iter()
        .any(|ops| try_operators(answer, &numbers, &ops))
    {
        Some(answer)
    } else {
        None
    }
}

fn generate_operator_combinations(
    answer: isize,
    numbers: &[isize],
    ops: &[Operator],
    all: &mut Vec<Vec<Operator>>,
) {
    if numbers.len() == 1 {
        let mut ops_clone = ops.to_vec();
        ops_clone.reverse();
        all.push(ops_clone);
        return;
    }

    let last_number = numbers[numbers.len() - 1];
    if answer % last_number != 0 {
        let mut new_ops = ops.to_vec();
        new_ops.push(Operator::Add);
        generate_operator_combinations(
            answer - last_number,
            &numbers[..numbers.len() - 1],
            &new_ops,
            all,
        );
    } else {
        let mut new_ops = ops.to_vec();
        new_ops.push(Operator::Multiply);
        generate_operator_combinations(
            answer / last_number,
            &numbers[..numbers.len() - 1],
            &new_ops,
            all,
        );
        let mut new_ops = ops.to_vec();
        new_ops.push(Operator::Add);
        generate_operator_combinations(
            answer - last_number,
            &numbers[..numbers.len() - 1],
            &new_ops,
            all,
        );
    }
}

fn try_operators(answer: isize, numbers: &[isize], operators: &[Operator]) -> bool {
    let mut result = numbers[0];
    for i in 0..operators.len() {
        let op = &operators[i];
        let n = numbers[i + 1];
        match op {
            Operator::Add => result += n,
            Operator::Multiply => result *= n,
        };

        if result > answer {
            return false;
        }
    }
    result == answer
}

fn parse_line(line: &str) -> (isize, Vec<isize>) {
    let parts: Vec<&str> = line.split(':').collect();
    let numbers = parts
        .iter()
        .flat_map(|part| {
            part.split_whitespace()
                .filter_map(|s| s.parse::<isize>().ok())
        })
        .collect::<Vec<_>>();
    let (answer, numbers) = numbers.split_at(1);
    (answer[0], numbers.to_vec())
}
