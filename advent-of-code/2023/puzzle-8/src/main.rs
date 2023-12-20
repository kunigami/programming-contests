use std::io::{self, Read};
use std::env;
use std::collections::HashMap;

fn solve1(directions: &Vec<char>, adj: &HashMap<&str, (&str, &str)>) -> i64 {
    let mut node = "AAA";
    let mut idx: i64 = 0;
    println!("Vector:\n{:#?}", directions);
    loop {
        println!("{}", node);
        let dir = directions[(idx % (directions.len() as i64)) as usize];
        idx += 1;
        let neighbor = adj.get(&node).unwrap();
        if dir == 'L' {
            node = neighbor.0;
        } else {
            node = neighbor.1;
        }
        if node == "ZZZ" { break; }
    }
    return idx;
}

fn solve2(directions: &Vec<char>, adj: &HashMap<&str, (&str, &str)>) -> i64 {
    let mut lengths: Vec<i64> = vec![];

    for key in adj.keys() {

        if !key.contains("A") {
            continue;
        }

        let mut visited = HashMap::new();
        let mut dist = 0;
        let mut node = *key;

        let node_id = format!("{}:{}", node, dist);
        visited.insert(node_id.clone(), 0);
        println!("Cycle of {}", node);
        loop {
            let idx = dist % (directions.len() as i64) as usize;
            let dir = directions[idx];

            let neighbor = adj.get(&node).unwrap();
            if dir == 'L' {
                node = neighbor.0;
            } else {
                node = neighbor.1;
            }
            dist += 1;

            let node_id = format!("{}:{}", node, idx);

            // found a cycle.
            match visited.get(&node_id) {
                Some(value) => {
                    println!("Cycle detected at {} ({}) with length {}", node, value, dist - value);
                    lengths.push((dist - value) as i64);
                    break;
                }
                None => {}
            }

            visited.insert(node_id.clone(), dist);

            if node.contains("Z") {
                println!("sink {}, {}", node, dist);
            }
        }
    }
    // For the particular instance of this problem, all cycles
    // are all of the form a + ka, so to find the next joint occurrence
    // we just need to find the LCM of all the lengths.
    let mut divider: i64 = 2;

    let max_len = lengths.iter().max().unwrap().clone();
    let mut prod: i64 = 1;
    while divider * divider <= max_len {
        let mut changed = false;
        for i in 0..lengths.len() {
            if lengths[i] % divider == 0 {
                lengths[i] /= divider;
                changed = true;
            }
        }

        if !changed {
            divider += 1;
        } else {
            prod *= divider;
        }
    }
    let remaining_divs = lengths.clone();
    for divider in remaining_divs {
        let mut changed = false;
        for i in 0..lengths.len() {
            if lengths[i] % divider == 0 {
                lengths[i] /= divider;
                changed = true;
            }
        }
        if changed {
            prod *= divider;
        }
    }

    return prod as i64;
}


fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let is_2nd = args.len() > 0 && args[0] == "2";

    let mut input_str = String::new();
    io::stdin().read_to_string(&mut input_str).expect("Failed to read input");
    let mut lines = input_str.lines();

    let directions = lines.next().unwrap().chars().collect::<Vec<_>>();

    lines.next(); // empty line

    let mut adj = HashMap::new();

    while let Some(line) = lines.next() {
        let parts: Vec<&str> = line.trim().split('=').collect();
        let src = parts[0].trim();
        let chars_to_trim: &[char] = &['(', ')'];
        let mut dsts_str = parts[1].trim();
        dsts_str = dsts_str.trim_matches(chars_to_trim);
        let dsts: Vec<_> = dsts_str.split(',').collect();
        let dst_left = dsts[0].trim();
        let dst_right = dsts[1].trim();

        adj.insert(
            src,
            (dst_left, dst_right),
        );
        println!("{} {} {}", src, dst_left, dst_right);
    }
    println!("===");

    if !is_2nd {
        println!("{}", solve1(&directions, &adj));
    } else {
        println!("{}", solve2(&directions, &adj));
    }
}
