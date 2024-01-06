use std::io::{self, Read};
use std::env;
use std::collections::VecDeque;
use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq)]
#[derive(Debug)]
#[derive(Clone, Copy)]
enum Direction {
    N,
    E,
    S,
    W,
}

type Node = (i32, i32, Direction);

fn north(node: &Node) -> Node {
    return (node.0 - 1, node.1, Direction::N);
}

fn east(node: &Node) -> Node {
    return (node.0, node.1 + 1, Direction::E);
}

fn south(node: &Node) -> Node {
    return (node.0 + 1, node.1, Direction::S);
}

fn west(node: &Node) -> Node {
    return (node.0, node.1 - 1, Direction::W);
}

fn neighbors(node: &Node, b: &Vec<Vec<char>>) -> Vec<Node> {
    let (i, j, d) = *node;
    let c = b[i as usize][j as usize];
    let raw_neighbords: Vec<Node> = match c {
        '.' => match d {
            Direction::N => vec![north(node)],
            Direction::W => vec![west(node)],
            Direction::S => vec![south(node)],
            Direction::E => vec![east(node)],
        },
        '|' => match d {
            Direction::N => vec![north(node)],
            Direction::W => vec![north(node), south(node)],
            Direction::S => vec![south(node)],
            Direction::E => vec![south(node), north(node)],
        },
        '-' => match d {
            Direction::N => vec![east(node), west(node)],
            Direction::W => vec![west(node)],
            Direction::S => vec![east(node), west(node)],
            Direction::E => vec![east(node)],
        },
        '/' => match d {
            Direction::N => vec![east(node)],
            Direction::E => vec![north(node)],
            Direction::S => vec![west(node)],
            Direction::W => vec![south(node)],
        }
        '\\' => match d {
            Direction::N => vec![west(node)],
            Direction::E => vec![south(node)],
            Direction::S => vec![east(node)],
            Direction::W => vec![north(node)],
        },
        _ => todo!()
    };

    let n = b.len() as i32;
    let m = b[0].len() as i32;

    let f = |node: &Node| {
        let (i, j, _) = *node;
        return i >= 0 && i < n && j >= 0 && j < m;
    };
    return raw_neighbords.into_iter().filter(f).collect();
}

fn count_energized(s: Node, b: &Vec<Vec<char>>) -> i32 {
    let n = b.len();
    let m = b[0].len();
    let mut energized: Vec<Vec<bool>> = vec![vec![false; m]; n];

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back(s);

    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();
        energized[node.0 as usize][node.1 as usize] = true;
        visited.insert(node.clone());
        let nbs = neighbors(&node, &b);

        for nb in nbs {
            if !visited.contains(&nb) {
                queue.push_back(nb);
            }
        }
    }

    let mut t = 0;
    for i in 0..energized.len() {
        for j in 0..energized[i].len() {
            if energized[i][j] {
                t += 1;
            }
        }
    }
    return t;
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let is_2nd = args.len() > 0 && args[0] == "2";

    let mut input_str = String::new();
    io::stdin().read_to_string(&mut input_str).expect("Failed to read input");

    let lines = input_str.lines();
    let mut board: Vec<Vec<char>> = vec![];
    for line in lines {
        let row = line.chars().collect::<Vec<char>>();
        board.push(row);
    }

    let n = board.len();
    let m = board[0].len();

    if is_2nd {
        let mut maxt = 0;
        for i in 0..n {
            let s = (i as i32, 0, Direction::E);
            maxt = maxt.max(count_energized(s, &board));
            let s = (i as i32, (m - 1) as i32, Direction::W);
            maxt = maxt.max(count_energized(s, &board));
        }
        for j in 0..m {
            let s = (0, j as i32, Direction::S);
            maxt = maxt.max(count_energized(s, &board));
            let s = ((n - 1) as i32, j as i32, Direction::N);
            maxt = maxt.max(count_energized(s, &board));
        }

        println!("{}", maxt);
    } else {
        let s = (0, 0, Direction::E);
        let t = count_energized(s, &board);
        println!("{}", t);
    }
}
