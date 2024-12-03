use std::{fs::File, io::{BufRead, BufReader}};

fn main() {
    let lines = get_input();

    let part1 = part1(&lines);

    println!("part1: NUM OF SAFE REPORTS {}", part1);
}

fn get_input() -> Vec<String> {
    let file = File::open("src/in.txt").unwrap();
    let reader = BufReader::new(file);
    reader.lines().map_while(Result::ok).collect()
}

fn part1(lines: &Vec<String>) -> i32 {
    let mut num_unsafe = 0;
    let mut total = 0;

    for report in lines {
        let mut prev: Option<i32> = None;
        let mut decreasing: Option<bool> = None;

        // Traverse over line
        println!("---STARTING---");
        for part in report.split_whitespace() {
            // must be i32
            let curr = part.parse::<i32>().expect("input broken");
            
            // Assume SAFE to start with
            let mut safe = true;

            if prev.is_some() {
                let diff = prev.unwrap() - curr;
                let diff_abs = diff.abs();


                safe = safe && diff_abs > 0;
                safe = safe && diff_abs <= 3;

                if decreasing.is_none() {
                    decreasing = Some(diff > 0);
                }

                if decreasing.is_some() {
                    if decreasing.unwrap() {
                        safe = safe && diff > 0;
                    }
                    else {
                        safe = safe && diff < 0;
                    }
                }

                println!("prev: {}, curr: {}, diff: {}, diff_abs: {}, decreasing: {}", prev.unwrap(), curr, diff, diff_abs, decreasing.unwrap());
            }

            prev = Some(curr);

            if !safe {
                println!("---FOUND UNSAFE---");
                num_unsafe += 1;
                break;
            }
        }
        total += 1;
    }

    println!("TOTAL REPORTS: {}", total);
    println!("TOTAL UNSAFE REPORTS: {}", num_unsafe);
    total - num_unsafe
}
