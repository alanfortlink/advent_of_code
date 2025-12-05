use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn get_lines() -> Vec<String> {
    let mut args = std::env::args();
    let input_src = args.nth(1).unwrap();

    let file = File::open(input_src).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|v| v.unwrap())
        .filter(|v| !v.is_empty())
        .collect::<Vec<_>>()
}

fn part1() {
    let lines = get_lines();
}

fn part2() {
    let lines = get_lines();
}

fn main() {
    part1();
    part2();
}
