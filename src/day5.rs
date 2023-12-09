use std::fs;

pub(crate) fn day5(){
    let file = fs::read_to_string("day 5/input.txt").expect("Could not read file");
    let values:Vec<_> = file.lines()
        .map(|x| x.split(" -> ")
            .map(|y| y.split(",")
                .map(|z| z
                    .parse::<i32>()
                    .unwrap())
                .collect::<Vec<_>>())
            .collect::<Vec<_>>())
        .collect();
    let filtered_values:Vec<_> = values.iter().filter(|x| (x[0][1] == x[1][1]) | (x[0][0] == x[1][0])).collect();
    let mut map = vec![vec![0; max_val(&values, 1)+1]; max_val(&values, 0)+1];
    // let mut map = vec![vec![0; 1000]; 1000];
    for value in filtered_values {
        for i in value[0][0].min(value[1][0])..(value[0][0].max(value[1][0])+1){
            for j in value[0][1].min(value[1][1])..(value[0][1].max(value[1][1])+1) {
                map[i as usize][j as usize] += 1;
            }
        }
    }

    let sum = map.iter().map(|x| x.iter().filter(|y| **y > 1).count()).sum::<usize>();
    println!("Day 5, part 1: {}", sum);

    let filtered_values:Vec<_> = values.iter().filter(|x| (x[0][1] != x[1][1]) & (x[0][0] != x[1][0])).collect();
    // let mut map = vec![vec![0; max_val_good(&filtered_values, 1)+1]; max_val_good(&filtered_values, 0)+1];
    for value in filtered_values {
        let range1:Vec<_> = match value[0][0] < value[1][0] {
            true => (value[0][0]..=value[1][0]).collect(),
            false => (value[1][0]..=value[0][0]).rev().collect(),
        };
        let range2:Vec<_> = match value[0][1] < value[1][1] {
            true => (value[0][1]..=value[1][1]).collect(),
            false => (value[1][1]..=value[0][1]).rev().collect(),
        };
        // println!("range1: {:?}, range2: {:?}, {:?}", range1, range2, values);
        for (i, j) in range1.iter().zip(range2.iter()) {
            map[*i as usize][*j as usize] += 1;
        }
    }

    let sum = map.iter().map(|x| x.iter().filter(|y| **y > 1).count()).sum::<usize>();
    println!("Day 5, part 2: {}", sum);

}

fn max_val(values: &Vec<Vec<Vec<i32>>>, index: usize) -> usize {
    fn max_val(values: &Vec<Vec<i32>>, index: usize) -> i32 {
        values.iter().max_by(|x, y| x[index].cmp(&y[index])).unwrap()[index]
    }
    values.iter()
        .max_by(|x, y| max_val(x, index).cmp(&max_val(y, index)))
        .unwrap()
        .iter()
        .max_by(|x, y| x[index].cmp(&y[index]))
        .unwrap()[index] as usize
}