use std::io::{self, Read};

use regex::Regex;

fn main() {
    let input = get_input();

    let part1 = part1(&input);
    //let part2 = part2(&lines);

    println!("part1: {}", part1);
    //println!("part2: {}", part2);
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

        println!("tuple: ({}, {})", a, b);

        total += a * b;
    }

    total
}

