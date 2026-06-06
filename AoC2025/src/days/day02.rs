use std::fs;

fn parse_text() -> Vec<Vec<usize>> {
    let input =
        fs::read_to_string("inputs/day02.txt").expect("Should have been able to read the file");

    let mut content = vec![];

    let ranges = input
        .trim()
        .split(',')
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    for range in ranges {
        let element = range
            .trim()
            .split('-')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        content.push(element);
    }

    content
}

fn is_double_seq(s: String) -> bool {
    let half = s.len() / 2;
    s.len() % 2 == 0 && s[..half] == s[half..]
}

fn is_invalid_id(s: String) -> bool {
    let length = s.len();

    for size in 1..=length / 2 {
        if length % size != 0 {
            continue;
        }

        let substring = &s[..size];
        if substring.repeat(length / size) == s {
            return true;
        }
    }

    false
}

fn part1() {
    println!("Day 2, Part 1");
    let mut total = 0;
    let con = parse_text();

    for range in con {
        let n1 = range[0];
        let n2 = range[1];

        for x in n1..=n2 {
            if is_double_seq(x.to_string()) {
                total += x;
            }
        }
    }

    println!("The sum is: {}", total);
}

fn part2() {
    println!("Day 2, Part 2");
    let mut total = 0;
    let con = parse_text();

    for range in con {
        let n1 = range[0];
        let n2 = range[1];

        for x in n1..=n2 {
            if is_invalid_id(x.to_string()) {
                total += x;
            }
        }
    }

    println!("The sum is: {}", total);
}

pub fn run(part: Option<u8>) {
    match part {
        Some(1) => part1(),
        Some(2) => part2(),
        None => {
            part1();
            part2();
        }
        _ => {
            eprintln!("invalid part");
            std::process::exit(2);
        }
    }
}
