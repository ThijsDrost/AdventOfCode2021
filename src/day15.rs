use std::fs;
use std::cmp::Ordering;

pub(crate) fn day15() {
    let file = fs::read_to_string("day 15/input.txt").expect("Could not read file");
    let grid = file
        .lines()
        .map(|line| line
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>()
        )
        .collect::<Vec<_>>();
    let mut new_grid: Vec<Vec<usize>> = vec![vec![0; 5*grid[0].len()]; 5*grid.len()];
    for i in 0..5 {
        for j in 0..5 {
            for k in 0..grid.len() {
                for l in 0..grid[0].len() {
                    new_grid[i*grid.len() + k][j*grid[0].len() + l] = (grid[k][l]+i+j)%10 + (grid[k][l]+i+j)/10;
                }
            }
        }
    }
    dijkstra(&grid, true);
    dijkstra(&new_grid, true);
    dijkstra(&grid, false);
    dijkstra(&new_grid, false);
}


fn dijkstra(grid: &Vec<Vec<usize>>, finish:bool) {
    let mut min_values = grid.iter().map(|x| x.iter().map(|y| usize::MAX).collect::<Vec<_>>()).collect::<Vec<_>>();
    let average_cost = 1;
    let end = (grid.len() - 1, grid[0].len() - 1);
    let mut queue = Vec::new();
    queue.push(Node::new(0, 0, 0, (end.0 + end.1) * average_cost, 0));
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    while let Some(node) = queue.pop() {
        for d in directions {
            let x = node.x + d.0;
            let y = node.y + d.1;
            if x < 0 || y < 0 {
                continue;
            }

            let x = x as usize;
            let y = y as usize;
            if  x >= grid.len() || y >= grid[0].len() {
                continue;
            }
            let g = node.g + grid[x][y];
            let h = (end.0 - x + end.1 - y) * average_cost;
            if x == end.0 && y == end.1 && finish {
                println!("{:?}", g);
                return;
            }
            if min_values[x][y] <= g {
                continue;
            }
            else {
                min_values[x][y] = g;
            }
            queue.push(Node::new(x as i32, y as i32, g, h, node.steps + 1));
        }
        queue.sort();
    }
    println!("{:?}", min_values[end.0][end.1]);

}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Node {
    x: i32,
    y: i32,
    g: usize,
    h: usize,
    steps: usize,
}

impl Node {
    fn new(x: i32, y: i32, g: usize, h: usize, steps: usize) -> Self {
        Self {
            x,
            y,
            g,
            h,
            steps,
        }
    }

    fn f(&self) -> usize {
        self.g + self.h
    }
}

impl PartialOrd<Self> for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f().cmp(&self.f()).then_with(|| other.g.cmp(&self.g))
    }
}
