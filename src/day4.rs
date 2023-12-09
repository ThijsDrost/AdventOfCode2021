use std::fs;

pub(crate) fn day4(){
    let file = fs::read_to_string("day 4/input.txt").expect("Could not read file");
    let mut lines = file.lines();
    let nums:Vec<_> = lines.next().expect("Helpie").split(',').map(|x| x.parse::<i32>().unwrap()).collect();
    let mut index = 0;
    let mut board = vec![vec![0; 5]; 5];
    let mut result = vec![(0, 0, 0); 0];
    for num in lines {
        if num == "" {
            continue;
        }
        board[index] = num.split(' ').filter(|x| *x != "").map(|x| x.parse::<i32>().unwrap()).collect();
        index += 1;
        if index == 5 {
            index = 0;
            let mut found = vec![vec![0; 5]; 5];
            for (round, num) in nums.iter().enumerate() {
                for i in 0..5 {
                    for j in 0..5 {
                        if board[i][j] == *num {
                            found[i][j] = 1;
                        }
                    }
                }
                if (found.iter().map(|x| x.iter().sum::<i32>()).filter(|x| *x == 5).count() > 0) |
                    (found.iter()
                        .fold(vec![0; 5], |old, new| old.iter().zip(new.iter()).map(|(a, &b)| a+b).collect::<Vec<_>>())
                        .iter()
                        .filter(|x| **x == 5)
                        .count() > 0)
                {
                    // println!("Found {}", round);
                    let mut sum = 0;
                    for i in 0..5 {
                        for j in 0..5 {
                            if found[i][j] == 0 {
                                sum += board[i][j];
                            }
                        }
                    }
                    result.append(&mut vec![(round, *num, sum)]);
                    break
                }
            }
        }
    }
    let min_val = result.iter().min_by(|x, y| x.0.cmp(&y.0)).unwrap();
    println!("Day 4, part 1: {}", min_val.1*min_val.2);
    let max_val = result.iter().max_by(|x, y| x.0.cmp(&y.0)).unwrap();
    println!("Day 4, part 2: {}", max_val.1*max_val.2);
}