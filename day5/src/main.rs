use std::io::{self, Read};

use regex::Regex;

fn main() {
    println!("Hello day 5!");

    let (rules, updates) = get_input();

    let part1 = part1(&rules, &updates);

    println!("part1: {}", part1);
}

fn get_input() -> (String, Vec<Vec<String>>) {
    let mut stdin = io::stdin();

    let mut input: String = String::new();
    let _ = stdin.read_to_string(&mut input);

    let mut parts = input.split("\n\n");

    let rules = parts
        .next()
        .unwrap_or("").to_string();

    let updates = parts
        .next()
        .unwrap_or("")
        .split("\n")
        .map(
            |s| s.split(",")
                .map(|d| d.to_string())
                .collect()
        )
        .collect();

    (rules, updates)
}

fn part1(rules: &str, updates: &[Vec<String>]) -> i32 {
    let mut total = 0;

    // FOR EACH updates
    //  FOR EACH page
    //   REGEX IT
    for update in updates {
        println!("----NEW UPDATE----");
        let mut processed_pages: Vec<String> = Vec::new();
        let mut safe: bool = true;

        for page in update {
            println!("--\ncurrent: {}", page);
            if processed_pages.is_empty() {
                // Special case: safe by default if first page in the update sequence
            }
            else {
                let pattern_before: &str = &format!(r"([0-9]+)\|{}", page);
                let pattern_after: &str = &format!(r"{}\|([0-9]+)", page);
                let re_before = Regex::new(pattern_before).expect("Invalid pattern");
                let re_after = Regex::new(pattern_after).expect("Invalid pattern");

                let matched_must_befores = re_before.captures_iter(rules).filter_map(|m| m.get(1).map(|m| m.as_str()));
                let matched_must_afters = re_after.captures_iter(rules).filter_map(|m| m.get(1).map(|m| m.as_str()));

                let mut one_before_rule_is_safe = false;

                for some_must_before in matched_must_befores {
                    println!("must be before: {:?}", some_must_before);
                    println!("processed: {:?}", processed_pages);
                    
                    if processed_pages.contains(&some_must_before.to_string()) {
                        one_before_rule_is_safe = true;
                        break; // Break out early
                    }
                }

                safe = one_before_rule_is_safe;

                for some_must_after in matched_must_afters {
                    println!("must be after: {:?}", some_must_after);
                    println!("processed: {:?}", processed_pages);

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


        println!("update: {:?}\nsafe: {}", update, safe);

        if safe {
            let middle_index = processed_pages.len()/2;
            let middle = processed_pages[middle_index].parse::<i32>().unwrap_or(0);

            total += middle;
        }
    }
    

    total
}
