use std::io::{self, Read};
use std::env;
use priority_queue::PriorityQueue;
use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq)]
#[derive(Debug)]
#[derive(Clone, Copy)]
enum Direction {
    N,
    E,
    S,
    W,
}

// i, j, direction, level
type Node = (i32, i32, i32, Direction);

fn get_neighbors(node: &Node, mat: &Vec<Vec<i32>>, min_lvl: &i32, max_lvl: &i32) -> Vec<Node> {
    let n = mat.len() as i32;
    let m = mat[0].len() as i32;
    let &(i, j, level, dir) = node;
    let deltas = vec![
        (-1, 0, Direction::N),
        (0, 1, Direction::E),
        (1, 0, Direction::S),
        (0, -1, Direction::W),
    ];
    let opposites = HashMap::from([
        (Direction::N, Direction::S),
        (Direction::S, Direction::N),
        (Direction::E, Direction::W),
        (Direction::W, Direction::E),
    ]);

    let mut neighbors = vec![];
    let odir = opposites.get(&dir).unwrap();
    for d in deltas {
        let (di, dj, ddir) = d;
        let ni = i + di;
        let nj = j + dj;
        if ni < 0 || ni >= n || nj < 0 || nj >= m {
            continue;
        }

        // in the same direction, it needs to up its level
        if ddir == dir {
            if level + 1 < *max_lvl { // cannot go on the same direction max_level times
                neighbors.push((ni, j + dj, level + 1, ddir));
            }
        } else if ddir != *odir && level + 1 >= *min_lvl {
            // 1. do not go backward
            // 2. cannot turn unless a certain lvl
            neighbors.push((i + di, j + dj, 0, ddir));
        }
    }
    neighbors
}

fn find_shortest_path(s: &Node, mat: &Vec<Vec<i32>>, min_lvl: &i32, max_lvl: &i32) -> i32 {
    let n = mat.len() as i32;
    let m = mat[0].len() as i32;
    let mut pq: PriorityQueue<Node, i32> = PriorityQueue::new();
    let mut visited: HashSet<Node> = HashSet::new();
    let mut sp = -123456789;

    pq.push(*s, 0);
    while !pq.is_empty() {
        let (node, curr_pri) = pq.pop().unwrap();
        let (i, j, _, _) = node;

        if i == n-1 && j == m-1 {
            sp = sp.max(curr_pri);
        }

        visited.insert(node);

        let neighbors = get_neighbors(&node, mat, min_lvl, max_lvl);
        for neighbor in neighbors {
            if visited.contains(&neighbor) {
                continue;
            }
            let (ni, nj, _, _) = neighbor;
            // already in the queue
            let pri_opt = pq.get_priority(&neighbor);
            let new_pri = curr_pri - mat[ni as usize][nj as usize];
            match pri_opt {
                Some(pri) => {
                    if new_pri >= *pri {
                        pq.change_priority(&neighbor, new_pri);
                    }
                },
                None => {
                    pq.push(neighbor, new_pri);
                }
            }
        }
    }
    -sp
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let is_2nd = args.len() > 0 && args[0] == "2";

    let mut input_str = String::new();
    io::stdin().read_to_string(&mut input_str).expect("Failed to read input");

    let lines = input_str.lines();
    let mut mat: Vec<Vec<i32>> = vec![];
    for line in lines {
        let row = line.chars().map(|c| { (c as i32) - ('0' as i32) }).collect::<Vec<i32>>();
        mat.push(row);
    }

    let min_level;
    let max_level;
    if !is_2nd {
        min_level = 0;
        max_level = 3;
    } else {
        min_level = 4;
        max_level = 10;
    }

    let dirs = vec![Direction::E, Direction::S];
    let mut sp = 123456789;
    for dir in dirs {
        let s = (0, 0, 0, dir);
        sp = sp.min(find_shortest_path(&s, &mat, &min_level, &max_level));
    }
    println!("{}", sp);
}
