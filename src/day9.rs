use std::fs;
use std::collections::{BinaryHeap, HashSet};

pub(crate) fn day9() {
    let file = fs::read_to_string("day 9/input.txt").expect("Could not read file");
    let heights: Vec<_> = file.lines()
        .map(|x| x
            .chars()
            .map(|y| y
                .to_digit(10)
                .unwrap()
            ).collect::<Vec<_>>()
        ).collect::<Vec<_>>();
    let locations = heights
            .iter()
            .enumerate()
            .map(|(i, values)| values
                .iter()
                .enumerate()
                .filter(|(j, value)| !(((i > 0) && (heights[i][*j] >= heights[i-1][*j]))
                    || ((j > &0) && (heights[i][*j] >= heights[i][*j-1]))
                    || ((i < heights.len()-1) && (heights[i][*j] >= heights[i+1][*j]))
                    || ((j < &(heights[i].len()-1)) && (heights[i][*j] >= heights[i][*j+1])))
                )
                .map(|(j, &value)| (i, j))
                .collect::<Vec<(usize, usize)>>()
            )
        .collect::<Vec<_>>();

    let flat_loc = locations.iter().flatten().collect::<Vec<_>>();

    let values = flat_loc
        .iter()
        .map(|y| &heights[y.0][y.1]+1)
        .sum::<u32>();

    let mut basins: Vec<usize> = vec![0; flat_loc.len()];
    for (index, loc) in flat_loc.iter().enumerate() {
        basins[index] = flood(loc.0 as i32, loc.1 as i32, &heights);
    }

    let mut max_basins: Vec<usize> = vec![0; 3];
    for basin in basins.iter() {
        if *basin > max_basins[0] {
            max_basins[2] = max_basins[1];
            max_basins[1] = max_basins[0];
            max_basins[0] = *basin;
        }
        else if *basin > max_basins[1] {
            max_basins[2] = max_basins[1];
            max_basins[1] = *basin;
        }
        else if *basin > max_basins[2] {
            max_basins[2] = *basin;
        }
    }

    println!("{:?}", flat_loc);
    println!("{:?}", max_basins);
    println!("Day 8, part 1: {}", values);
    println!("Day 8, part 2: {:?}", max_basins[0]*max_basins[1]*max_basins[2]);

}

fn flood(x: i32, y: i32, map: &Vec<Vec<u32>>) -> usize {
    let mut been: HashSet<(i32, i32)> = HashSet::new();
    let mut to_visit: Vec<(i32, i32)> = Vec::new();
    been.insert((x, y));
    to_visit.push((x, y));
    let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
    while to_visit.len() != 0 {
        let (x, y) = to_visit.pop().unwrap();
        for (dx, dy) in directions.iter() {
            let (x, y) = (x + dx, y + dy);
            if (x >= 0) && (y >= 0) && (x < map.len() as i32) && (y < map[0].len() as i32) {
                if !been.contains(&(x, y)) && (map[x as usize][y as usize] != 9){
                    been.insert((x, y));
                    to_visit.push((x, y));
                }
            }
        }
    }
    return been.len();
}

