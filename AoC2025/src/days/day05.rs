use std::{cmp, fs};

fn parse_text() -> (Vec<(i64, i64)>, Vec<i64>) {
    let binding =
        fs::read_to_string("inputs/day05.txt").expect("Should have been able to read the file");
    let lines: Vec<&str> = binding.trim().lines().collect();

    let mut ranges: Vec<(i64, i64)> = vec![];
    let mut idx = 0;

    for line in &lines {
        if *line == "" {
            break;
        }

        let range_val: Vec<i64> = line.split('-').map(|x| x.parse().unwrap_or(0)).collect();
        let range_tuple = (range_val[0], range_val[1]);
        ranges.push(range_tuple);

        idx += 1;
    }

    let ids = lines[idx + 1..]
        .iter()
        .map(|x| x.parse().unwrap_or(0))
        .collect::<Vec<i64>>();

    (ranges, ids)
}

fn reuce_ranges(ranges: Vec<(i64, i64)>, length: usize) -> Vec<(i64, i64)> {
    let mut new_ranges: Vec<(i64, i64)> = vec![];

    let mut i = 1;
    while i < length {
        let current = ranges.get(i).unwrap();
        let prev = ranges.get(i - 1).unwrap();

        if prev.1 >= current.0 {
            if current.0 == current.1 {
                new_ranges.push((prev.0, prev.1));
            } else {
                new_ranges.push((prev.0, cmp::max(current.1, prev.1)));
            }
            new_ranges.extend(&ranges[i + 1..]);

            break;
        } else {
            new_ranges.push(*prev);
            if i == length - 1 {
                new_ranges.push(*current);
            }
        }
        i += 1;
    }

    let new_length = new_ranges.len();

    if length == new_length || new_ranges.len() == 1 {
        new_ranges
    } else {
        reuce_ranges(new_ranges, new_length)
    }
}

fn construc_range(mut ranges: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    ranges.sort_by(|x, y| x.0.partial_cmp(&y.0).unwrap());
    ranges
}

fn part1(ranges: Vec<(i64, i64)>, ingredients: Vec<i64>) {
    println!("Day 5, Part 1");

    let mut total = 0;

    for ing in ingredients {
        let mut included = false;
        for range in &ranges {
            if range.0 <= ing && range.1 >= ing {
                included = true;
                break;
            }
        }

        if included {
            total += 1;
        }
    }

    println!("{} ingredients are fresh", total);
}

fn part2(ranges: Vec<(i64, i64)>) {
    println!("Day 5, Part 2");

    let length = ranges.len();

    let sorted_ranges = construc_range(ranges);
    let merged_ranges = reuce_ranges(sorted_ranges, length);

    let mut total = 0;

    for range in merged_ranges {
        total += range.1 - range.0 + 1;
    }

    println!("{} ingredient IDs are available", total);
}

pub fn run(part: Option<u8>) {
    let input = parse_text();
    match part {
        Some(1) => part1(input.0, input.1),
        Some(2) => part2(input.0),
        None => {
            part1(input.0.clone(), input.1);
            part2(input.0);
        }
        _ => {
            eprintln!("invalid part");
            std::process::exit(2);
        }
    }
}
