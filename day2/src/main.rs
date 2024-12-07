use std::{fs::File, io::{BufRead, BufReader}};

fn main() {
    let lines = get_input();

    let part1 = part1(&lines);
    let part2 = part2(&lines);

    println!("part1: NUM OF SAFE REPORTS {}", part1);
    println!("part2: {}", part2);
}

fn get_input() -> Vec<String> {
    //let file = File::open("src/in-mini.txt").unwrap();
    let file = File::open("src/in.txt").unwrap();
    let reader = BufReader::new(file);
    reader.lines().map_while(Result::ok).collect()
}

fn part1(lines: &Vec<String>) -> i32 {
    let mut num_unsafe = 0;
    let mut total = 0;

    for report in lines {
        let mut prev: Option<&str> = None;
        let mut decreasing: Option<bool> = None;

        // Traverse over line
        // println!("---STARTING---");
        for part in report.split_whitespace() {
            let curr = Some(part);

            let safe = compare_is_safe(prev, curr, &mut decreasing);
            
            //// Assume SAFE to start with
            //let mut safe = true;
            //
            //// IF we have a previous to campare with
            //if prev.is_some() {
            //    // get DIFF
            //    let diff = prev.unwrap() - curr;
            //    // get DIFF abs
            //    let diff_abs = diff.abs();
            //
            //    // MUST NOT BE EQUAL
            //    safe = safe && diff_abs > 0;
            //    // MUST NOT HAVE ABS DIFF GREATER THAN 3
            //    safe = safe && diff_abs <= 3;
            //
            //    if decreasing.is_none() {
            //        // SET ONLY the initial "is decreasing" flag
            //        decreasing = Some(diff > 0);
            //    }
            //
            //    if decreasing.is_some() {
            //        // IF decreasing, diff must be greater than 1
            //        if decreasing.unwrap() {
            //            safe = safe && diff > 0;
            //        }
            //        // IF increasing, diff must be less than 1
            //        else {
            //            safe = safe && diff < 0;
            //        }
            //    }
            //
            //    // println!("prev: {}, curr: {}, diff: {}, diff_abs: {}, decreasing: {}", prev.unwrap(), curr, diff, diff_abs, decreasing.unwrap());
            //}
            prev = curr;

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

/**
 * This is insane
 */
fn part2(lines: &Vec<String>) -> i32 {
    let mut num_unsafe = 0;
    let mut total = 0;

    for report in lines {
        let mut problems_dampened = false;
        let mut decreasing: Option<bool> = None;

        let mut index = 0;

        let parts: Vec<&str> = report.split_whitespace().collect();

        // Traverse over line
        println!("---STARTING---");
        println!("report: {}", report);
        // for part in report.split_whitespace() {
        while index < parts.len() {
            let curr = Some(parts[index]);
            let prev = if index > 0 { Some(parts[index-1]) } else { None };
            let safe = compare_is_safe(prev, curr, &mut decreasing);

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
                    // The problem has been dampened, now we must decide which item is best to
                    // remove, this or the previous item
                    
                    // THIS IS THE LAST ITEM, SO WE R GOOD
                    if index + 1 >= parts.len() {
                        println!("THIS IS THE LAST ITEM, SO WE ARE GOOD");
                        index += 1;
                        continue;
                    }

                    // SEE WHAT HAPPENS IF WE REMOVE CURR
                    println!("SEE WHAT HAPPENS IF WE REMOVE curr");
                    let x_next = Some(parts[index+1]);
                    let mut x_decreasing = if index > 1 { decreasing } else { None };
                    let x_test = compare_is_safe(prev, x_next, &mut x_decreasing);
                    // If this worked, we continute
                    if x_test && look_ahead(&parts, &mut x_decreasing, index+2, index + 4) {
                        index += 2;
                        decreasing = x_decreasing;
                    }
                    else {
                        // OTHERWISE SEE WHAT HAPPENS IF WE REMOVE PREV
                        println!("OTHERWISE SEE WHAT HAPPENS IF WE REMOVE prev");
                        if index < 2 {
                            println!("actually, there is nothing before prev so we can continue");
                            index += 1;
                            decreasing = None; // remember to reset decreasing memory
                            continue;
                        }
                        else {
                            let y_prev = Some(parts[index-2]);
                            let mut y_decreasing = if index > 2 { decreasing } else { None };
                            let y_test = compare_is_safe(y_prev, curr, &mut y_decreasing);

                            if y_test && look_ahead(&parts, &mut y_decreasing, index+1, index + 4) {
                                index += 1;

                                decreasing = y_decreasing;
                            }
                            else {
                                // CHECK a special case to see if we can remove the very first item
                                // to make it SAFE
                                if index == 2 {
                                    //index -= 1;
                                    decreasing = None; // remember to reset decreasing memory
                                } 
                                else {
                                    println!("DAMPENER OVERLOADED DETECTED -- FAILING");
                                    num_unsafe += 1;
                                    break;
                                }
                            }
                        }
                    }
                }
                println!("continuing from dampened, onto index: {}", index);
            }
            else {
                index += 1;
            }
        }
        println!("num_unsafe: {}", num_unsafe);
        total += 1;
    }

    println!("TOTAL REPORTS: {}", total);
    println!("TOTAL UNSAFE REPORTS: {}", num_unsafe);

    // BET: 301 & 383
    total - num_unsafe
}

/**
 * Check test the next four iterations
 */
fn look_ahead(parts: &[&str], is_decreasing: &mut Option<bool>, mut index: usize, to: usize) -> bool {
    while index < to && index < parts.len() {
        println!("LOOKING AHEAD at index: {}", index);
        let curr = Some(parts[index]);
        let prev = if index > 0 { Some(parts[index-1]) } else { None };
        let safe = compare_is_safe(prev, curr, is_decreasing);
        if safe {
            index += 1
        }
        else {
            println!("LOOK AHEAD FAILED");
            return false;
        }
    }

    true
}

fn compare_is_safe(a: Option<&str>, b: Option<&str>, is_decreasing: &mut Option<bool>) -> bool {
    // must be i32
    let curr_val = b.unwrap().parse::<i32>().expect("input broken");

    // Assume SAFE to start with
    let mut safe = true;

    // IF we have a previous to campare with
    if a.is_some() {
        let prev_val = a.unwrap().parse::<i32>().expect("input broken");

        // get DIFF
        let diff = prev_val - curr_val;
        // get DIFF abs
        let diff_abs = diff.abs();

        // MUST NOT BE EQUAL
        safe = safe && diff_abs > 0;
        // MUST NOT HAVE ABS DIFF GREATER THAN 3
        safe = safe && diff_abs <= 3;

        if is_decreasing.is_none() {
            // SET ONLY the initial "is decreasing" flag
            *is_decreasing = Some(diff > 0);
        }

        if is_decreasing.is_some() {
            // IF decreasing, diff must be greater than 1
            if is_decreasing.unwrap() {
                safe = safe && diff > 0;
            }
            // IF increasing, diff must be less than 1
            else {
                safe = safe && diff < 0;
            }
        }

        println!("prev_val: {}, curr_val: {}, diff: {}, diff_abs: {}, decreasing: {}", prev_val, curr_val, diff, diff_abs, if is_decreasing.is_some() { if is_decreasing.unwrap() { "DECREASING" } else { "INCREASING" } } else { "NONE" });
    }

    safe
}

