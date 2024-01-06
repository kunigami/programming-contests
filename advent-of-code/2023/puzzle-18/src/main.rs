use std::io::{self, Read};
use std::env;
use std::collections::HashSet;
use std::collections::HashMap;

fn calc_area(polygon: &Vec<(i64, i64)>) -> i64 {
    let mut area = 0;
    for i in 0..polygon.len() {
        let a = polygon[i];
        let b = polygon[(i + 1) % polygon.len()];
        let cross = a.0 * b.1 - b.0 * a.1;
        area += cross;
    }
    return area;
}

fn solve2(input: &Vec<(String, i64, String)>) {
    let mut input2: Vec<(String, i64, String)> = vec![];

    let dirs = vec!["R","D","L","U"];
    for (dir_, len_, color) in input.iter() {
        let len = i64::from_str_radix(color.get(1..6).unwrap(), 16).unwrap();
        let chars = color.chars().collect::<Vec<_>>();
        let c: char = *chars.last().unwrap();
        let di = (c as usize) - ('0' as usize);
        let dir = dirs[di].to_owned();
        input2.push((dir, len, color.clone()));
    }
    solve1(&input2);
}

fn solve1(input: &Vec<(String, i64, String)>) {
    let mut polygon = vec![(0, 0)];
    let deltas: HashMap<String, (i64, i64)> = HashMap::from([
        ("R".to_owned(), (0, 1)),
        ("L".to_owned(), (0, -1)),
        ("U".to_owned(), (-1, 0)),
        ("D".to_owned(), (1, 0)),
    ]);
    let mut p = (0, 0);
    let mut pts_cnt = 0;
    for (dir, len, color) in input.iter() {
        let next_p = (
            p.0 + deltas[dir].0 * len,
            p.1 + deltas[dir].1 * len
        );
        pts_cnt += len;
        polygon.push(next_p);
        p = next_p;
    }
    let area = -calc_area(&polygon);
    println!("Area: {}", area);
    let inside = (area - pts_cnt) / 2 + 1;
    println!("Inside: {}", inside);
    println!("Total: {}", inside + pts_cnt);
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let is_2nd = args.len() > 0 && args[0] == "2";

    let mut input_str = String::new();
    io::stdin().read_to_string(&mut input_str).expect("Failed to read input");
    let lines = input_str.lines();
    let mut mat: Vec<Vec<char>> = vec![];
    let mut input: Vec<(String, i64, String)> = vec![];
    for line in lines {
        let parts = line.split(' ').collect::<Vec<&str>>();
        let dir = parts[0];
        let len = parts[1];
        let color = parts[2].trim_matches(&['(', ')'][..]);
        input.push((
            dir.to_owned(),
            len.parse::<i64>().unwrap(),
            color.to_owned()
        ));
    }

    if !is_2nd {
        solve1(&input);
    } else {
        solve2(&input);
    }
}
