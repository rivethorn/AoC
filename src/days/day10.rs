use std::{
    collections::{HashSet, VecDeque},
    fs,
};

use z3::{Optimize, SatResult, ast::Int};

struct Machine {
    target: u64,
    buttons: Vec<u64>,
    joltages: Vec<i64>,
}

impl From<&str> for Machine {
    fn from(value: &str) -> Self {
        let info: Vec<&str> = value.split(" ").collect();

        Self {
            target: parse_target(info[0]),
            buttons: parse_buttons(&info[1..info.len() - 1]),
            joltages: parse_joltages(info[info.len() - 1]),
        }
    }
}

fn parse_target(target: &str) -> u64 {
    let chars = target.strip_prefix("[").unwrap().strip_suffix("]").unwrap();
    chars.chars().rev().fold(0, |acc, c| match c {
        '#' => (acc << 1) | 1,
        '.' => acc << 1,
        _ => panic!(),
    })
}

fn parse_joltages(jolt: &str) -> Vec<i64> {
    let n = jolt.strip_prefix("{").unwrap().strip_suffix("}").unwrap();
    n.split(",").map(|num| num.parse().unwrap()).collect()
}

fn parse_buttons(buttons: &[&str]) -> Vec<u64> {
    buttons.iter().map(|b| parse_button(b)).collect()
}

fn parse_button(btn: &str) -> u64 {
    let n = btn.strip_prefix("(").unwrap().strip_suffix(")").unwrap();

    n.split(",")
        .map(|num| num.parse::<u64>().unwrap())
        .fold(0, |acc, num| acc | (1 << num))
}

fn parse_input() -> Vec<Machine> {
    fs::read_to_string("inputs/day10.txt")
        .expect("Should have been able to read the file")
        .lines()
        .map(Machine::from)
        .collect()
}

fn fewest_presses(machine: &Machine) -> u64 {
    let seen: HashSet<u64> = HashSet::new();

    let mut queue: VecDeque<(u64, u64)> = VecDeque::new();

    machine.buttons.iter().for_each(|btn| {
        queue.push_back((1, *btn));
    });

    while !queue.is_empty() {
        let (count, val) = queue.pop_front().unwrap();

        if val == machine.target {
            return count;
        }
        machine.buttons.iter().for_each(|btn| {
            let new = val ^ btn;
            if !seen.contains(&new) {
                queue.push_back((count + 1, new));
            }
        });
    }
    panic!("No solution found");
}

fn positions(button: u64) -> Vec<usize> {
    let mut b = button;
    let mut res = vec![];
    let mut pos = 0;

    while b != 0 {
        if (b & 1) != 0 {
            res.push(pos);
        }
        b >>= 1;
        pos += 1;
    }

    res
}

fn fewest_joltages_min(machine: &Machine) -> u64 {
    let joltages = &machine.joltages;
    let buttons: Vec<Vec<usize>> = machine.buttons.iter().map(|b| positions(*b)).collect();

    let opt = Optimize::new();
    let total_presses = Int::fresh_const("total_presses");

    let button_presses: Vec<Int> = (0..buttons.len())
        .map(|i| Int::fresh_const(&format!("button_{i}")))
        .collect();

    button_presses.iter().for_each(|b| opt.assert(&b.ge(0)));
    for (pos, &target) in joltages.iter().enumerate() {
        let mut terms = Vec::new();

        for (i, btn) in buttons.iter().enumerate() {
            if btn.contains(&pos) {
                terms.push(button_presses[i].clone());
            }
        }
        let sum = Int::add(&terms.iter().collect::<Vec<&Int>>());
        opt.assert(&sum.eq(Int::from_u64(target as u64)));
    }

    opt.assert(&total_presses.eq(Int::add(&button_presses)));
    opt.minimize(&total_presses);

    match opt.check(&[]) {
        SatResult::Sat => opt
            .get_model()
            .unwrap()
            .eval(&total_presses, true)
            .and_then(|t| t.as_u64())
            .unwrap(),
        _ => panic!("No solution found"),
    }
}

fn part1(machines: &Vec<Machine>) {
    println!("Day 10, Part 1");

    let total: u64 = machines.iter().map(fewest_presses).sum();

    println!("{} button presses required.", total);
}

fn part2(machines: &Vec<Machine>) {
    println!("Day 10, Part 2");

    let total: u64 = machines.iter().map(fewest_joltages_min).sum();

    println!("{} button presses required.", total);
}

pub fn run(part: Option<u8>) {
    let machines = parse_input();
    match part {
        Some(1) => part1(&machines),
        Some(2) => part2(&machines),
        None => {
            part1(&machines);
            part2(&machines);
        }
        _ => {
            eprintln!("invalid part");
            std::process::exit(2);
        }
    }
}
