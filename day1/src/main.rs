use std::{collections::{HashMap}, fs::File, io::{BufRead, BufReader, Result}};

/**
 * Find the total of the difference between sorted pairs
 */
fn main() {
    let lines = get_input();

    let part1 = part1(&lines);
    let part2 = part2(&lines);

    println!("part1: {}", part1);
    println!("part2: {}", part2);
}

fn get_input() -> Vec<String> {
    let file = File::open("src/in.txt").unwrap();
    let reader = BufReader::new(file);
    reader.lines().map_while(Result::ok).collect()
}

fn part1(lines: &[String]) -> i32 {
    
    let mut a: Vec<i32> = Vec::new();
    let mut b: Vec<i32> = Vec::new();

    for line in lines {
        let (first, second) = process_line(line).unwrap();

        a.push(first);
        b.push(second);
    }

    a.sort();
    b.sort();

    sum_diffs(&a, &b)
}

fn part2(lines: &[String]) -> i32 {
    let mut total: i32 = 0;
    let mut counts: HashMap<i32, i32> = HashMap::new();

    for line in lines {
        let (_first, second) = process_line(line).unwrap();

        let count = counts.get(&second).unwrap_or(&0);
        counts.insert(second, count + 1);
    }
    
    for line in lines {
        let (first, _second) = process_line(line).unwrap();

        let count = counts.get(&first).unwrap_or(&0);
        total += first * *count;
    }

    total
}

fn process_line(line: &str) -> Option<(i32, i32)> {
    let mut parts = line.split_whitespace();

    let first = parts.next().expect("input broken").parse::<i32>().expect("input broken");
    let second = parts.next().expect("input broken").parse::<i32>().expect("input broken");

    Some((first, second))
}

fn sum_diffs(a: &[i32], b: &[i32]) -> i32 {
    let mut total = 0;

    for (first, second) in a.iter().zip(b.iter()) {
        let distance = (first - second).abs();
        total += distance; 
    }

    total
}

