use std::{
    collections::{HashMap, HashSet, VecDeque},
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
    let matrix = lines
        .into_iter()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();

    let mut splits: HashSet<(usize, usize)> = HashSet::new();
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut origins: VecDeque<(usize, usize)> = VecDeque::new();

    origins.push_back((
        0,
        matrix[0]
            .iter()
            .enumerate()
            .filter_map(|v| {
                if *v.1 == 'S' {
                    return Some(v.0);
                }
                return None;
            })
            .nth(0)
            .unwrap(),
    ));

    let mut res = 0;

    while !origins.is_empty() {
        let (r, c) = origins.pop_front().unwrap();
        if seen.contains(&(r, c)) {
            continue;
        }

        seen.insert((r, c));

        for pr in r..matrix.len() {
            if matrix[pr][c] == '^' {
                splits.insert((pr, c));
                origins.push_back((pr, c - 1));
                origins.push_back((pr, c + 1));
                break;
            }
        }
    }

    res = splits.len();

    println!("{}", res);
}

fn part2() {
    let lines = get_lines();
    let matrix = lines
        .into_iter()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();

    let mut C: HashMap<(usize, usize), u64> = HashMap::new();

    fn num_paths(C: &mut HashMap<(usize, usize), u64>, matrix: &Vec<Vec<char>>, r: usize, c: usize) -> u64 {
        if C.contains_key(&(r, c)) {
            return C[&(r, c)];
        }

        if r == 0 {
            if matrix[r][c] == 'S' {
                return 1;
            }

            return 0;
        }

        let mut count = 0;
        if matrix[r - 1][c] != '^' {
            count += num_paths(C, &matrix, r - 1, c);
        }

        if c < (matrix[r].len() - 1) && matrix[r][c + 1] == '^' {
            count += num_paths(C, &matrix, r, c + 1);
        }

        if c > 0 && matrix[r][c - 1] == '^' {
            count += num_paths(C, &matrix, r, c - 1);
        }

        C.insert((r, c), count);

        return count;
    }

    let mut res = 0;

    let last_row = matrix.len() - 1;
    res = (0..matrix[last_row].len())
        .map(|c| num_paths(&mut C, &matrix, last_row, c))
        .sum();

    println!("{}", res);
}

fn main() {
    part1();
    part2();
}
