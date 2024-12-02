use std::{fs::File, io::{BufRead, BufReader}};

fn main() {
    let lines = get_input();

    let part1 = part1(&lines);

    println!("part1: {}", part1)

}

fn get_input() -> Vec<String> {
    let file = File::open("src/in.txt").unwrap();
    let reader = BufReader::new(file);
    reader.lines().map_while(Result::ok).collect()
}

fn part1(lines: &Vec<String>) -> i32 {
    let mut num_unsafe = 0;
    let mut total = 0;

    for line in lines {
        let mut prev: Option<i32> = None;
        let mut decreasing: Option<bool> = None;

        for part in line.split_whitespace() {
            let curr = part.parse::<i32>().expect("input broken");
    
            let mut safe = true;
            
            if prev.is_some() {
                if prev == Some(curr) {
                    safe = false;
                }
                else if decreasing.is_some() {
                    if decreasing == Some(true) {
                        safe = (prev < Some(curr)) && (prev >= Some(curr - 3));
                    }
                    else {
                        safe = (prev > Some(curr)) && (prev <= Some(curr + 3));
                    }
                }
            }

            if decreasing.is_none() {
                decreasing = Some(prev > Some(curr));
            }

            prev = Some(curr);

            total += 1;
            if !safe {
                num_unsafe += 1;
                break;
            }
        }
    }

    total - num_unsafe
}
