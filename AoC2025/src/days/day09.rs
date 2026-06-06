use std::fs;

#[derive(Clone)]
struct Coord(u64, u64);

impl From<&str> for Coord {
    fn from(value: &str) -> Self {
        let nums: Vec<u64> = value.split(",").map(|n| n.parse().unwrap()).collect();
        match nums.as_slice() {
            [x, y] => Self(*x, *y),
            _ => panic!("Expected 2-element vector"),
        }
    }
}

impl Coord {
    fn area(&self, other: &Self) -> u64 {
        (self.0.abs_diff(other.0) + 1) * (self.1.abs_diff(other.1) + 1)
    }
}

fn parse_input() -> Vec<Coord> {
    fs::read_to_string("inputs/day09.txt")
        .expect("Should have been able to read file")
        .lines()
        .map(Coord::from)
        .collect()
}

fn rectangles(coords: &[Coord]) -> Vec<(Coord, Coord)> {
    let mut pairs = vec![];
    for i in 0..(coords.len() - 1) {
        for j in i + 1..coords.len() {
            pairs.push((coords[i].clone(), coords[j].clone()))
        }
    }

    pairs.sort_by(|p1, p2| p1.0.area(&p1.1).cmp(&p2.0.area(&p2.1)));
    pairs.reverse();
    pairs
}

fn edges(point1: &Coord, point2: &Coord) -> (u64, u64, u64, u64) {
    (
        point1.0.min(point2.0),
        point1.0.max(point2.0),
        point1.1.min(point2.1),
        point1.1.max(point2.1),
    )
}

fn part1(coords: &Vec<Coord>) {
    println!("Day 9, Part 1");

    let mut max_val = u64::MIN;

    for i in 0..(coords.len() - 1) {
        for j in i + 1..coords.len() {
            max_val = max_val.max(coords[i].area(&coords[j]));
        }
    }

    println!("The largest area of the rectangle is {}", max_val);
}

fn part2(coords: &Vec<Coord>) {
    println!("Day 9, Part 2");

    let length = coords.len();
    let rectangles = rectangles(&coords);

    let (p1, p2) = rectangles
        .iter()
        .find(|(p1, p2)| {
            let (xmin, xmax, ymin, ymax) = edges(p1, p2);
            for (i, c1) in coords.iter().enumerate() {
                let c2 = &coords[(i + 1) % length];

                if c1.0 == c2.0 {
                    let (ylmin, ylmax) = (c1.1.min(c2.1), c1.1.max(c2.1));
                    if xmin < c1.0 && xmax > c1.0 && !(ymin >= ylmax || ymax <= ylmin) {
                        return false;
                    }
                } else if c1.1 == c2.1 {
                    let (xlmin, xlmax) = (c1.0.min(c2.0), c1.0.max(c2.0));
                    if ymin < c1.1 && ymax > c1.1 && !(xmin >= xlmax || xmax <= xlmin) {
                        return false;
                    }
                } else {
                    panic!("Unreachable");
                }
            }
            true
        })
        .unwrap();

    println!(
        "The largest area of any rectangle using only red and green is {}",
        p1.area(p2)
    );
}

pub fn run(part: Option<u8>) {
    let coords = parse_input();
    match part {
        Some(1) => part1(&coords),
        Some(2) => part2(&coords),
        None => {
            part1(&coords);
            part2(&coords);
        }
        _ => {
            eprintln!("invalid part");
            std::process::exit(2);
        }
    }
}
