use std::{
    collections::HashMap,
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

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
    group: usize,
}

impl Point {
    fn new(x: i64, y: i64, z: i64, group: usize) -> Self {
        Point { x, y, z, group }
    }
}

fn dist(p1: &Point, p2: &Point) -> i64 {
    let a = (p1.x - p2.x).pow(2);
    let b = (p1.y - p2.y).pow(2);
    let c = (p1.z - p2.z).pow(2);

    let s = a + b + c;
    return s.isqrt();
}

fn part1() {
    let lines = get_lines();
    let mut points = lines
        .iter()
        .enumerate()
        .map(|(i, l)| {
            let parts = l
                .split(",")
                .map(|v| v.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            return Point::new(parts[0], parts[1], parts[2], i);
        })
        .collect::<Vec<_>>();

    let mut distances: Vec<(i64, usize, usize)> = Vec::new();

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let d = dist(&points[i], &points[j]);
            distances.push((d, i, j));
        }
    }

    distances.sort();

    let mut count = if points.len() < 50 { 10 } else { 1000 };
    for (_, i1, i2) in distances {
        if count == 0 {
            break;
        }

        let p1 = points[i1].clone();
        let p2 = points[i2].clone();

        count -= 1;

        if p1.group == p2.group {
            continue;
        }

        for i in 0..points.len() {
            let p = points[i].clone();
            if p.group == p1.group {
                points[i] = Point::new(p.x, p.y, p.z, p2.group);
            }
        }
    }

    let mut groups: HashMap<usize, usize> = HashMap::new();

    for p in points {
        if !groups.contains_key(&p.group) {
            groups.insert(p.group, 0);
        }

        groups.insert(p.group, groups[&p.group] + 1);
    }

    let mut sizes = groups.iter().map(|g| g.1).collect::<Vec<_>>();
    sizes.sort();
    sizes.reverse();

    let mut res = sizes[0] * sizes[1] * sizes[2];

    println!("{}", res);
}

fn part2() {
    let lines = get_lines();
    let mut points = lines
        .iter()
        .enumerate()
        .map(|(i, l)| {
            let parts = l
                .split(",")
                .map(|v| v.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            return Point::new(parts[0], parts[1], parts[2], i);
        })
        .collect::<Vec<_>>();

    let mut distances: Vec<(i64, usize, usize)> = Vec::new();

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let d = dist(&points[i], &points[j]);
            distances.push((d, i, j));
        }
    }

    distances.sort();

    for (_, i1, i2) in distances {
        let p1 = points[i1].clone();
        let p2 = points[i2].clone();

        if p1.group == p2.group {
            continue;
        }

        for i in 0..points.len() {
            let p = points[i].clone();
            if p.group == p1.group {
                points[i] = Point::new(p.x, p.y, p.z, p2.group);
            }
        }

        let n = points.iter().filter(|p| p.group == p2.group).count();
        if n == points.len() {
            let res = p1.x * p2.x;
            println!("{}", res);
            break;
        }
    }
}

fn main() {
    part1();
    part2();
}
