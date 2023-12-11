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
    let mut total = 0;
    for line in file.lines() {
        let mut values = vec![vec![' '; 0]; 10];
        let mut start = line.split(" | ").collect::<Vec<_>>()[0].split(" ").collect::<Vec<_>>();
        let end = line.split(" | ").collect::<Vec<_>>()[1].split(" ").collect::<Vec<_>>();
        values[1] = start.iter().filter(|z| z.len() == 2).collect::<Vec<_>>()[0].chars().collect::<Vec<_>>();
        values[4] = start.iter().filter(|z| z.len() == 4).collect::<Vec<_>>()[0].chars().collect::<Vec<_>>();
        values[7] = start.iter().filter(|z| z.len() == 3).collect::<Vec<_>>()[0].chars().collect::<Vec<_>>();
        values[8] = start.iter().filter(|z| z.len() == 7).collect::<Vec<_>>()[0].chars().collect::<Vec<_>>();
        values[3] = start.iter().filter(|z| (z.len() == 5) & (z.contains(values[1][0])) & (z.contains(values[1][1]))).collect::<Vec<_>>()[0].chars().collect::<Vec<_>>();
        values[9] = start.iter().filter(|z| (z.len() == 6) & values[3].iter().all(|x| z.contains(*x))).collect::<Vec<_>>()[0].chars().collect::<Vec<_>>();
        values[0] = start.iter().filter(|z| (z.len() == 6) & values[1].iter().all(|x| z.contains(*x)) & !values[3].iter().all(|x| z.contains(*x))).collect::<Vec<_>>()[0].chars().collect::<Vec<_>>();
        values[6] = start.iter().filter(|z| (z.len() == 6) & (z.chars().collect::<Vec<_>>() != values[0]) & (z.chars().collect::<Vec<_>>() != values[9])).collect::<Vec<_>>()[0].chars().collect::<Vec<_>>();
        values[5] = start.iter().filter(|z| (z.len() == 5) & z.chars().all(|x| values[6].contains(&x))).collect::<Vec<_>>()[0].chars().collect::<Vec<_>>();
        values[2] = start.iter().filter(|z| (z.len() == 5) & (z.chars().collect::<Vec<_>>() != values[3]) & (z.chars().collect::<Vec<_>>() != values[5])).collect::<Vec<_>>()[0].chars().collect::<Vec<_>>();

        for (num, value) in end.iter().rev().enumerate() {
            let char_vec = value.chars().collect::<Vec<_>>();
            for (index, val) in values.iter().enumerate() {
                if (val.len() == char_vec.len()) && char_vec.iter().all(|x| val.contains(&x)) {
                    total += index * 10_usize.pow(num as u32);
                }
            }
        }
    }
    println!("Day 8, part 2: {}", total);
}