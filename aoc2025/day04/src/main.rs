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
        .filter(|v| !v.is_empty())
        .collect::<Vec<_>>()
}

fn part1() {
    let lines = get_lines();
    let matrix = lines
        .into_iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    let rows = matrix.len() as i32;
    let cols = matrix[0].len() as i32;

    let mut res = 0;

    let dirs: Vec<(i32, i32)> = vec![
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ];

    for i in 0..rows {
        for j in 0..cols {
            let mut count = 0;
            if matrix[i as usize][j as usize] != '@' {
                continue;
            }
            for (di, dj) in &dirs {
                let (ni, nj) = (i + di, j + dj);
                if ni >= 0 && ni < rows && nj >= 0 && nj < cols {
                    if matrix[ni as usize][nj as usize] == '@' {
                        count += 1;
                    }
                }
            }
            if count <= 3 {
                res += 1;
            }
        }
    }

    println!("{}", res);
}

fn part2() {
    let lines = get_lines();
    let mut matrix = lines
        .into_iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    let rows = matrix.len() as i32;
    let cols = matrix[0].len() as i32;

    let mut res = 0;

    let dirs: Vec<(i32, i32)> = vec![
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ];

    let mut active = true;

    while active {
        active = false;
        for i in 0..rows {
            for j in 0..cols {
                let mut count = 0;
                if matrix[i as usize][j as usize] != '@' {
                    continue;
                }
                for (di, dj) in &dirs {
                    let (ni, nj) = (i + di, j + dj);
                    if ni >= 0 && ni < rows && nj >= 0 && nj < cols {
                        if matrix[ni as usize][nj as usize] == '@' {
                            count += 1;
                        }
                    }
                }
                if count <= 3 {
                    res += 1;
                    active = true;
                    matrix[i as usize][j as usize] = '.';
                }
            }
        }
    }

    println!("{}", res);
}

fn main() {
    part1();
    part2();
}
