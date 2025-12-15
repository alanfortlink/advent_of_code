use nalgebra::{DMatrix, DVector};
use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

fn get_lines() -> Vec<String> {
    let mut args = std::env::args();

    let input_src = if args.len() >= 2 {
        args.nth(1).unwrap()
    } else {
        String::from("example.txt")
    };

    let file = File::open(input_src).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|v| v.unwrap())
        //.filter(|v| !v.is_empty())
        .collect::<Vec<_>>()
}

struct Instance {
    target: u64,
    buttons: Vec<u64>,
    joltage_requirements: Vec<u64>,
}

fn parse_target(encoded: &str) -> u64 {
    return encoded
        .trim_matches(|c| c == '[' || c == ']')
        .chars()
        .enumerate()
        .map(|(i, c)| if c == '#' { 2_u64.pow(i as u32) } else { 0 })
        .sum();
}

fn parse_button(encoded: &str) -> u64 {
    return encoded
        .trim_matches(|c| c == '(' || c == ')')
        .split(",")
        .map(|i| 2_u64.pow(i.parse::<u32>().unwrap()))
        .sum();
}

fn parse_joltage_requirements(encoded: &str) -> Vec<u64> {
    return encoded
        .trim_matches(|c| c == '{' || c == '}')
        .split(",")
        .map(|v| v.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
}

fn parse_line(line: &String) -> Instance {
    let mut parts = line.split_whitespace();
    let target = parse_target(parts.next().unwrap());

    let mut parts = parts.rev();
    let joltage_requirements = parse_joltage_requirements(parts.next().unwrap());

    let buttons = parts.map(parse_button).collect::<Vec<_>>();

    Instance {
        target,
        buttons,
        joltage_requirements,
    }
}

fn part1() {
    let lines = get_lines();

    let instances = lines.iter().map(parse_line).collect::<Vec<_>>();
    let mut res = 0;

    for instance in instances {
        let mut queue: VecDeque<(u64, u64)> = VecDeque::new();
        let mut seen: HashSet<u64> = HashSet::new();

        queue.push_back((0, 0));

        while !queue.is_empty() {
            let (state, presses) = queue.pop_front().unwrap();
            if state == instance.target {
                res += presses;
                break;
            }

            for button in instance.buttons.iter() {
                let next_state = state ^ button;
                if !seen.contains(&next_state) {
                    queue.push_back((next_state, presses + 1));
                    seen.insert(next_state);
                }
            }
        }
    }

    println!("{}", res);
}

fn part2() {
}

fn main() {
    part1();
    part2();
}
