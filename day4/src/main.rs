use std::{io, str::Chars};

fn main() {
    let lines = get_input();

    let part1 = part1(&lines);
    let part2 = part2(&lines);

    println!("part1: {}", part1);
    println!("part2: {}", part2);
}

fn get_input() -> Vec<Vec<char>> {
    let stdin = io::stdin();
    let lines = stdin.lines();
    lines.map_while(|line| line.ok())
        .map(|line| line.chars().collect())
        .collect()
}

fn test(lines: &[Vec<char>], current_x: usize, current_y: usize, test_char: char) -> bool {
    let current_line = lines.get(current_y);
    let mut pass: bool = false;


    if current_line.is_some() {
        let current_char = current_line.unwrap().get(current_x);

        println!("Comparing {:?} at ({}, {}) -> {}", current_char, current_x, current_y, test_char);
        if current_char.is_some() {
            pass = *current_char.unwrap() == test_char;
        }
    }

    pass
}

#[derive(PartialEq)] // Derives the PartialEq trait
enum Direction {
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
    TopLeft,
    Top,
    TopRight,
}

fn search(lines: &[Vec<char>], current_x: usize, current_y: usize, mut search_chars: Chars, lock_direction: Option<Direction>) -> i32 {
    let mut total: i32 = 0;

    let search_char = search_chars.next();
    
    if search_char.is_some() && test(lines, current_x, current_y, search_char.unwrap()) {
        println!("{} ({},{})", search_char.unwrap(), current_x, current_y);

        let next_search_char = search_chars.clone().next();
        if next_search_char.is_none() {
            total += 1;
            println!("YAY, total is {}", total);
        }
        else {
            let has_direction = lock_direction.is_none();
            let direction = lock_direction.or(None);

            if has_direction || direction == Some(Direction::Right) {
                // right
                total += search(lines, current_x + 1, current_y, search_chars.clone(), Some(Direction::Right));
            }
            if has_direction || direction == Some(Direction::BottomRight) {
                // bottom-right
                total += search(lines, current_x + 1, current_y + 1, search_chars.clone(), Some(Direction::BottomRight));
            }
            if has_direction || direction == Some(Direction::Bottom) {
                // bottom
                total += search(lines, current_x, current_y + 1, search_chars.clone(), Some(Direction::Bottom));
            }
            if has_direction || direction == Some(Direction::BottomLeft) {
                // bottom-left
                if current_x > 0 {
                    total += search(lines, current_x - 1, current_y + 1, search_chars.clone(), Some(Direction::BottomLeft));
                }
            }
            if has_direction || direction == Some(Direction::Left) {
                // left
                if current_x > 0 {
                    total += search(lines, current_x - 1, current_y, search_chars.clone(), Some(Direction::Left));
                }
            }
            if has_direction || direction == Some(Direction::TopLeft) {
                // top-left
                if current_x > 0 && current_y > 0 {
                    total += search(lines, current_x - 1, current_y - 1, search_chars.clone(), Some(Direction::TopLeft));
                }
            }
            if has_direction || direction == Some(Direction::Top) {
                // top
                if current_y > 0 {
                    total += search(lines, current_x, current_y - 1, search_chars.clone(), Some(Direction::Top));
                }
            }
            if has_direction || direction == Some(Direction::TopRight) {
                // top-right
                if current_y > 0 {
                    total += search(lines, current_x + 1, current_y - 1, search_chars.clone(), Some(Direction::TopRight));
                }
            }
        }
    }

    total
}

/**
 * Process input left -> right, top -> bottom
 * IF we found X
 *     THEN look for M right, right-bottom, and bottom
 *         IF recursively look for next char until no match
 */
fn part1(lines: &[Vec<char>]) -> i32 {
    let mut total: i32 = 0;

    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            total += search(lines, x, y, "XMAS".chars(), None);
        }
        println!("-----TOTAL BY THE END OF line {}: {}", y, total);
    }

    total
}

fn check_criss_cross(lines: &[Vec<char>], x: usize, y: usize) -> bool {
    if y < 1 || x < 1 || y >= lines.len()-1 || x >= lines[y].len()-1 {
        return false;
    }

    let top_left_is_m:bool = test(lines, x-1, y-1, 'M');
    let top_left_is_s:bool = test(lines, x-1, y-1, 'S');

    let bottom_right_is_m:bool = test(lines, x+1, y+1, 'M');
    let bottom_right_is_s:bool = test(lines, x+1, y+1, 'S');

    let top_right_is_m:bool = test(lines, x+1, y-1, 'M');
    let top_right_is_s:bool = test(lines, x+1, y-1, 'S');

    let bottom_left_is_m:bool = test(lines, x-1, y+1, 'M');
    let bottom_left_is_s:bool = test(lines, x-1, y+1, 'S');

    let criss:bool = (top_left_is_m && bottom_right_is_s) || (top_left_is_s && bottom_right_is_m);
    let cross:bool = (top_right_is_m && bottom_left_is_s) || (top_right_is_s && bottom_left_is_m);

    criss && cross
}

fn part2(lines: &[Vec<char>]) -> i32 {
    let mut total: i32 = 0;

    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            // IF WE FOUND AN A
            if test(lines, x, y, 'A') {
                // THEN CHECK FOR the criss-cross X MAS
                if check_criss_cross(lines, x, y) {
                    total += 1;
                }
            }
        }
        println!("-----TOTAL BY THE END OF line {}: {}", y, total);
    }

    total
}
