use std::fs;

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    values: Vec<usize>,
}

#[derive(Debug)]
struct Input {
    grids: Vec<Grid>,
}

fn parse_input() -> Input {
    let lines: Vec<String> = fs::read_to_string("inputs/day12.txt")
        .expect("Should have been able to read the file")
        .lines()
        .map(|x| x.to_string())
        .collect();

    let mut grids = vec![];
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].trim();

        if line.is_empty() {
            i += 1;
            continue;
        }

        if line.contains(':') {
            let parts: Vec<&str> = line.split(':').collect();
            let label = parts[0].trim();

            if label.contains('x') {
                let dims: Vec<usize> = label.split('x').map(|s| s.parse().unwrap()).collect();
                let values: Vec<usize> = parts[1]
                    .trim()
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect();

                grids.push(Grid {
                    width: dims[0],
                    height: dims[1],
                    values,
                });
            }
        }
        i += 1;
    }

    Input { grids }
}

fn part1(input: Input) {
    println!("Day 12, Part 1");

    let mut defo_true = 0;
    for grid in input.grids {
        let mut present_area = 0;
        for value in grid.values {
            present_area += value * 8;
        }
        if present_area < grid.height * grid.width {
            defo_true += 1;
        }
    }

    println!("{} regions can fit all of the presents", defo_true);
}

fn part2() {
    println!("🎄");
}

pub fn run(part: Option<u8>) {
    let input = parse_input();
    match part {
        Some(1) => part1(input),
        Some(2) => part2(),
        None => {
            part1(input);
            part2();
        }
        _ => {
            eprintln!("invalid part");
            std::process::exit(2);
        }
    }
}
