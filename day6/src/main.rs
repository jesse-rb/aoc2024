use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    io,
    path::absolute,
    thread::sleep,
    time::Duration,
};

const DIRECTIONS: &[(char, (i32, i32))] =
    &[('^', (-1, 0)), ('>', (0, 1)), ('v', (1, 0)), ('<', (0, -1))];

fn main() {
    let input = get_input();

    let part1 = part1(input);

    println!("part 1: {}", part1);
}

fn get_input() -> Vec<Vec<char>> {
    let stdin = io::stdin();
    let grid = stdin
        .lines()
        .map(|y| y.unwrap().chars().collect())
        .collect();

    grid
}

fn ui_clear() {
    print!("\x1B[2J"); // Clear screen
}

fn ui_move_cursor_to_top_left() {
    print!("\x1B[H"); // Move cursor to top-left
}

fn ui_render(grid: &Vec<Vec<char>>, guard_y: usize, guard_x: usize) {
    ui_clear();
    ui_move_cursor_to_top_left();

    let min_res_y: usize = 40;
    let mut min_y = max(0, guard_y as i32 - min_res_y as i32 / 2) as usize;
    let mut max_y = min(grid.len() - 1, guard_y + min_res_y / 2);

    if min_y == 0 {
        max_y = min(grid.len() - 1, min_res_y);
    }
    if max_y >= grid.len() - 1 {
        min_y = max(0, max_y - min_res_y);
    }

    for y in min_y..max_y {
        let min_res_x: usize = 50;

        let mut min_x = max(0, guard_x as i32 - min_res_x as i32 / 2) as usize;
        let mut max_x = min(grid[y].len() - 1, guard_x + min_res_x / 2);

        if min_x == 0 {
            max_x = min(grid[y].len() - 1, min_res_x);
        }
        if max_x >= grid[y].len() - 1 {
            min_x = max(0, max_x - min_res_x);
        }

        for x in min_x..max_x {
            print!("{}", grid[y][x]);
        }

        print!("\n");
    }

    sleep(Duration::from_millis(25));
}

fn is_guard(c: char) -> bool {
    let directions_set: HashSet<char> = DIRECTIONS.iter().map(|(k, _)| *k).collect();
    directions_set.contains(&c)
}

fn peek_next(grid: &Vec<Vec<char>>, y: usize, x: usize, direction: (i32, i32)) -> char {
    let mut next: char = '\0';

    let (offset_y, offset_x) = direction;

    if !((y == 0 && offset_y == -1) || (x == 0 && offset_x == -1)) {
        let peek_y = (y as i32 + offset_y) as usize;
        let peek_x = (x as i32 + offset_x) as usize; // :,(

        if peek_y < grid.len() && peek_x < grid[peek_y].len() {
            next = grid[peek_y][peek_x];
        }
    }

    next
}

fn cycle_direction(
    directions_map: &HashMap<char, (i32, i32)>,
    direction: (i32, i32),
) -> (i32, i32) {
    let mut new_direction = direction;

    if direction == directions_map[&'^'] {
        new_direction = directions_map[&'>'];
    }
    if direction == directions_map[&'>'] {
        new_direction = directions_map[&'v'];
    }
    if direction == directions_map[&'v'] {
        new_direction = directions_map[&'<'];
    }
    if direction == directions_map[&'<'] {
        new_direction = directions_map[&'^'];
    }

    new_direction
}

fn part1(mut grid: Vec<Vec<char>>) -> i32 {
    let directions_map: HashMap<char, (i32, i32)> = DIRECTIONS.iter().cloned().collect();
    let inverse_directions_map: HashMap<(i32, i32), char> = directions_map
        .iter()
        .map(|(&key, &value)| (value, key))
        .collect();
    let mut total = 0;

    // Initial processing of our grid to find the guard pos
    let mut guard_x: usize = 0;
    let mut guard_y: usize = 0;
    let mut direction: (i32, i32) = (0, 0);

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if is_guard(grid[y][x]) {
                guard_y = y;
                guard_x = x;

                direction = directions_map[&grid[y][x]];
            }
        }
    }

    let mut done = false;

    while !done {
        // Check
        ui_render(&grid, guard_y, guard_x);

        let (offset_y, offset_x) = direction;
        let peek_next = peek_next(&grid, guard_y, guard_x, direction);

        match peek_next {
            '\0' => {
                done = true;
            }
            '#' => {
                direction = cycle_direction(&directions_map, direction);

                grid[guard_y][guard_x] = inverse_directions_map[&direction];
            }
            _ => {
                let new_guard_y = (guard_y as i32 + offset_y) as usize;
                let new_guard_x = (guard_x as i32 + offset_x) as usize;

                let swap = grid[new_guard_y][new_guard_x];
                grid[new_guard_y][new_guard_x] = grid[guard_y][guard_x];
                grid[guard_y][guard_x] = swap;

                guard_y = new_guard_y;
                guard_x = new_guard_x;

                total += 1;
            }
        }
    }

    total
}
