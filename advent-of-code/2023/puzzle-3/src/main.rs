use std::env;
use std::io::{self, Read};
use std::collections::HashSet;

fn get_neighbors(i: i32, j: i32, mat: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut idx: Vec<(usize, usize)> = Vec::new();
    for di in -1..2 {
        for dj in -1..2 {
            if di == 0 && dj == 0 {
                continue;
            }

            let ii = i + di;
            let jj = j + dj;
            let clen = mat[i as usize].len();
            if ii < 0 || ii >= mat.len() as i32 || jj < 0 || jj >= clen as i32 {
                continue;
            }
            idx.push((ii as usize, jj as usize));
        }
    }
    return idx;
}

fn solve1(mat: &Vec<Vec<char>>) {
    let mut total: i32 = 0;
    let mut has_any_symbol = false;
    let mut numb: i32 = 0;

    for i in 0..mat.len() {
        for j in 0..mat[i].len() {
            let v = mat[i][j];
            if v.is_ascii_digit() {
                for (ii, jj) in get_neighbors(i as i32, j as i32, &mat) {
                    let u = mat[ii][jj];
                    if u != '.' && !u.is_ascii_digit() {
                        has_any_symbol = true;
                    }
                }
                numb = numb * 10 + (v.to_digit(10).unwrap() as i32);
            } else { // ended digit
                if has_any_symbol {
                    total += numb;
                }
                numb = 0;
                has_any_symbol = false;
            }
        }
        // ended digit
        if has_any_symbol {
            total += numb;
        }
        numb = 0;
        has_any_symbol = false;
    }
    println!("{}", total);
}

fn solve2(mat: &Vec<Vec<char>>) {
    let mut total: i32 = 0;

    let mut idx: Vec<Vec<i32>> = Vec::new();
    for i in 0..mat.len() {
        let row: Vec<i32> = vec![-1; mat[i].len()];
        idx.push(row);
    }

    // map where the numbers are
    let mut numb: i32 = 0;
    let mut numbs: Vec<i32> = Vec::new();
    for i in 0..mat.len() {
        for j in 0..mat[i].len() {
            let v = mat[i][j];
            if v.is_ascii_digit() {
                numb = numb * 10 + (v.to_digit(10).unwrap() as i32);
                idx[i][j] = numbs.len() as i32;
            } else { // ended digit
                numbs.push(numb);
                numb = 0;
            }
        }
        // ended digit
        numbs.push(numb);
        numb = 0;
    }

    for i in 0..mat.len() {
        for j in 0..mat[i].len() {
            let v = mat[i][j];
            if v != '*' {
                continue;
            }

            let mut set = HashSet::new();
            for (ii, jj) in get_neighbors(i as i32, j as i32, &mat) {
                if idx[ii][jj] >= 0 {
                    set.insert(idx[ii][jj] as usize);
                }
            }
            let unique_idx: Vec<usize> = set.into_iter().collect();

            if unique_idx.len() == 2 {
                total += numbs[unique_idx[0]] * numbs[unique_idx[1]];
            }
        }
    }

    println!("{}", total);
}

fn main() {

    let args: Vec<String> = env::args().skip(1).collect();
    let is_2nd = args.len() > 0 && args[0] == "2";

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");

    let lines: Vec<&str> = input.lines().collect();

    let mut mat: Vec<Vec<char>> = Vec::new();
    for i in 0..lines.len() {
        mat.push(lines[i].chars().collect::<Vec<char>>());
    }

    if is_2nd {
        solve2(&mat);
    }  else {
        solve1(&mat);
    }
}
