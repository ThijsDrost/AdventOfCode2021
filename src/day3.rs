use std::fs;

pub(crate) fn day3(){
    let lines = fs::read_to_string("day 3/input.txt").expect("Could not read file");
    let mut nums:Vec<_> = lines.lines().map(|x| x.chars().map(|y| y.to_digit(10).unwrap() as i32).collect::<Vec<_>>()).collect();
    let mut sums:Vec<_> = nums.iter().fold(vec![0; nums[0].len()], |old, new| add(&old, new));
    let mut num: Vec<_> = sums.iter().map(|x| (*x > (nums.len()/2).try_into().unwrap()) as i32).collect();
    let gamma = num.iter().rev().enumerate().fold(0, |old, (i, value)| old + value*2i32.pow(i as u32));
    let epsilon = num.iter().rev().enumerate().fold(0, |old, (i, value)| old + (value-1).abs() *2i32.pow(i as u32));

    println!("Day 3, part 1: {}", gamma*epsilon);

    let mut index = 0;
    while index < nums[0].len() {
        nums = nums.iter()
            .enumerate()
            .filter(|(i, val)| val[index] == num[index])
            .map(|(_, v)| v.iter().map(|x| *x).collect())
            .collect();
        if nums.len() <= 1 {
            break;
        }
        sums = nums.iter().fold(vec![0; nums[0].len()], |old, new| add(&old, new));
        num = sums.iter().map(|x| (2*(*x) >= (nums.len().try_into().unwrap())) as i32).collect();
        index += 1;
    }
    if index == nums[0].len() {
        println!("No solution found");
    }
    let oxygen = nums[0].iter().rev().enumerate().fold(0, |old, (i, value)| old + value*2i32.pow(i as u32));

    nums = lines.lines().map(|x| x.chars().map(|y| y.to_digit(10).unwrap() as i32).collect::<Vec<_>>()).collect();
    let mut index = 0;
    while index < nums[0].len() {
        sums = nums.iter().fold(vec![0; nums[0].len()], |old, new| add(&old, new));
        num = sums.iter().map(|x| (2*(*x) < (nums.len().try_into().unwrap())) as i32).collect();
        nums = nums.iter()
            .enumerate()
            .filter(|(i, val)| val[index] == num[index])
            .map(|(_, v)| v.iter().map(|x| *x).collect())
            .collect();
        if nums.len() <= 1 {
            break;
        }
        index += 1;
    }
    if index == nums[0].len() {
        println!("No solution found");
    }
    let co2 = nums[0].iter().rev().enumerate().fold(0, |old, (i, value)| old + value*2i32.pow(i as u32));
    println!("Day 3, part 2: {}", oxygen*co2);
}

fn add(a: &Vec<i32>, b: &Vec<i32>) -> Vec<i32> {
    let mut c = vec![0; a.len()];
    for i in 0..a.len() {
        c[i] = a[i] + b[i];
    }
    return c;
}
