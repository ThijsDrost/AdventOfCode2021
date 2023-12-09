use std::fs;

pub(crate) fn day8() {
    let file = fs::read_to_string("day 8/input.txt").expect("Could not read file");
    let values = file.lines()
        .map(|x| x
            .split(" | ").collect::<Vec<_>>()[1]
            .split(" ")
                .map(|z| z.len() as i32)
                .collect::<Vec<i32>>()
            )
            .collect::<Vec<Vec<i32>>>();
    let total = values.iter().map(|x| x.iter().filter(|z| (**z == 2) | (**z == 3) | (**z == 4) | (**z == 7)).count()).sum::<usize>();
    println!("Day 8, part 1: {}", total);

}