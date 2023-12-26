use std::io::{self, Read};
use std::env;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::collections::HashMap;

// Assume already shifted
fn compute_load(mat: &mut Vec<Vec<char>>) -> i32 {
    let m = mat[0].len();
    let n = mat.len();
    let mut r = 0;
    for i in 0..n {
        for j in 0..m {
            if mat[i][j] == 'O' {
                r += (n - i) as i32;
            }
        }
    }
    return r;
}

fn shift(mat: &mut Vec<Vec<char>>) {
    let m = mat[0].len();
    let n = mat.len();
    let mut pos: Vec<usize> = vec![0; m];
    let mut score = 0;
    for i in 0..n {
        for j in 0..m {
            match mat[i][j] {
                'O' => {
                    mat[i][j] = '.';
                    mat[pos[j]][j] = 'O';
                    pos[j] += 1;
                },
                '#' => {
                    pos[j] = i + 1;
                },
                '.' => {},
                _ => panic!()
            };
        }
    }
}

fn cycle(mat: &mut Vec<Vec<char>>) {
    for i in 0..4 {
        shift(mat);
        rotate_cw(mat);
    }
}

fn calc_hash(mat: &mut Vec<Vec<char>>) -> u64 {
    let mut s: String = "".to_owned();
    let m = mat[0].len();
    let n = mat.len();
    for i in 0..n {
        let row: String = mat[i].clone().into_iter().collect();
        s += &row;
    }
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    return hasher.finish();
}

fn solve2(mat: &mut Vec<Vec<char>>) {
    for r in &mut mat.iter() {
        println!("{:?}", *r);
    }
    let N = 1000000000;
    let mut vis: HashMap<u64, usize> = HashMap::new();
    let mut offset: i32 = -1;
    let mut len: i32 = -1;
    let mut loads: Vec<i32> = vec![-1; 1000];
    for i in 0..1000 {
        let h = calc_hash(mat);
        loads[i] = compute_load(mat);
        println!("hash {}", h);
        match vis.get(&h) {
            Some(value) => {
                offset = *value as i32;
                len = (i - value) as i32;
                break;
            }
            None => {}
        }

        vis.insert(h, i);
        cycle(mat);
    }
    let p = ((N - offset) % len + offset) as usize;
    println!("cycle {}", len);
    println!("offset {}", offset);
    println!("load at p={} is {}", p, loads[p]);
}

fn rotate_cw(mat: &mut Vec<Vec<char>>) {
    let m = mat[0].len();
    let n = mat.len();
    let mut new_mat: Vec<Vec<char>> = vec![vec!['.'; m]; n];
    for i in 0..n {
        for j in 0..m {
            new_mat[j][n-1-i] = mat[i][j];
        }
    }
    *mat = new_mat;
}


fn solve1(mat: &Vec<Vec<char>>) {
    let m = mat[0].len();
    let n = mat.len();
    let mut pos: Vec<usize> = vec![0; m];
    let mut score = 0;
    for i in 0..n {
        for j in 0..m {
            match mat[i][j] {
                'O' => {
                    score += n - pos[j];
                    pos[j] += 1;
                },
                '#' => {
                    pos[j] = i + 1;
                },
                '.' => {},
                _ => panic!()
            };
        }
    }
    println!("{}", score);
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let is_2nd = args.len() > 0 && args[0] == "2";

    let mut input_str = String::new();
    io::stdin().read_to_string(&mut input_str).expect("Failed to read input");

    let mut mat: Vec<Vec<char>> = vec![];
    for line in input_str.lines() {
        mat.push(line.chars().collect());
    }

    if !is_2nd {
        solve1(&mat);
    } else {
        solve2(&mut mat);
    }
}
