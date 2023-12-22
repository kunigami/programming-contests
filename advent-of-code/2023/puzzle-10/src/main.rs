use std::io::{self, Read};
use std::env;
use std::collections::HashMap;
use std::collections::VecDeque;

fn to_i32(p: &(usize, usize)) -> (i32, i32) {
    return (p.0 as i32, p.1 as i32);
}

fn get_neighbors_raw(cu: &(usize, usize), mat: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let c = to_i32(cu);
    let n = mat.len() as i32;
    let m = mat[0].len() as i32;
    let v = mat[c.0 as usize][c.1 as usize];
    let ns : Vec<(i32, i32)> = match v {
        '7' => vec![(c.0 + 1, c.1), (c.0, c.1 - 1)],
        'F' => vec![(c.0 + 1, c.1), (c.0, c.1 + 1)],
        'L' => vec![(c.0 - 1, c.1), (c.0, c.1 + 1)],
        'J' => vec![(c.0 - 1, c.1), (c.0, c.1 - 1)],
        '|' => vec![(c.0 - 1, c.1), (c.0 + 1, c.1)],
        '-' => vec![(c.0, c.1 - 1), (c.0, c.1 + 1)],
        'S' => vec![
            (c.0, c.1 - 1),
            (c.0, c.1 + 1),
            (c.0 - 1, c.1),
            (c.0 + 1, c.1)
        ],
        '.' => vec![],
        _ => todo!("{}", v),
    };
    return ns
        .iter()
        // remove out of bounds
        .filter(|&x| x.0 < n && x.1 < m && x.0 >= 0 && x.1 >= 0)
        // valid indices now
        .map(|&x| (x.0 as usize, x.1 as usize))
        .collect::<Vec<(usize, usize)>>();
}

fn get_neighbors(c: &(usize, usize), m: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let ns = get_neighbors_raw(c, m);
    let mut filtered_ns: Vec<(usize, usize)> = vec![];
    for z in ns.iter() {
        let rns = get_neighbors_raw(z, m);
        if rns.contains(c) {
            filtered_ns.push(*z);
        }
    }
    return filtered_ns;
}

fn bfs(mat: &Vec<Vec<char>>) -> Vec<Vec<i64>> {
    let mut q = VecDeque::new();

    // Find start
    let n = mat.len();
    let m = mat[0].len();
    let mut s = (0, 0);
    let mut dist: Vec<Vec<i64>> = vec![vec![-1; m]; n];
    for i in 0..n {
        for j in 0..m {
            if mat[i][j] == 'S' {
                s = (i, j);
            }
        }
    }
    dist[s.0][s.1] = 0;
    q.push_back(s);

    // BFS
    while !q.is_empty() {
        let x: (usize, usize) = q.pop_front().unwrap();
        let d = dist[x.0][x.1];
        let ns = get_neighbors(&x, &mat);
        for z in ns {
            // already visited
            if dist[z.0][z.1] != -1 { continue; }
            q.push_back(z);
            dist[z.0][z.1] = d + 1;
        }
    }
    return dist;
}

fn solve1(mat: &Vec<Vec<char>>) {
    let n = mat.len();
    let m = mat[0].len();
    let dist = bfs(mat);
    println!("dist");
    let mut max_d = 0;
    for i in 0..n {
        println!("{:?}", dist[i]);
        for j in 0..m {
            max_d = max_d.max(dist[i][j]);
        }
    }
    println!("max dist {}", max_d);
}


fn dfs(
    pt: (usize, usize),
    mat: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    polygon: &mut Vec<(usize, usize)>,
) {
    if visited[pt.0 as usize][pt.1 as usize] { return; }
    visited[pt.0 as usize][pt.1 as usize] = true;

    let ns = get_neighbors(&pt, &mat).iter().cloned().filter(|x| {
        return !visited[x.0][x.1];
    }).collect::<Vec<(usize, usize)>>();
    if ns.len() > 1 {
        panic!("Should only have one candidate! Got ");
    }
    if ns.len() == 1 {
        let prev = to_i32(polygon.last().unwrap());
        let pt32 = to_i32(&pt);
        let z = ns[0];
        let z32 = to_i32(&z);
        // ignore collinear points
        let vec1 = (z32.0 - prev.0 , z32.1 - prev.1);
        let vec2 = (pt32.0 - prev.0, pt32.1 - prev.1);
        if vec1.0 * vec2.1 - vec2.0 * vec1.1 != 0 {
            polygon.push(pt);
        }
        dfs((z.0, z.1), mat, visited, polygon);
    } else {
        polygon.push(pt);
    }
}

fn solve2(mat: &Vec<Vec<char>>) {
    let n = mat.len();
    let m = mat[0].len();
    // first, solve the maze to identify where the path
    // is. dist != -1
    let dist = bfs(mat);

    // Find the top left corner so we can orient ourselves
    // Also count how many vertices the path has - this is
    // used for pick's theorem
    let mut s = (0, 0);
    let mut found = false;
    let mut pathl = 0;
    for i in 0..n {
        for j in 0..m {
            if dist[i][j] != -1 {
                pathl += 1;
                if !found {
                    s = (i, j);
                    found = true;
                }
            }
        }
    }
    // There's nothing above or to the left of this point.
    // If it's on a pah, it must be at si + 1, sj
    // Do a DFS starting from the next point and mark s
    // as visited so we go counter-clockwise
    let mut visited: Vec<Vec<bool>> = vec![vec![false; m]; n];
    let mut polygon: Vec<(usize, usize)> = vec![];
    visited[s.0][s.1] = true;
    polygon.push(s);
    dfs((s.0 + 1, s.1), &mat, &mut visited, &mut polygon);

    // Area of polygon
    let mut area = 0;
    println!("poly {:?}", polygon);
    for i in 0..polygon.len() {
        let a = to_i32(&polygon[i]);
        let b = to_i32(&polygon[(i + 1) % polygon.len()]);
        let cross = a.0 * b.1 - b.0 * a.1;
        area += cross;
    }
    println!("outside {}", pathl);
    println!("area {}", area);
    // Pick's theorem
    let inside = (area - pathl) / 2 + 1;
    println!("inside {}", inside);
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let is_2nd = args.len() > 0 && args[0] == "2";

    let mut input_str = String::new();
    io::stdin().read_to_string(&mut input_str).expect("Failed to read input");
    let mut mat: Vec<Vec<char>> = vec![];
    for line in input_str.lines() {
        let row = line.chars().collect();
        mat.push(row);
    }

    if is_2nd {
        solve2(&mat);
    } else {
        solve1(&mat);
    }
}
