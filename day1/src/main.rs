use std::{fs::File, io::{BufRead, BufReader}};

/**
 * Find the total of the difference between sorted pairs
 */
fn main() {
    let total = read_file();

    print!("{}", total)
}

fn read_file() -> i32 {
    let file = File::open("src/in.txt").unwrap();

    let reader = BufReader::new(file);

    let lines = reader.lines();

    let mut a: Vec<i32> = Vec::new();
    let mut b: Vec<i32> = Vec::new();

    for line in lines {
        process_line(&line.unwrap(), &mut a, &mut b);
        
        a.sort();
        b.sort();
    }

    sum_diffs(&a, &b)
}

fn process_line(line: &str, a: &mut Vec<i32>, b: &mut Vec<i32>) {
    let mut parts = line.split_whitespace();

    let first = parts.next().expect("input broken").parse::<i32>().expect("input broken");
    let second = parts.next().expect("input broken").parse::<i32>().expect("input broken");
    a.push(first);
    b.push(second);
}

fn sum_diffs(a: &[i32], b: &[i32]) -> i32 {
    let mut total = 0;

    for (first, second) in a.iter().zip(b.iter()) {
        let distance = (first - second).abs();
        total += distance; 
    }

    total
}

