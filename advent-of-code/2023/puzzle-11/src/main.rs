use std::io::{self, Read};
use std::env;

fn solve(mat: &Vec<Vec<char>>, padding: i64) {
    // Find columns and rows without galaxies
    // Gather coordinates of galexies
    let mut rows: Vec<usize> = vec![];
    let n = mat.len();
    let m = mat[0].len();
    let mut coords: Vec<(usize, usize)> = vec![];

    for i in 0..n {
        let mut has_galaxy = false;
        for j in 0..m {
            if mat[i][j] == '#' {
                has_galaxy = true;
                coords.push((i, j));
            }
        }
        if !has_galaxy {
            rows.push(i);
        }
    }

    let mut cols: Vec<usize> = vec![];
    let mut tot_dist = 0;
    for j in 0..m {
        let mut has_galaxy = false;
        for i in 0..n {
            if mat[i][j] == '#' {
                has_galaxy = true;
            }
        }
        if !has_galaxy {
            cols.push(j);
        }
    }
    println!("rows = {:?}", rows);
    println!("cols = {:?}", cols);

    // compute all distances
    let mut tot_dist = 0;
    for i in 0..coords.len() {
        let ga = &coords[i];
        for j in (i+1)..coords.len() {
            let gb = &coords[j];
            let y0 = ga.0.min(gb.0);
            let y1 = ga.0.max(gb.0);
            let x0 = ga.1.min(gb.1);
            let x1 = ga.1.max(gb.1);

            let mut dist: i64 = (y1 - y0 + x1 - x0) as i64;
            for row in &rows {
                if y0 < *row && *row < y1 {
                    dist += padding;
                }
            }
            for col in &cols {
                if x0 < *col && *col < x1 {
                    dist += padding;
                }
            }
            tot_dist += dist;
        }
    }
    println!("{}", tot_dist);
}

fn main() {
    println!("Hello, world!");

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
        solve(&mat, 1000000 - 1);
    } else {
        solve(&mat, 1);
    }

}
