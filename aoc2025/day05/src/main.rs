use std::{
    collections::{VecDeque, btree_map::Range},
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
    let mut lines = get_lines().into_iter();
    let mut res = 0;

    let mut ranges: Vec<(u64, u64)> = Vec::new();

    fn is_fresh_id(id: &u64, ranges: &Vec<(u64, u64)>) -> bool {
        for range in ranges.iter() {
            if id >= &range.0 && id <= &range.1 {
                return true;
            }
        }

        return false;
    }

    loop {
        let line = lines.next();
        if line.is_none() {
            break;
        }

        let line = line.unwrap();

        if line.is_empty() {
            break;
        }

        let (start, end) = line.split_once("-").unwrap();
        let (start, end) = (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap());

        ranges.push((start, end));
    }

    let ids = lines.map(|l| l.parse::<u64>().unwrap());
    let fresh_ids = ids.filter(|id| is_fresh_id(id, &ranges));
    res = fresh_ids.count();

    println!("{}", res);
}

fn part2() {
    let lines = get_lines().into_iter();
    let mut res = 0;

    fn merge_ranges(ranges: &mut Vec<(u64, u64)>) {
        #[derive(PartialEq, Eq, PartialOrd, Ord)]
        enum EventType {
            RangeStart = 0,
            RangeEnd = 1,
        }

        let mut events: Vec<(u64, EventType)> = Vec::new();
        for range in ranges.iter() {
            events.push((range.0, EventType::RangeStart));
            events.push((range.1, EventType::RangeEnd));
        }

        events.sort();

        ranges.clear();

        let mut stack : VecDeque<(u64, EventType)> = VecDeque::new();

        for ev in events {
            if ev.1 == EventType::RangeStart {
                stack.push_back(ev);
            } else {
                let front = stack.pop_back().unwrap();
                if stack.is_empty() {
                    ranges.push((front.0, ev.0));
                }
            }
        }

    }

    let mut ranges: Vec<(u64, u64)> = Vec::new();

    for line in lines {
        if line.is_empty() {
            break;
        }

        let (start, end) = line.split_once("-").unwrap();
        let (start, end) = (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap());

        ranges.push((start, end));
    }

    merge_ranges(&mut ranges);

    for range in ranges {
        res += range.1 - range.0 + 1;
    }

    println!("{}", res);
}

fn main() {
    part1();
    part2();
}
