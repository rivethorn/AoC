use std::fs;

use disjoint_set::DisjointSet;

#[derive(Debug, Clone, Copy)]
struct Point(i32, i32, i32);

fn build_dsu(n: usize) -> DisjointSet<usize> {
    let mut dsu = DisjointSet::new();
    for i in 0..n {
        dsu.make_set(i);
    }
    dsu
}

fn build_sorted_edges(points: &[Point]) -> Vec<(i64, usize, usize)> {
    let n = points.len();
    let mut edges = Vec::with_capacity(n.saturating_mul(n) / 2);

    for i in 0..n {
        for j in (i + 1)..n {
            edges.push((distance(points[i], points[j]), i, j));
        }
    }

    edges.sort_by(|a, b| {
        a.0.cmp(&b.0)
            .then_with(|| a.1.cmp(&b.1))
            .then_with(|| a.2.cmp(&b.2))
    });

    edges
}

fn parse_input() -> Vec<Point> {
    fs::read_to_string("inputs/day08.txt")
        .expect("Should have been able to read the file")
        .lines()
        .map(|line| {
            let nums: Vec<i32> = line.split(',').filter_map(|s| s.parse().ok()).collect();
            Point(nums[0], nums[1], nums[2])
        })
        .collect()
}

fn distance(a: Point, b: Point) -> i64 {
    let dx = (a.0 - b.0) as i64;
    let dy = (a.1 - b.1) as i64;
    let dz = (a.2 - b.2) as i64;
    dx * dx + dy * dy + dz * dz
}

fn part1() {
    println!("Day 8, Part 1");
    let points = parse_input();

    let n = points.len();

    let mut dsu = build_dsu(n);
    let edges = build_sorted_edges(&points);

    let limit = edges.len().min(1000);
    for k in 0..limit {
        let (_, i, j) = edges[k];
        let _ = dsu.union(i, j);
    }

    let mut sizes = vec![0usize; n];
    for i in 0..n {
        let root = dsu.find(i).expect("all indices were added");
        sizes[root] += 1;
    }

    sizes.sort_unstable_by(|a, b| b.cmp(a));

    let ans = sizes.iter().take(3).product::<usize>();

    println!("The product of three largest circuits is {}", ans);
}

fn part2() {
    println!("Day 8, Part 2");
    let points = parse_input();
    let n = points.len();

    let mut dsu = build_dsu(n);
    let edges = build_sorted_edges(&points);

    let mut components = n;
    for &(_, i, j) in edges.iter() {
        let ri = dsu.find(i).expect("all indices were added");
        let rj = dsu.find(j).expect("all indices were added");

        if ri != rj {
            let _ = dsu.union(i, j);
            components -= 1;

            if components == 1 {
                let ans = (points[i].0 as i64) * (points[j].0 as i64);
                println!(
                    "Product of X coords of last connected pair ({} and {}) is {}",
                    points[i].0, points[j].0, ans
                );
                break;
            }
        }
    }
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
