use std::{
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

fn part1() {
    let lines = get_lines();
    let mut res = 0;

    let len = lines.len();
    let mut rows: Vec<Vec<i64>> = Vec::new();

    for (idx, line) in lines.into_iter().enumerate() {
        if idx == len - 1 {
            let ops: Vec<&str> = line.split_whitespace().collect();

            let cols = rows[0].len();
            let mut partial = 0;

            for c in 0..cols {
                if ops[c] == "*" {
                    partial = 1;
                } else {
                    partial = 0;
                }

                for r in 0..(len - 1) {
                    if ops[c] == "*" {
                        partial *= rows[r][c];
                    } else {
                        partial += rows[r][c];
                    }
                }

                res += partial;
            }

            break;
        }

        let values = line
            .split_whitespace()
            .map(|v| v.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        rows.push(values);
    }

    println!("{}", res);
}

fn part2() {
    let lines = get_lines();
    let mut res = 0;
    let mut rows: Vec<Vec<char>> = Vec::new();
    let len = lines.len();

    for (l, line) in lines.iter().enumerate() {
        if l == len - 1 {
            let ops: Vec<&str> = line.split_whitespace().collect();

            let mut opi = 0;

            let max_cols = lines.iter().map(|r| r.len()).max().unwrap();

            let mut current = if ops[opi] == "*" { 1 } else { 0 };

            for c in 0..(max_cols+1) {
                let column = rows
                    .iter()
                    .map(|r| r.iter().nth(c).unwrap_or(&' '))
                    .collect::<Vec<_>>();
                if c == max_cols || column.iter().all(|v| **v == ' ') {
                    res += current;

                    if (c == max_cols) {
                        continue;
                    }

                    opi += 1;
                    current = if ops[opi] == "*" { 1 } else { 0 };
                    continue;
                }

                let num = column
                    .iter()
                    .filter(|v| ***v != ' ')
                    .map(|v| v.to_string())
                    .collect::<String>()
                    .parse::<i64>()
                    .unwrap();

                if ops[opi] == "*" {
                    current *= num;
                } else {
                    current += num;
                }
            }
        }

        let values = line.chars().collect::<Vec<_>>();

        rows.push(values);
    }

    println!("{}", res);
}

fn main() {
    part1();
    part2();
}
