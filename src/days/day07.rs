use std::{collections::VecDeque, fs};

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Empty,
    Splitter,
    Beam,
}

#[derive(Clone, Copy, PartialEq)]
enum QuantumTile {
    Empty,
    Splitter,
    Beam(u64),
}

fn parse_input() -> String {
    fs::read_to_string("inputs/day07.txt").expect("Should have been able to read the file")
}

fn part1(input: &str) {
    println!("Day 7, Part 1");

    let width = input.lines().next().unwrap().len();
    let mut beams = vec![];

    let mut grid: Vec<_> = input
        .chars()
        .filter(|&c| c != '\n')
        .enumerate()
        .map(|(n, c)| match c {
            '.' => Tile::Empty,
            '^' => Tile::Splitter,
            'S' => {
                beams.push(n);
                Tile::Beam
            }
            _ => panic!("Unexpected character at position {}", n),
        })
        .collect();

    let mut count = 0;

    while let Some(idx) = beams.pop() {
        let new_idx = idx + width;
        if new_idx >= grid.len() {
            continue;
        }

        match grid[new_idx] {
            Tile::Empty => {
                grid[new_idx] = Tile::Beam;
                beams.push(new_idx);
            }
            Tile::Splitter => {
                count += 1;
                for new_idx in [new_idx - 1, new_idx + 1] {
                    if grid[new_idx] == Tile::Empty {
                        grid[new_idx] = Tile::Beam;
                        beams.push(new_idx);
                    }
                }
            }
            Tile::Beam => {}
        }
    }

    println!("The beam will split {} times", count);
}

fn part2(input: &str) {
    println!("Day 7, Part 2");

    let width = input.lines().next().unwrap().len();
    let mut beams = VecDeque::new();

    let mut grid: Vec<_> = input
        .chars()
        .filter(|&c| c != '\n')
        .enumerate()
        .map(|(n, c)| match c {
            '.' => QuantumTile::Empty,
            '^' => QuantumTile::Splitter,
            'S' => {
                beams.push_back(n);
                QuantumTile::Beam(1)
            }
            _ => panic!("Unexpected character ar position {}", n),
        })
        .collect();

    let mut total = 0;

    while let Some(idx) = beams.pop_front() {
        let count = if let QuantumTile::Beam(count) = grid[idx] {
            count
        } else {
            panic!("Expected beam ar position {}", idx);
        };

        let new_idx = idx + width;
        if new_idx >= grid.len() {
            total += count;
            continue;
        }

        match grid[new_idx] {
            QuantumTile::Empty => {
                grid[new_idx] = QuantumTile::Beam(count);
                beams.push_back(new_idx);
            }
            QuantumTile::Splitter => {
                for new_idx in [new_idx - 1, new_idx + 1] {
                    match grid[new_idx] {
                        QuantumTile::Beam(ref mut c) => {
                            *c += count;
                        }
                        QuantumTile::Empty => {
                            grid[new_idx] = QuantumTile::Beam(count);
                            beams.push_back(new_idx);
                        }
                        QuantumTile::Splitter => {
                            panic!("Unexpected splitter at position {}", new_idx)
                        }
                    }
                }
            }
            QuantumTile::Beam(ref mut c) => {
                *c += count;
            }
        }
    }

    println!("A Tachyon particle will end up on {} timelines", total);
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
