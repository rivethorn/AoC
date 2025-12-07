use std::fs;

fn parse_text() -> Vec<String> {
    fs::read_to_string("inputs/day03.txt")
        .expect("Should have been able to read the file")
        .trim()
        .lines()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
}

fn find_joltage(joltages: Vec<String>, units: usize) -> i64 {
    let mut total: i64 = 0;

    for joltage in joltages {
        let numbers: Vec<i64> = joltage
            .chars()
            .map(|x| x.to_string().parse().unwrap_or(0))
            .collect();

        let length = numbers.len();
        let mut n = length - units;
        let mut factor = (10 as i64).pow(units as u32 - 1);
        let mut index = 0;

        while n < length {
            let window = &numbers[index..=n];
            let max = window.iter().max().unwrap_or(&0);

            total += max * factor;

            let pos_in_window = window.iter().position(|r| r == max).unwrap();
            index += pos_in_window + 1;

            factor /= 10;
            n += 1;
        }
    }

    total
}

fn part1() {
    println!("Day 3, Part 1");
    let content = parse_text();
    println!("joltage: {}", find_joltage(content, 2));
}

fn part2() {
    println!("Day 3, Part 2");
    let content = parse_text();
    println!("joltage: {}", find_joltage(content, 12));
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
