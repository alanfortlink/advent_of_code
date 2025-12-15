use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

fn get_lines() -> (String, Vec<String>) {
    let mut args = std::env::args();

    let input_src = if args.len() >= 2 {
        args.nth(1).unwrap()
    } else {
        String::from("example.txt")
    };

    let file = File::open(input_src.clone()).unwrap();
    let reader = BufReader::new(file);

    (
        input_src,
        reader
            .lines()
            .map(|v| v.unwrap())
            //.filter(|v| !v.is_empty())
            .collect::<Vec<_>>(),
    )
}

fn count_paths(
    cache: &mut HashMap<String, usize>,
    graph: &HashMap<String, Vec<String>>,
    curr: &String,
    target: &String,
) -> usize {
    if curr == target {
        return 1;
    }

    if let Some(&count) = cache.get(curr) {
        return count;
    }

    let mut paths = 0;

    let neighbors = match graph.get(curr) {
        Some(n) => n,
        None => return 0,
    };

    for neigh in neighbors {
        paths += count_paths(cache, graph, neigh, target);
    }

    cache.insert(curr.clone(), paths);

    paths
}

fn get_paths(
    graph: &HashMap<String, Vec<String>>,
    origin: &String,
    target: &String,
) -> HashSet<String> {
    let mut queue = VecDeque::<String>::new();
    let mut seen = HashSet::<String>::new();
    let mut paths = HashSet::<String>::new();

    queue.push_back(origin.clone());

    while !queue.is_empty() {
        let path = queue.pop_front().unwrap();

        let curr = path.chars().skip(path.len() - 3).collect::<String>();

        if &curr == target {
            paths.insert(path);
            continue;
        }

        if !graph.contains_key(&curr) {
            continue;
        }

        for neigh in &graph[&curr] {
            if path.contains(neigh) {
                continue;
            }

            let new_path = path.to_string() + neigh;

            if !seen.contains(&new_path) {
                seen.insert(new_path.clone());
                queue.push_back(new_path);
            }
        }
    }

    paths
}

fn part1() {
    let (filename, lines) = get_lines();

    if filename == "example2.txt" {
        println!("NA");
        return;
    }

    let graph = lines
        .iter()
        .map(|line| {
            let (source, neighs) = line.split_once(": ").unwrap();
            (
                source.to_string(),
                neighs
                    .split_whitespace()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<HashMap<_, _>>();

    let res = get_paths(&graph, &"you".to_string(), &"out".to_string()).len();

    println!("{}", res);
}

fn part2() {
    let (filename, lines) = get_lines();

    if filename == "example.txt" {
        println!("NA");
        return;
    }

    let graph = lines
        .iter()
        .map(|line| {
            let (source, neighs) = line.split_once(": ").unwrap();
            (
                source.to_string(),
                neighs
                    .split_whitespace()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<HashMap<_, _>>();

    let paths_dac_fft = count_paths(
        &mut HashMap::new(),
        &graph,
        &"dac".to_string(),
        &"fft".to_string(),
    );
    let paths_fft_dac = count_paths(
        &mut HashMap::new(),
        &graph,
        &"fft".to_string(),
        &"dac".to_string(),
    );

    let paths_svr_dac = count_paths(
        &mut HashMap::new(),
        &graph,
        &"svr".to_string(),
        &"dac".to_string(),
    );
    let paths_svr_fft = count_paths(
        &mut HashMap::new(),
        &graph,
        &"svr".to_string(),
        &"fft".to_string(),
    );

    let paths_fft_out = count_paths(
        &mut HashMap::new(),
        &graph,
        &"fft".to_string(),
        &"out".to_string(),
    );
    let paths_dac_out = count_paths(
        &mut HashMap::new(),
        &graph,
        &"dac".to_string(),
        &"out".to_string(),
    );

    let res = paths_svr_dac * paths_dac_fft * paths_fft_out
        + paths_svr_fft * paths_fft_dac * paths_dac_out;

    println!("{}", res);
}

fn main() {
    part1();
    part2();
}
