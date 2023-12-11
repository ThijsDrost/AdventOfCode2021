use std::fs;
use std::collections::HashMap};

pub(crate) fn day14() {
    let file = fs::read_to_string("day 14/input.txt").expect("Could not read file");
    let lines = file
        .lines()
        .collect::<Vec<_>>();

    let instructions = lines[2..lines.len()]
        .iter()
        .map(|x| x.split(" -> ").collect::<Vec<_>>())
        .map(|x| (x[0].chars().collect::<Vec<_>>(),
                  ([x[0].chars().collect::<Vec<_>>()[0], x[1].chars().next().unwrap()],
                   [x[1].chars().next().unwrap(), x[0].chars().collect::<Vec<_>>()[1]])))
        .collect::<HashMap<_, _>>();
    let mut polymer_values = instructions
        .iter()
        .map(|(key, _)| (key.clone(), 0))
        .collect::<HashMap<_, _>>();

    let polymer = lines[0].chars().collect::<Vec<_>>();
    for i in 1..polymer.len() {
        *polymer_values.get_mut(&polymer[i - 1..=i]).unwrap() += 1;
    }
    for index in 1..=40 {
        let mut new_polymer: HashMap<Vec<char>, i64> = instructions
            .iter()
            .map(|(key, _)| (key.clone(), 0))
            .collect::<HashMap<_, _>>();
        for (key, value) in &polymer_values {
            let connections = instructions.get(key).unwrap();
            *new_polymer.get_mut(&*connections.0.to_vec()).unwrap() += value;
            *new_polymer.get_mut(&*connections.1.to_vec()).unwrap() += value;
        }

        let (_, num) = get_num(&new_polymer, polymer[0]);
        polymer_values = new_polymer;
        if index == 10 {
            println!("Day 14, part 1: {}", num);
        }
        if index == 40 {
            println!("Day 14, part 2: {}", num);
        }
    }
}

fn get_num(map: &HashMap<Vec<char>, i64>, first_val: char) -> (HashMap<char, i64>, i64) {
    let mut values = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().map(|x| (x, 0)).collect::<HashMap<_, _>>();
    for (chars, value) in map {
        *values.get_mut(&chars[1]).unwrap() +=  value;
    }
    *values.get_mut(&first_val).unwrap() +=  1;
    let num = values.iter().map(|(_, x)| x).max().unwrap() - values.iter().map(|(_, x)| x).filter(|x| **x != 0).min().unwrap();
    (values, num)
}