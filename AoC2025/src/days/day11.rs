use std::{collections::HashMap, fs};

fn parse_input() -> Vec<String> {
    fs::read_to_string("inputs/day11.txt")
        .expect("Should have been able to read the file")
        .lines()
        .map(String::from)
        .collect()
}

fn walk(
    map: &HashMap<String, Vec<String>>,
    cache: &mut HashMap<String, i64>,
    next: String,
    dest: String,
) -> i64 {
    if cache.contains_key(&next) {
        return cache[&next];
    }
    if next == dest {
        return 1;
    }
    if !map.contains_key(&next) {
        return 0;
    }
    let mut sum = 0;
    for key in &map[&next] {
        sum += walk(map, cache, key.to_string(), dest.to_string());
    }
    cache.insert(next, sum);

    sum
}

fn part1(input: &Vec<String>) {
    println!("Day 11, Part 1");

    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for line in input {
        let key_values: Vec<&str> = line.split(": ").collect();
        let key = key_values[0].to_string();
        let values: Vec<String> = key_values[1].split(" ").map(|x| x.to_string()).collect();
        map.insert(key, values);
    }

    let mut cache: HashMap<String, i64> = HashMap::new();
    let count = walk(&map, &mut cache, "you".to_string(), "out".to_string());

    println!("{} different paths lead form you to out", count);
}

fn part2(input: &Vec<String>) {
    println!("Day 11, Part 2");

    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for line in input {
        let key_values: Vec<&str> = line.split(": ").collect();
        let key = key_values[0].to_string();
        let values: Vec<String> = key_values[1].split(" ").map(|x| x.to_string()).collect();
        map.insert(key, values);
    }

    let ans: i64 = [["svr", "fft"], ["fft", "dac"], ["dac", "out"]]
        .iter()
        .fold(1, |mut acc, x| {
            let mut cache: HashMap<String, i64> = HashMap::new();
            let val = walk(&map, &mut cache, x[0].to_string(), x[1].to_string());
            acc *= val;
            acc
        });

    println!("{} paths will visit both", ans);
}

pub fn run(part: Option<u8>) {
    let input = parse_input();
    match part {
        Some(1) => part1(&input),
        Some(2) => part2(&input),
        None => {
            part1(&input);
            part2(&input);
        }
        _ => {
            eprintln!("invalid part");
            std::process::exit(2);
        }
    }
}
