use std::io;

fn main() {
    let lines = get_input();

    let part1 = part1(&lines);
    //let part2 = part2(&lines);

    println!("part1: {}", part1);
    //println!("part2: {}", part2);
}

fn get_input() -> Vec<String> {
    let stdin = io::stdin();
    let lines = stdin.lines();
    lines.map_while(Result::ok).collect()
}

fn part1(lines: &[String]) -> i32 {

    2
}

