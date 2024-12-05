use std::collections::HashSet;

pub fn solve(input: impl Iterator<Item = String>) -> i32 {
    let (rules, page_updates) = read_input(input);

    page_updates
        .iter()
        .filter_map(|page_update| {
            if is_good_update(&rules, page_update) {
                let middle_page = page_update[page_update.len() / 2];
                Some(middle_page)
            } else {
                None
            }
        })
        .sum()
}

fn is_good_update(rules: &HashSet<String>, page_update: &Vec<i32>) -> bool {
    // the rules are stored unmodified as a HashSet of strings
    // interating over each pair of updates and checking if their inverse rule is in the set
    for i in 0..page_update.len() {
        for j in (i + 1)..page_update.len() {
            let inverse_rule = format!("{}|{}", page_update[j], page_update[i]);
            if rules.contains(&inverse_rule) {
                return false;
            }
        }
    }
    true
}

fn read_input(input: impl Iterator<Item = String>) -> (HashSet<String>, Vec<Vec<i32>>) {
    let mut rules: HashSet<String> = HashSet::new();
    let mut page_updates: Vec<Vec<i32>> = Vec::new();

    let mut loading_rules = true;
    for line in input {
        if line.is_empty() {
            loading_rules = false;
        } else if loading_rules {
            rules.insert(line);
        } else {
            page_updates.push(line.split(',').map(|s| s.parse().unwrap()).collect());
        }
    }
    (rules, page_updates)
}
