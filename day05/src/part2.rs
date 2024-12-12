use std::collections::HashSet;

pub fn solve(input: &[String]) -> i32 {
    let (rules, page_updates) = read_input(input);

    page_updates
        .iter()
        .filter(|page_update| !check_for_rule_violations(&rules, page_update).is_empty())
        .filter_map(|page_update| correct_update(&rules, page_update))
        .map(|corrected_update| {
            corrected_update[corrected_update.len() / 2]
                .parse::<i32>()
                .unwrap()
        })
        .sum()
}

fn check_for_rule_violations(rules: &HashSet<String>, page_update: &Vec<String>) -> Vec<String> {
    let mut violations: Vec<String> = Vec::new();
    for i in 0..page_update.len() {
        for j in (i + 1)..page_update.len() {
            let inverse_rule = format!("{}|{}", page_update[j], page_update[i]);
            if rules.contains(&inverse_rule) {
                violations.push(inverse_rule);
            }
        }
    }
    violations
}

fn correct_update(rules: &HashSet<String>, page_update: &Vec<String>) -> Option<Vec<String>> {
    let mut corrected_update = page_update.clone();

    loop {
        let violations = check_for_rule_violations(rules, &corrected_update);
        if violations.is_empty() {
            return Some(corrected_update);
        };
        for violation in violations {
            let parts = violation
                .split('|')
                .flat_map(str::parse)
                .collect::<Vec<String>>();
            let a = &parts[0];
            let b = &parts[1];
            let a_index = corrected_update.iter().position(|c| c == a).unwrap();
            let b_index = corrected_update.iter().position(|c| c == b).unwrap();
            corrected_update.swap(a_index, b_index);
        }
    }
}

fn read_input(input: &[String]) -> (HashSet<String>, Vec<Vec<String>>) {
    let mut rules: HashSet<String> = HashSet::new();
    let mut page_updates: Vec<Vec<String>> = Vec::new();

    let mut loading_rules = true;
    for line in input {
        if line.is_empty() {
            loading_rules = false;
        } else if loading_rules {
            rules.insert(line.to_string());
        } else {
            page_updates.push(
                line.split(',')
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>(),
            );
        }
    }
    (rules, page_updates)
}
