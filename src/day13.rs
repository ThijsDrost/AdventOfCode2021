use std::fs;
use std::collections::HashSet;

pub(crate) fn day13() {
    let file = fs::read_to_string("day 13/input.txt").expect("Could not read file");
    let lines = file
        .lines()
        .collect::<Vec<_>>();
    let mut dots = lines
        .iter()
        .filter(|&&x| (x.chars().next() != None) && (x.chars().next().unwrap() != 'f'))
        .map(|&x| x.split(",")
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let folds = lines
        .iter()
        .filter(|&&x| (x.chars().next() != None) && (x.chars().next().unwrap() == 'f'))
        .map(|&x| x.chars().collect::<Vec<_>>())
        .map(|x| (x[11], x[13..x.len()].iter().rev().enumerate().map(|(index, c)| c.to_digit(10).unwrap()*10u32.pow(index as u32)).sum::<u32>()))
        .collect::<Vec<_>>();
    for (fold_dir, fold_loc) in folds.iter() {
        let mut index = 0;
        match fold_dir {
            'x' => {
                 index = 0;
            },
            'y' => {
                index = 1;
            },
            x => {panic!("Invalid fold direction: {}", x)},
        }
        for dot in dots.iter_mut() {
            if dot[index] > *fold_loc {
                dot[index] = 2*fold_loc - dot[index];
            }
        }
        let dots_set = dots.iter().collect::<HashSet<_>>();
        println!("{:?}", dots_set.len());
    }
    let x_max = dots.iter().map(|x| x[0]).max().unwrap();
    let y_max = dots.iter().map(|x| x[1]).max().unwrap();
    let mut grid = vec![vec!['.'; (x_max+1) as usize]; (y_max+1) as usize];
    for dot in dots.iter() {
        grid[dot[1] as usize][dot[0] as usize] = '#';
    }
    for row in grid.iter() {
        for char in row.iter() {
            print!("{}", char);
        }
        println!(" ");
    }
}