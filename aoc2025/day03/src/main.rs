use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn get_lines() -> Vec<String> {
    let mut args = std::env::args();
    let input_src = args.nth(1).unwrap();

    let file = File::open(input_src).unwrap();
    let reader = BufReader::new(file);

    reader.lines().map(|v| v.unwrap()).collect::<Vec<_>>()
}

fn part1() {
    let mut total = 0;
    let lines = get_lines();

    for line in lines {
        if line.is_empty() {
            continue;
        }

        let mut max_so_far = line[0..2].parse::<u64>().unwrap();
        let mut highest = line[0..2]
            .chars()
            .map(|v| v.to_string().parse::<u64>().unwrap())
            .max()
            .unwrap();

        for c in line.chars().skip(2) {
            let jolt = c.to_string().parse::<u64>().unwrap();
            let power = highest * 10 + jolt;
            if power > max_so_far {
                max_so_far = power;
            }

            if jolt > highest {
                highest = jolt;
            }
        }

        total += max_so_far;
    }

    println!("{}", total);
}

fn get_highest(jolts: &Vec<u64>, p: usize, q: usize) -> (u64, usize) {
    let mut highest = 0;
    let mut index = 0;

    for i in p..q {
        let v = jolts.iter().nth(i).unwrap();
        if v > &highest {
            highest = v.clone();
            index = i;
        }
    }

    (highest, index)
}

fn part2() {
    let mut total = 0;
    let lines = get_lines();

    for line in lines {
        if line.is_empty() {
            continue;
        }

        let mut left: u32 = 12;
        let mut last = 0;

        let mut result = 0;

        let jolts = line
            .chars()
            .map(|v| v.to_string().parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        while left > 0 {
            let (highest, index) = get_highest(&jolts, last, jolts.len() - (left as usize) + 1);
            last = index + 1;
            result += highest * (10 as u64).pow(left - 1);

            left -= 1;
        }

        total += result;
    }

    println!("{}", total);
}

fn main() {
    part1();
    part2();
}
