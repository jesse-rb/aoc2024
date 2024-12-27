use std::{
    collections::{HashMap, HashSet},
    io::{self, Read},
};

use regex::Regex;

fn main() {
    let (rules, updates) = get_input();

    let part1 = part1(&rules, &updates);
    let part2 = part2(&rules, &updates);

    println!("part1: {}", part1);
    println!("part2: {}", part2);
}

fn get_input() -> (String, Vec<Vec<String>>) {
    let mut stdin = io::stdin();

    let mut input: String = String::new();
    let _ = stdin.read_to_string(&mut input);

    let mut parts = input.split("\n\n");

    let rules = parts.next().unwrap().to_string();

    let updates = parts
        .next()
        .unwrap()
        .trim()
        .split("\n")
        .map(|s| s.split(",").map(|d| d.trim().to_string()).collect())
        .collect();

    (rules, updates)
}

fn part1(rules: &str, updates: &[Vec<String>]) -> i32 {
    let mut total = 0;

    // FOR EACH updates
    //  FOR EACH page
    //   REGEX IT
    for update in updates {
        let mut processed_pages: Vec<String> = Vec::new();
        let mut safe: bool = true;

        for page in update {
            if processed_pages.is_empty() {
                // Special case: safe by default if first page in the update sequence
            } else {
                let pattern_before: &str = &format!(r"([0-9]+)\|{}", page);
                let pattern_after: &str = &format!(r"{}\|([0-9]+)", page);
                let re_before = Regex::new(pattern_before).expect("Invalid pattern");
                let re_after = Regex::new(pattern_after).expect("Invalid pattern");

                let matched_must_befores = re_before
                    .captures_iter(rules)
                    .filter_map(|m| m.get(1).map(|m| m.as_str()));
                let matched_must_afters = re_after
                    .captures_iter(rules)
                    .filter_map(|m| m.get(1).map(|m| m.as_str()));

                let mut one_before_rule_is_safe = false;

                for some_must_before in matched_must_befores {
                    if processed_pages.contains(&some_must_before.to_string()) {
                        one_before_rule_is_safe = true;
                        break; // Break out early
                    }
                }

                safe = one_before_rule_is_safe;

                for some_must_after in matched_must_afters {
                    if processed_pages.contains(&some_must_after.to_string()) {
                        safe = false;
                        break; // Break out early
                    }
                }
            }

            if !safe {
                break; // Break early if not safe
            }

            processed_pages.push(page.to_string());
        }

        if safe {
            let middle_index = processed_pages.len() / 2;
            let middle = processed_pages[middle_index].parse::<i32>().unwrap_or(0);

            total += middle;
        }
    }

    total
}

fn part2(rules: &str, updates: &[Vec<String>]) -> i32 {
    let mut total: i32 = 0;

    // Process our rules first to build out a map of valid entries
    let mut rules_map: HashMap<i32, HashSet<i32>> = HashMap::new();

    for rule in rules.split("\n") {
        let mut rule_parts = rule.split("|");
        let before = rule_parts.next().unwrap().parse::<i32>().unwrap();
        let after = rule_parts.next().unwrap().parse::<i32>().unwrap();

        if let Some(before_requirements_set) = rules_map.get_mut(&after) {
            before_requirements_set.insert(before);
        } else {
            rules_map.insert(after, HashSet::from([before]));
        }
    }

    // Now we can process our update lines checking if the state is valid according to our rules
    // and visited maps
    for update in updates {
        // Init a map for vitisted entries
        let mut visited: HashSet<i32> = HashSet::new();

        let mut valid = true;
        let mut middle = 0;

        let update_set: HashSet<i32> =
            HashSet::from_iter(update.iter().map(|x| x.parse::<i32>().unwrap()));

        for (i, page_str) in update.iter().enumerate() {
            let page: i32 = page_str.parse::<i32>().unwrap();

            if let Some(before_requirements_set) = rules_map.get(&page) {
                let this_update_before_requirements_set: HashSet<i32> = before_requirements_set
                    .intersection(&update_set)
                    .cloned()
                    .collect();

                if !this_update_before_requirements_set.is_empty()
                    && !this_update_before_requirements_set.is_subset(&visited)
                {
                    valid = false;
                }
            }

            if i == update.len() / 2 {
                middle = page;
            }

            visited.insert(page);

            if !valid {
                break;
            }
        }

        if valid {
            total += middle;
        }
    }

    total
}
