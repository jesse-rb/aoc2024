use std::io::{self, Read};

use regex::Regex;

fn main() {
    let input = get_input();

    let part1 = part1(&input);
    let part2 = part2(&input);

    println!("part1: {}", part1);
    println!("part2: {}", part2);
}

fn get_input() -> String {
    let mut buf:String = String::new();
    let _ = io::stdin().read_to_string(&mut buf).expect("Failed reading stdin as string");

    buf
}

fn part1(input: &str) -> i32 {
    let mut total: i32 = 0;

    let pattern: &str = r"mul\((\d+),(\d+)\)";
    let re = Regex::new(pattern).expect("Invalid regex");

    let matches = re.captures_iter(input).map(|m| {
        let a = m.get(1)?.as_str().parse::<i32>().ok()?;
        let b = m.get(2)?.as_str().parse::<i32>().ok()?;

        Some((a, b))
    });

    for m in matches {
        let (a, b) = m.unwrap();

        //println!("tuple: ({}, {})", a, b);

        total += a * b;
    }

    total
}

fn part2(input: &str) -> i32 {
    let mut total: i32 = 0;

    let pattern: &str = r"(?x) # Verbose mode for comments and whitespace (readability)
        (?P<do>do\(\))
        |(?P<dont>don't\(\))
        |(?P<mul>mul\((\d+),(\d+)\))
    ";
    let re = Regex::new(pattern).expect("Invalid regex");
    let matches = re.captures_iter(input);

    let mut is_do: bool = true;
    let mut to_sum:Vec<(i32, i32)> = Vec::new();

    for m in matches {
        if m.name("do").is_some() {
            println!("found do");
            is_do = true;
        }
        else if m.name("dont").is_some() {
            println!("found don't");
            is_do = false;
        }
        else if m.name("mul").is_some() && is_do {
            println!("found mul");
            let a:Option<i32> = m.get(4).and_then(|r| r.as_str().parse::<i32>().ok());
            let b:Option<i32> = m.get(5).and_then(|r| r.as_str().parse::<i32>().ok());

            if let (Some(some_a), Some(some_b)) = (a, b) {
                to_sum.push((some_a, some_b));
            }
        }
    }

    for m in to_sum {
        let (a, b) = m;

        println!("tuple: ({}, {})", a, b);

        total += a * b;
    }

    total
}

