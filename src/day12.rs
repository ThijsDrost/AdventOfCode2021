use std::fs;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Node {
    name: String,
    connections: Vec<String>,
    upper: bool,
}

pub(crate) fn day12() {
    let file = fs::read_to_string("day 12/input.txt").expect("Could not read file");
    let mut nodes: HashMap<String, Node> = HashMap::new();
    let lines = file
        .lines()
        .collect::<Vec<_>>();
    for line in lines {
        let mut parts = line.split("-");
        let node1 = parts.next().unwrap().to_string();
        let node2 = parts.next().unwrap().to_string();
        if !nodes.contains_key(&node1) {
            nodes.insert(node1.clone(), Node {
                name: node1.clone(),
                connections: vec![node2.clone()],
                upper: node1.to_lowercase() != node1
            });
        } else {
            nodes.get_mut(&node1).unwrap().connections.push(node2.clone());
        }
        if !nodes.contains_key(&node2) {
            nodes.insert(node2.clone(), Node {
                name: node2.clone(),
                connections: vec![node1.clone()],
                upper: node2.to_lowercase() != node2
            });
        } else {
            nodes.get_mut(&node2).unwrap().connections.push(node1);
        }
    }
    println!("Day 12, part 1: {}", find_path(&nodes, "start".to_string(), "end".to_string(), HashSet::from(["start".to_string()])));
    println!("Day 12, part 2: {}", find_path2(&nodes, "start".to_string(), "end".to_string(),
                                              HashSet::from(["start".to_string()]), false,
                                              vec!["start".to_string()], HashSet::new()));
}

fn find_path(nodes: &HashMap<String, Node>, current: String, end: String, been: HashSet<String>) -> i32{
    let mut total = 0;
    for connection in nodes.get(&current).unwrap().connections.iter() {
        if connection == &end {
            total += 1;
        } else {
            if !been.contains(connection) {
                let mut new_been = been.clone();
                if !nodes.get(connection).unwrap().upper {
                    new_been.insert(connection.to_lowercase());
                }
                total += find_path(nodes, connection.clone(), end.clone(), new_been);
            }
        }
    }
    return total;
}

fn find_path2(nodes: &HashMap<String, Node>, current: String, end: String, been: HashSet<String>,
              twice: bool, path: Vec<String>, been_small: HashSet<String>) -> i32{
    let mut total = 0;
    for connection in nodes.get(&current).unwrap().connections.iter() {
        if connection == &end {
            total += 1;
        } else {
            if !been.contains(connection) {
                let mut new_been = been.clone();
                let mut new_been_small = been_small.clone();
                let mut new_path = path.clone();
                let mut twice = twice;
                new_path.push(connection.clone());
                if !nodes.get(connection).unwrap().upper {
                    if !twice {
                        if been_small.contains(connection){
                            twice = true;
                            new_been.insert(connection.clone());
                        }
                        else {
                            new_been_small.insert(connection.clone());
                        }
                    }
                    else {
                        new_been.insert(connection.clone());
                        if been_small.contains(connection) {
                            continue;
                        }
                    }
                }
                total += find_path2(nodes, connection.clone(), end.clone(), new_been, twice,
                                    new_path, new_been_small);
            }
        }
    }
    return total;
}
