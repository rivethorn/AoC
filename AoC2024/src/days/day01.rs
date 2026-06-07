use std::fs;

fn parse_text() -> Vec<String> {
    fs::read_to_string("inputs/day01.txt")
        .expect("Should have been able to read the file")
        .trim()
        .split('\n')
        .map(|x| x.to_string())
        .collect()
}

fn part1() {
    println!("Day 1, Part 1");

    let mut lefies: Vec<isize> = vec![];
    let mut righties: Vec<isize> = vec![];
    let mut diffs: Vec<isize> = vec![];

    let lines = parse_text();

    for line in lines {
        let l: Vec<isize> = line
            .split("   ")
            .map(|x| x.parse::<isize>().unwrap())
            .collect();
        lefies.push(l[0]);
        righties.push(l[1]);
    }

    lefies.sort();
    righties.sort();

    lefies.iter().enumerate().for_each(|(li, lv)| {
        righties.iter().enumerate().for_each(|(ri, rv)| {
            if li == ri {
                match lv >= rv {
                    true => diffs.push(lv - rv),
                    false => diffs.push(rv - lv),
                }
            }
        });
    });

    println!("The total distance is: {}", diffs.iter().sum::<isize>());
}

fn part2() {
    println!("Day 1, Part 2");

    let mut lefies: Vec<isize> = vec![];
    let mut righties: Vec<isize> = vec![];
    let mut sims: Vec<isize> = vec![];

    let lines = parse_text();

    for line in lines {
        let l: Vec<isize> = line
            .split("   ")
            .map(|x| x.parse::<isize>().unwrap())
            .collect();
        lefies.push(l[0]);
        righties.push(l[1]);
    }

    lefies.iter().for_each(|le| {
        let mut occ = 0;
        righties.iter().for_each(|re| {
            if le == re {
                occ += 1;
            }
        });
        sims.push(le * occ);
    });

    println!("The similarity score is: {}", sims.iter().sum::<isize>());
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
