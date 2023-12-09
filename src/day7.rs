use std::fs;

pub(crate) fn day7(){
    let file = fs::read_to_string("day 7/input.txt").expect("Could not read file");
    let values:Vec<_> = file.lines().next().expect("oops").split(",").map(|x| x.parse::<i32>().unwrap()).collect();
    let mut location = 1;
    let mut cost: i32 = values.iter().map(|x| (x-0).abs()).sum();
    let mut new_cost: i32;

    for loc in 0..(*values.iter().max().unwrap()){
        new_cost = values.iter().map(|x| (x-loc).abs()).sum();
        if new_cost <= cost {
            cost = new_cost;
            location = loc;
        }
    }
    println!("Day 7, part 1: {}", cost);

    fn get_cost(values: &Vec<i32>, location: i32) -> i32 {
        fn cost(loc: i32, val: i32) -> i32 {
            let n = (val-loc).abs() as f64;
            (n*(n+1.0)/2.0) as i32
        }
        values.iter().map(|x| cost(*x, location)).sum()
    }

    cost = get_cost(&values, 0);
    for loc in 0..(*values.iter().max().unwrap()){
        new_cost = get_cost(&values, loc);
        if new_cost <= cost {
            cost = new_cost;
            location = loc;
        }
    }
    println!("Day 7, part 2: {}", cost);
}