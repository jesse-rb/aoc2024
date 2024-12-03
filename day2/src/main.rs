use std::{fs::File, io::{BufRead, BufReader}};

fn main() {
    let lines = get_input();

    let part1 = part1(&lines);
    let part2 = part2(&lines);

    println!("part1: NUM OF SAFE REPORTS {}", part1);
    println!("part2: {}", part2);
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
        // println!("---STARTING---");
        for part in report.split_whitespace() {
            // must be i32
            let curr = part.parse::<i32>().expect("input broken");
            
            // Assume SAFE to start with
            let mut safe = true;

            // IF we have a previous to campare with
            if prev.is_some() {
                // get DIFF
                let diff = prev.unwrap() - curr;
                // get DIFF abs
                let diff_abs = diff.abs();

                // MUST NOT BE EQUAL
                safe = safe && diff_abs > 0;
                // MUST NOT HAVE ABS DIFF GREATER THAN 3
                safe = safe && diff_abs <= 3;

                if decreasing.is_none() {
                    // SET ONLY the initial "is decreasing" flag
                    decreasing = Some(diff > 0);
                }

                if decreasing.is_some() {
                    // IF decreasing, diff must be greater than 1
                    if decreasing.unwrap() {
                        safe = safe && diff > 0;
                    }
                    // IF increasing, diff must be less than 1
                    else {
                        safe = safe && diff < 0;
                    }
                }

                // println!("prev: {}, curr: {}, diff: {}, diff_abs: {}, decreasing: {}", prev.unwrap(), curr, diff, diff_abs, decreasing.unwrap());
            }

            prev = Some(curr);

            if !safe {
                // println!("---FOUND UNSAFE---");
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

fn part2(lines: &Vec<String>) -> i32 {
    let mut num_unsafe = 0;
    let mut total = 0;

    for report in lines {
        let mut problems_dampened = false;
        let mut prev: Option<&str> = None;
        let mut decreasing: Option<bool> = None;

        let mut parts = report.split_whitespace();

        let mut curr = parts.next();

        // Traverse over line
        println!("---STARTING---");
        // for part in report.split_whitespace() {
        while curr.is_some() {
            // must be i32
            let curr_val = curr.unwrap().parse::<i32>().expect("input broken");
            
            // Assume SAFE to start with
            let mut safe = true;

            // IF we have a previous to campare with
            if prev.is_some() {
                let prev_val = prev.unwrap().parse::<i32>().expect("input broken");

                // get DIFF
                let diff = prev_val - curr_val;
                // get DIFF abs
                let diff_abs = diff.abs();

                // MUST NOT BE EQUAL
                safe = safe && diff_abs > 0;
                // MUST NOT HAVE ABS DIFF GREATER THAN 3
                safe = safe && diff_abs <= 3;

                if decreasing.is_none() {
                    // SET ONLY the initial "is decreasing" flag
                    decreasing = Some(diff > 0);
                }

                if decreasing.is_some() {
                    // IF decreasing, diff must be greater than 1
                    if decreasing.unwrap() {
                        safe = safe && diff > 0;
                    }
                    // IF increasing, diff must be less than 1
                    else {
                        safe = safe && diff < 0;
                    }
                }

                println!("prev_val: {}, curr_val: {}, diff: {}, diff_abs: {}, decreasing: {}", prev_val, curr_val, diff, diff_abs, decreasing.unwrap());
            }

            

            if !safe {
                println!("---FOUND UNSAFE---");
                if problems_dampened {
                    println!("EOL");
                    num_unsafe += 1;
                    break;
                }
                else {
                    println!("DAMPENED");
                    problems_dampened = true;
                    curr = parts.next();
                }
            }
            else {
                prev = curr;
                curr = parts.next();
            }
        }
        total += 1;
    }

    println!("TOTAL REPORTS: {}", total);
    println!("TOTAL UNSAFE REPORTS: {}", num_unsafe);

    // BET: 301 & 383
    total - num_unsafe
}

