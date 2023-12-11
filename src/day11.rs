use std::fs;
use std::collections::HashSet;

pub(crate) fn day11() {
    let file = fs::read_to_string("day 11/input.txt").expect("Could not read file");
    let lines = file
        .lines()
        .collect::<Vec<_>>();

    let mut octipii = lines.iter()
        .map(|x| x
            .chars()
            .map(|y| y.to_digit(10).unwrap() as u8)
            .collect::<Vec<u8>>()
        )
        .collect::<Vec<Vec<u8>>>();

    let total = octipii.len() * octipii[0].len();

    let mut total_flash: usize = 0;
    let mut step: usize = 1;
    loop {
        let mut been = HashSet::new();
        loop {
            let mut flash: usize = 0;
            for i in 0..octipii.len() {
                for j in 0..octipii[0].len() {
                    if octipii[i][j] >= 9 && !been.contains(&(i, j)) {
                        been.insert((i, j));
                        flash += 1;
                        if i > 0 { octipii[i - 1][j] += 1; }
                        if j > 0 { octipii[i][j - 1] += 1; }
                        if i < octipii.len() - 1 { octipii[i + 1][j] += 1; }
                        if j < octipii[0].len() - 1 { octipii[i][j + 1] += 1; }
                        if i > 0 && j > 0 { octipii[i - 1][j - 1] += 1; }
                        if i < octipii.len() - 1 && j > 0 { octipii[i + 1][j - 1] += 1; }
                        if i > 0 && j < octipii[0].len() - 1 { octipii[i - 1][j + 1] += 1; }
                        if i < octipii.len() - 1 && j < octipii[0].len() - 1 { octipii[i + 1][j + 1] += 1; }
                    }
                }
            }
            total_flash += flash;
            if flash == 0 { break; }
        }
        for i in 0..octipii.len() {
            for j in 0..octipii[0].len() {
                if been.contains(&(i, j)) {
                    octipii[i][j] = 0;
                } else { octipii[i][j] += 1; }
            }
        }
        if step == 100 {
            println!("Day 11, part 1: {}", total_flash);
        }
        if been.len() == total {
            println!("Day 11, part 2: {}", step);
            break;
        }
        step += 1;
    }
}