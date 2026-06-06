use std::fs;

fn parse_input() -> (Vec<Vec<i64>>, Vec<String>) {
    let content: Vec<String> = fs::read_to_string("inputs/day06.txt")
        .expect("Should have been able to read the file")
        .trim()
        .lines()
        .map(|x| x.to_string())
        .collect();

    let length = content.len();

    let initial_operands: Vec<Vec<i64>> = content[0..length - 1]
        .to_vec()
        .iter()
        .map(|x| {
            x.trim()
                .split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect()
        })
        .collect();

    let rows = initial_operands.len();
    let cols = initial_operands[0].len();

    let mut operands = vec![];

    for i in 0..cols {
        let mut group = vec![];
        for j in 0..rows {
            group.push(*initial_operands[j].get(i).unwrap());
        }
        operands.push(group);
    }

    let operations: Vec<String> = content
        .get(length - 1)
        .unwrap()
        .split_whitespace()
        .map(|x| x.to_string())
        .collect();

    (operands, operations)
}

fn parse_zeroed_input() -> (Vec<Vec<i64>>, Vec<String>) {
    let content: Vec<String> = fs::read_to_string("inputs/day06.txt")
        .expect("Should have been able to read the file")
        .trim()
        .lines()
        .map(|x| x.to_string())
        .collect();

    let length = content.len();
    let mut numbers = content[..length - 1].to_vec();

    numbers.reverse();

    let cols = content[0].len();
    let mut new_numbers = vec![];

    for c in 0..cols {
        let mut s = "".to_owned();
        for i in 0..length - 1 {
            let to_push = numbers[i].chars().nth(c).unwrap().to_string();
            if !to_push.trim().is_empty() {
                s = numbers[i].chars().nth(c).unwrap().to_string() + s.as_str();
            }
        }

        new_numbers.push(s.parse::<i64>().unwrap_or(0));
    }

    let out = new_numbers
        .split(|x| *x == 0)
        .map(|slice| slice.to_vec())
        .collect::<Vec<Vec<i64>>>();

    let operations: Vec<String> = content
        .get(length - 1)
        .unwrap()
        .split_whitespace()
        .map(|x| x.to_string())
        .collect();

    (out, operations)
}

fn do_homework(operands: Vec<Vec<i64>>, operations: Vec<String>) -> i64 {
    let operations_lenght = operations.len();

    let mut total = 0;

    for i in 0..operations_lenght {
        let operation = operations.get(i).unwrap();

        match operation.as_str() {
            "+" => {
                let sum: i64 = operands.get(i).unwrap().iter().sum();
                total += sum;
            }
            "*" => {
                let product = operands
                    .get(i)
                    .unwrap()
                    .iter()
                    .copied()
                    .reduce(|x, y| x * y)
                    .unwrap();
                total += product;
            }
            _ => panic!("Operation expected, found none."),
        }
    }

    total
}

fn part1() {
    println!("Day 6, Part 1");

    let (human_operands, operations) = parse_input();

    let total = do_homework(human_operands, operations);

    println!("Human grand total: {}", total);
}

fn part2() {
    println!("Day 6, Part 2");

    let (cephalopod_operands, operations) = parse_zeroed_input();

    let total = do_homework(cephalopod_operands, operations);

    println!("Cephalopod grand total: {}", total);
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
