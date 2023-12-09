use std::fs;

pub(crate) fn day1() {
    let file = fs::read_to_string("day 1/input.txt").expect("Could not read file");
    let lines: Vec<_> = file.lines().map(|x| x.parse::<i32>().unwrap()).collect();
    let total = lines.windows(2).map(|x| x[1] - x[0]).filter(|x| *x > 0).count();
    println!("Day 1, part 1: {}", total);
    let total = file.lines().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>()
        .windows(3).map(|x| x.iter().sum::<i32>()).collect::<Vec<_>>()
        .windows(2).map(|x| x[1] - x[0]).filter(|x| *x > 0).count();
    println!("Day 1, part 2: {}", total);
}