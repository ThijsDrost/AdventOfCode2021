use std::fs;
use std::collections::HashMap;

pub(crate) fn day2(){
    let file = fs::read_to_string("day 2/input.txt").expect("Could not read file");
    let direction_map = HashMap::from([
        ("forward", (1, 0)),
        ("up", (0, -1)),
        ("down", (0, 1)),
    ]);
    let direction:Vec<_> = file.lines().map(|x| direction_map[x.split(" ").collect::<Vec<_>>()[0]]).collect();
    let distance:Vec<_> = file.lines().map(|x| x.split(" ").collect::<Vec<_>>()[1].parse::<i32>().unwrap()).collect();
    let travel_dist = direction.iter().zip(distance.iter()).map(|(x, y)| (x.0 * y, x.1 * y)).collect::<Vec<_>>();
    let traveled = travel_dist.iter().fold((0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1));
    println!("Day 2, part 1: {}", traveled.0* traveled.1);

    let traveled = travel_dist.iter().fold((0, 0, 0), |old, new| (new.0 + old.0, new.1 + old.1, new.0*old.1+old.2));
    println!("Day 2, part 2: {}", traveled.0*traveled.2);
}