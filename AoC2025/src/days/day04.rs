use std::fs;

const DIRECTIONS: [(i32, i32); 8] = [
    (0, 1),   // bottom
    (0, -1),  // up
    (1, 0),   // right
    (-1, 0),  // left
    (1, 1),   // bottom-right
    (-1, -1), // up-left
    (1, -1),  // up-right
    (-1, 1),  // left-up
];

fn parse_text() -> Vec<Vec<u32>> {
    let lines = fs::read_to_string("inputs/day04.txt")
        .expect("Should have been able to read te file")
        .trim()
        .lines()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    let mut content: Vec<Vec<u32>> = vec![];

    for line in lines {
        let element = line
            .trim()
            .chars()
            .map(|x| if x == '@' { 1 } else { 0 })
            .collect();

        content.push(element);
    }

    content
}

fn find_accessibles(rolls: Vec<Vec<u32>>) -> (i32, Vec<(i32, i32)>) {
    let rows = rolls.len();
    let cols = rolls[0].len();

    let mut total = 0;
    let mut to_be_removed: Vec<(i32, i32)> = vec![];

    for r_idx in 0..rows {
        for c_idx in 0..cols {
            if rolls[r_idx][c_idx] == 1 {
                let mut all = 0;

                for d in DIRECTIONS {
                    let row_idx = r_idx as i32 + d.1 as i32;
                    let col_idx = c_idx as i32 + d.0;

                    if row_idx >= 0
                        && row_idx < rows as i32
                        && col_idx >= 0
                        && col_idx < cols as i32
                    {
                        if rolls[row_idx as usize][col_idx as usize] == 1 {
                            all += 1;
                        }
                    }
                }

                if all < 4 {
                    total += 1;
                    to_be_removed.push((r_idx as i32, c_idx as i32));
                }
            }
        }
    }

    (total, to_be_removed)
}

fn total_removable_rolls(rolls: Vec<Vec<u32>>, rolls_removed: i32) -> i32 {
    let (_, accessibles) = find_accessibles(rolls.clone());

    if accessibles.len() == 0 {
        return rolls_removed;
    } else {
        let new_rolls = remove_rolls(rolls);
        return total_removable_rolls(new_rolls, rolls_removed + accessibles.len() as i32);
    }
}

fn remove_rolls(rolls: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let (_, accessibles) = find_accessibles(rolls.clone());
    let mut new_rolls = rolls.clone();

    for accessible in accessibles {
        new_rolls[accessible.0 as usize][accessible.1 as usize] = 0
    }

    new_rolls
}

fn part1(input: Vec<Vec<u32>>) {
    println!("Day 4, Part 1");
    let (rolls, _) = find_accessibles(input);

    println!("{} rolls can be removed", rolls);
}

fn part2(input: Vec<Vec<u32>>) {
    println!("Day 4, Part 2");
    let total = total_removable_rolls(input, 0);

    println!("{} rolls were removed", total);
}

pub fn run(part: Option<u8>) {
    let input = parse_text();
    match part {
        Some(1) => part1(input),
        Some(2) => part2(input),
        None => {
            part1(input.clone());
            part2(input);
        }
        _ => {
            eprintln!("invalid part");
            std::process::exit(2);
        }
    }
}
