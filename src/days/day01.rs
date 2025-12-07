use std::fs;

fn parse_text() -> Vec<String> {
    let content =
        fs::read_to_string("inputs/day01.txt").expect("Should have been able to read the file");

    content.trim().split("\n").map(|x| x.to_string()).collect()
}

fn part1() {
    println!("Day 1, Part 1");

    let mut position = 50;
    let mut password = 0;

    let lines = parse_text();

    for line in lines {
        let rotation = line.replace("L", "-").replace("R", "").parse().unwrap_or(0);
        position = (position + rotation) % 100;

        if position == 0 {
            password += 1;
        }
    }

    println!("The password is: {password}");
}

fn part2() {
    println!("Day 1, Part 2");

    let mut position = 50;
    let mut password = 0;

    let lines = parse_text();

    for line in lines {
        let dir = line.chars().next().unwrap_or(' ');
        let amount: i32 = line[1..].parse().unwrap();

        for _ in 0..amount {
            match dir {
                'L' => position -= 1,
                'R' => position += 1,
                _ => std::process::exit(2),
            }
            if position % 100 == 0 {
                password += 1;
            }
        }
    }

    println!("The password is: {}", password);
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
