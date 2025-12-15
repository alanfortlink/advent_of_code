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
    reader.lines().map(|v| v.unwrap()).collect::<Vec<_>>()
}

fn part1() {
    let mut lines = get_lines().into_iter();

    let mut sizes = Vec::<usize>::new();

    for i in 0..6 {
        lines.next();
        let mut size = 0;

        for _ in 0..3 {
            size += lines.next().unwrap().chars().filter(|c| *c == '#').count();
        }

        sizes.push(size as usize);
        lines.next();
    }

    let mut res = 0;

    for case in lines {
        let (sz, counts) = case.split_once(": ").unwrap();
        let (w, h) = sz.split_once("x").unwrap();
        let (w, h) = (w.parse::<usize>().unwrap(), h.parse::<usize>().unwrap());

        let counts = counts
            .split_whitespace()
            .map(|c| c.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let av_sz = w * h;
        let req_sz: usize = counts.iter().enumerate().map(|(i, c)| c * sizes[i]).sum();

        if (req_sz as f32 * 1.3) < av_sz as f32 {
            res += 1;
        } else {
            println!("{} did NOT", case);
        }
    }

    println!("{}", res);
}

fn part2() {
    let _lines = get_lines();
    let mut res = 0;
    println!("{}", res);
}

fn main() {
    part1();
    part2();
}
