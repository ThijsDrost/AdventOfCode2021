use std::collections::HashMap;
use std::fs;

pub(crate) fn day6(){
    let mut fishes: HashMap<i32, i64> = HashMap::from([
        (0, 0),
        (1, 0),
        (2, 0),
        (3, 0),
        (4, 0),
        (5, 0),
        (6, 0),
        (7, 0),
        (8, 0),
    ]);
    let file = fs::read_to_string("day 6/input.txt").expect("Could not read file");
    let days = 256;
    let values:Vec<_> = file.lines().next().expect("oops").split(",").map(|x| x.parse::<i32>().unwrap()).collect();
    for val in &values {
        fishes.insert(*val, fishes[&val] + 1);
    }
    let mut old_fishes = fishes.clone();
    for day in 0..days {
        for key in (0..=8).rev() {
            if key != 0 {
                fishes.insert(key - 1, old_fishes[&key]);
            } else {
                fishes.insert(8, old_fishes[&0]);
                fishes.insert(6, old_fishes[&7] + old_fishes[&0]);
            }
        }
        old_fishes = fishes.clone();
        if day == 79 {
            let sum = fishes.values().sum::<i64>();
            println!("Day 6, part 1: {}", sum);
        }
    }
    let sum = fishes.values().sum::<i64>();
    println!("Day 6, part 2: {}", sum);
}