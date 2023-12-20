use std::io::{self, Read};
use std::env;
use std::collections::HashMap;
use std::collections::VecDeque;

fn get_list(s: &str) -> Vec<i64> {
    return s.split(' ')
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
}

fn solve_row(row: &mut VecDeque<i64>) {
    println!("{:?}", row);
    let mut is_all_zero: bool = true;
    for i in 0..row.len() {
        if row[i] != 0 {
            is_all_zero = false;
            break;
        }
    }
    if is_all_zero {
        row.push_front(0);
        row.push_back(0);
        return;
    }

    let mut diff: VecDeque<i64> = VecDeque::new();
    for i in 0..(row.len() - 1) {
        diff.push_back(row[i + 1] - row[i]);
    }
    solve_row(&mut diff);
    let first_v = row[0];
    let last_v = row[row.len() - 1];
    row.push_front(first_v - diff[0]);
    row.push_back(last_v + diff[diff.len() - 1]);
}


fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let is_2nd = args.len() > 0 && args[0] == "2";

    let mut input_str = String::new();
    io::stdin().read_to_string(&mut input_str).expect("Failed to read input");
    let lines = input_str.lines();

    let mut data: Vec<Vec<i64>> = vec![];
    for line in lines {
        let values = get_list(&line);
        data.push(values);
    }

    let mut total: i64 = 0;
    for row in data {
        let mut curr: VecDeque<i64> = VecDeque::new();
        for v in row {
            curr.push_back(v);
        }
        solve_row(&mut curr);
        println!("{:?}", curr);
        if is_2nd {
            total += curr[0];
        } else {
            total += curr[curr.len() - 1];
        }
    }

    println!("{}", total);

}
