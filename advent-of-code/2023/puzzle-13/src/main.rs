use std::io::{self, Read};
use std::env;

fn diff_cnt(va: &Vec<char>, vb: &Vec<char>) -> usize {
    let mut cnt = 0;
    for (a, b) in va.iter().zip(vb.iter()) {
        if a != b {
            cnt += 1;
        }
    }
    cnt
}

fn find_sym_vert(matrix: &Vec<Vec<char>>, t: usize) -> i32 {
    let n = matrix.len();
    let mut total_diff = 0;
    for i in 1..n {
        total_diff = diff_cnt(&matrix[i], &matrix[i - 1]);
        if total_diff > 1 {
            continue;
        }
        for k in 1..n {
            if i - 1 < k || i + k >= n {
                if total_diff == t {
                    return i as i32;
                } else {
                    break;
                }
            }
            let s = i - 1 - k;
            let e = i + k;
            total_diff += diff_cnt(&matrix[s], &matrix[e]);
            if total_diff > 2 {
                break;
            }
        }
    }
    return -1;
}

fn rotate_right(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let n = matrix.len();
    let m = matrix[0].len();
    let mut d = vec![vec!['a'; n]; m];
    for i in 0..m {
        for j in 0..n {
            d[i][j] = matrix[n - 1 - j][i];
        }
    }
    d
}

fn solve1_instance(m1: &Vec<Vec<char>>, t: usize) -> i32 {
    let m2 = rotate_right(m1);
    let mut tot = 0;
    // horizontal sym
    let r2 = find_sym_vert(&m2, t);
    if r2 >= 0 {
        tot += r2;
    }
    let r1 = find_sym_vert(m1, t);
    if r1 >= 0 {
        tot += 100*r1;
    }
    tot
}

fn solve(matrices: &Vec<Vec<Vec<char>>>, t: usize) {
    let mut total = 0;
    for matrix in matrices {
        let r = solve1_instance(&matrix, t);
        total += r;
    }
    println!("{}", total);
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let is_2nd = args.len() > 0 && args[0] == "2";

    let mut input_str = String::new();
    io::stdin().read_to_string(&mut input_str).expect("Failed to read input");

    let mut matrices: Vec<Vec<Vec<char>>> = vec![];
    let mut matrix: Vec<Vec<char>> = vec![];
    for line in input_str.lines() {

        if line.is_empty() {
            matrices.push(matrix);
            matrix = vec![];
            continue;
        }

        matrix.push(line.chars().collect());
    }
    matrices.push(matrix);

    if !is_2nd {
        solve(&matrices, 0);
    } else {
        solve(&matrices, 1);
    }
}
