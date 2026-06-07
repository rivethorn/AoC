use std::{collections::VecDeque, fs};

use rayon::iter::{ParallelBridge, ParallelIterator};
use rustc_hash::FxHashSet;

const EPSILON: f64 = 1e-9;

fn parse_input() -> Vec<Machine> {
    fs::read_to_string("inputs/day10.txt")
        .expect("Should have been able to read the file")
        .lines()
        .map(Machine::from)
        .collect()
}

struct Machine {
    lights: usize,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<usize>,
}

impl From<&str> for Machine {
    fn from(value: &str) -> Self {
        let mut parts = value.split_whitespace();
        let lights = parts
            .next()
            .map(|l| {
                l.trim_matches(['[', ']'])
                    .chars()
                    .rev()
                    .fold(0, |acc, c| (acc << 1) | if c == '#' { 1 } else { 0 })
            })
            .unwrap();

        let mut parts: Vec<&str> = parts.collect();
        let joltages = parts
            .pop()
            .unwrap()
            .trim_matches(['{', '}'])
            .split(',')
            .map(|v| v.parse().unwrap())
            .collect();

        let mut buttons: Vec<Vec<usize>> = parts
            .iter()
            .map(|b| {
                b.trim_matches(['(', ')'])
                    .split(',')
                    .map(|v| v.parse().unwrap())
                    .collect()
            })
            .collect();

        buttons.sort_by_key(|b| std::cmp::Reverse(b.len()));

        Self {
            lights,
            buttons,
            joltages,
        }
    }
}

fn part1(machines: &[Machine]) {
    println!("Day 10, Part 1");

    let total: u64 = machines
        .iter()
        .map(|machine| {
            let mut frontier = VecDeque::new();
            frontier.push_back((0, 0));

            let mut seen = FxHashSet::default();
            seen.insert(0);

            while let Some((lights, dist)) = frontier.pop_front() {
                if lights == machine.lights {
                    return dist;
                }

                for neighbor in machine.buttons.iter() {
                    let neighbor = neighbor.iter().fold(lights, |acc, n| acc ^ (1 << n));
                    if seen.insert(neighbor) {
                        frontier.push_back((neighbor, dist + 1));
                    }
                }
            }
            unreachable!()
        })
        .sum();

    println!("{} button presses required.", total);
}

struct Matrix {
    data: Vec<Vec<f64>>,
    rows: usize,
    cols: usize,
    dependents: Vec<usize>,
    independents: Vec<usize>,
}

impl Matrix {
    fn from_machine(machine: &Machine) -> Self {
        let rows = machine.joltages.len();
        let cols = machine.buttons.len();
        let mut data = vec![vec![0.0; cols + 1]; rows];

        for (c, button) in machine.buttons.iter().enumerate() {
            for &r in button {
                if r < rows {
                    data[r][c] = 1.0;
                }
            }
        }

        for (r, &val) in machine.joltages.iter().enumerate() {
            data[r][cols] = val as f64;
        }

        let mut matrix = Self {
            data,
            rows,
            cols,
            dependents: Vec::new(),
            independents: Vec::new(),
        };

        matrix.gaussian_elimination();
        matrix
    }

    fn gaussian_elimination(&mut self) {
        let mut pivot = 0;

        let mut col = 0;
        while pivot < self.rows && col < self.cols {
            let (best_row, best_value) = self
                .data
                .iter()
                .enumerate()
                .skip(pivot)
                .map(|(r, row)| (r, row[col].abs()))
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .unwrap();

            if best_value < EPSILON {
                self.independents.push(col);
                col += 1;
                continue;
            }

            self.data.swap(pivot, best_row);
            self.dependents.push(col);

            let pivot_value = self.data[pivot][col];
            for val in &mut self.data[pivot][col..=self.cols] {
                *val /= pivot_value;
            }

            for r in 0..self.rows {
                if r != pivot {
                    let factor = self.data[r][col];
                    if factor.abs() > EPSILON {
                        let pivot_row = self.data[pivot][col..=self.cols].to_vec();
                        self.data[r][col..=self.cols]
                            .iter_mut()
                            .zip(&pivot_row)
                            .for_each(|(val, &pivot_val)| {
                                *val -= factor * pivot_val;
                            });
                    }
                }
            }

            pivot += 1;
            col += 1;
        }

        self.independents.extend(col..self.cols);
    }

    fn valid(&self, values: &[usize]) -> Option<usize> {
        let mut total = values.iter().sum::<usize>();

        for row in 0..self.dependents.len() {
            let val = self
                .independents
                .iter()
                .enumerate()
                .fold(self.data[row][self.cols], |acc, (i, &col)| {
                    acc - self.data[row][col] * (values[i] as f64)
                });

            if val < -EPSILON {
                return None;
            }
            let rounded = val.round();
            if (val - rounded).abs() > EPSILON {
                return None;
            }

            total += rounded as usize;
        }

        Some(total)
    }
}

fn dfs(matrix: &Matrix, idx: usize, values: &mut [usize], min: &mut usize, max: usize) {
    if idx == matrix.independents.len() {
        if let Some(total) = matrix.valid(values) {
            *min = (*min).min(total);
        }
        return;
    }

    let total: usize = values[..idx].iter().sum();
    for val in 0..max {
        if total + val >= *min {
            break;
        }
        values[idx] = val;
        dfs(matrix, idx + 1, values, min, max);
    }
}

fn part2(machines: &[Machine]) {
    println!("Day 10, Part 2");

    let total: usize = machines
        .iter()
        .par_bridge()
        .map(|machine| {
            let matrix = Matrix::from_machine(machine);

            let max = *machine.joltages.iter().max().unwrap() + 1;
            let mut min = usize::MAX;
            let mut values = vec![0; matrix.independents.len()];

            dfs(&matrix, 0, &mut values, &mut min, max);

            min
        })
        .sum();

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
