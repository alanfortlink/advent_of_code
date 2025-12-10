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

    let coords = lines
        .into_iter()
        .map(|l| {
            l.split(",")
                .map(|v| v.to_string().parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut res = 0;

    for i in 0..coords.len() {
        for j in (i + 1)..coords.len() {
            let dx = (coords[i][0] - coords[j][0]).abs() + 1;
            let dy = (coords[i][1] - coords[j][1]).abs() + 1;

            res = res.max(dx * dy);
        }
    }

    println!("{}", res);
}

fn part2() {
    let mut lines = get_lines();
    lines.push(lines[0].clone());

    let coords = lines
        .into_iter()
        .map(|l| {
            l.split(",")
                .map(|v| v.to_string().parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut edges = Vec::new();
    for i in 0..coords.len() {
        let p1 = &coords[i];
        let p2 = &coords[(i + 1) % coords.len()];

        edges.push((
            (p1[0].min(p2[0]).clone(), p1[1].min(p2[1]).clone()),
            (p2[0].max(p1[0]).clone(), p2[1].max(p1[1]).clone()),
        ));
    }

    fn is_inside(x: i64, y: i64, edges: &Vec<((i64, i64), (i64, i64))>) -> bool {
        let mut inside = false;
        for ((x1, y1), (x2, y2)) in edges.iter() {
            let (x1, y1, x2, y2) = (*x1, *y1, *x2, *y2);
            if x1 <= x && x <= x2 && y1 <= y && y <= y2 {
                return true;
            }

            if x1 == x2 {
                if y1 <= y && y < y2 {
                    if x1 > x {
                        inside = !inside;
                    }
                }
            }
        }

        return inside;
    }

    fn intersects(
        x1: i64,
        y1: i64,
        x2: i64,
        y2: i64,
        edges: &Vec<((i64, i64), (i64, i64))>,
    ) -> bool {
        let (r_min_x, r_max_x) = (x1.min(x2), x1.max(x2));
        let (r_min_y, r_max_y) = (y1.min(y2), y1.max(y2));

        for ((x1, y1), (x2, y2)) in edges.iter() {
            let (x1, y1, x2, y2) = (*x1, *y1, *x2, *y2);
            if x1 == x2 {
                if (r_min_x < x1 && x1 < r_max_x) && !(y2 <= r_min_y || y1 >= r_max_y) {
                    return true;
                }
            } else {
                if (r_min_y < y1 && y1 < r_max_y) && !(x2 <= r_min_x || x1 >= r_max_x) {
                    return true;
                }
            }
        }

        return false;
    }

    let mut res = 0;

    for i in 0..coords.len() {
        for j in (i + 1)..coords.len() {
            let (x1, y1) = (coords[i][0], coords[i][1]);
            let (x2, y2) = (coords[j][0], coords[j][1]);

            let (cx1, cy1) = (x1, y2);
            let (cx2, cy2) = (x2, y1);

            if !is_inside(cx1, cy1, &edges) {
                continue;
            }

            if !is_inside(cx2, cy2, &edges) {
                continue;
            }

            if intersects(x1, y1, x2, y2, &edges) {
                continue;
            }

            let dx = (coords[i][0] - coords[j][0]).abs() + 1;
            let dy = (coords[i][1] - coords[j][1]).abs() + 1;

            res = res.max(dx * dy);
        }
    }

    println!("{}", res);
}

fn main() {
    part1();
    part2();
}
