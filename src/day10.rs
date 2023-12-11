use std::fs;

pub(crate) fn day10() {
    let file = fs::read_to_string("day 10/input.txt").expect("Could not read file");
    let lines = file
        .lines()
        .collect::<Vec<_>>();
    let corrupted = lines.iter()
        .map(|x| corrupted(x.chars().collect::<Vec<_>>()));

    // println!("{:?}", corrupted);
    println!("Day 10, part 1: {}", corrupted.clone().filter(|x| x.1).map(|x| x.0).sum::<usize>());
    let mut values = corrupted.filter(|x| !x.1).map(|x| x.0).collect::<Vec<_>>();
    values.sort_by(|a, b| a.cmp(b));
    println!("Day 10, part 2: {}", values[values.len()/2]);
}

fn corrupted(input: Vec<char>) -> (usize, bool) {
    let mut opened: Vec<char> = Vec::new();
    for &value in input.iter(){
        match value {
            '{' => opened.push('}'),
            '}' => if opened.pop() != Some('}') { return (1197, true); },
            '<' => opened.push('>'),
            '>' => if opened.pop() != Some('>') { return (25137, true); },
            '[' => opened.push(']'),
            ']' => if opened.pop() != Some(']') { return (57, true); },
            '(' => opened.push(')'),
            ')' => if opened.pop() != Some(')') { return (3, true); },
            _ => (),
        }
    }
    let mut total: usize = 0;
    for character in opened.iter().rev() {
        total *= 5;
        match character {
            '}' => total += 3,
            '>' => total += 4,
            ']' => total += 2,
            ')' => total += 1,
            _ => (),
        }
    }
    (total, false)
}