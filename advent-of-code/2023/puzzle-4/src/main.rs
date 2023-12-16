use std::env;
use std::io::{self, Read};
use std::collections::HashSet;

fn count_matches(e: &Vec<i64>, a: &Vec<i64>) -> i64 {
    let set_e: HashSet<_> = e.iter().cloned().collect();
    let set_a: HashSet<_> = a.iter().cloned().collect();

    let intersection: HashSet<_> = set_e.intersection(&set_a).cloned().collect();
    return intersection.len() as i64;
}

fn solve1(input: &Vec<(Vec<i64>, Vec<i64>)>) -> i64 {
    let mut total: i64 = 0;
    for (e, a) in input {
        let matches = count_matches(&e, &a);
        if matches > 0 {
            total += (2 as i64).pow((matches - 1) as u32);
        }
    }
    return total;
}

fn solve2(input: &Vec<(Vec<i64>, Vec<i64>)>) -> i64 {
    let mut scores: Vec<i64> = vec![1; input.len()];
    for i in 0..scores.len() {
        let (e, a) = &input[i];
        let matches = count_matches(&e, &a) as usize;
        for j in 0..matches {
            scores[j + i + 1] += scores[i];
        }
    }
    return scores.iter().sum();
}

fn get_list(s: &str) -> Vec<i64> {
    return s.split(' ')
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
}

fn main() {

    let args: Vec<String> = env::args().skip(1).collect();
    let is_2nd = args.len() > 0 && args[0] == "2";

    // read input
    let mut input_str = String::new();
    io::stdin().read_to_string(&mut input_str).expect("Failed to read input");
    let mut input: Vec<(Vec<i64>, Vec<i64>)> = Vec::new();
    for line in input_str.lines() {
        let split_result: Vec<&str> = line.split(':').collect();
        let numbers = split_result[1].trim();
        let parts: Vec<&str> = numbers.split('|').collect();
        let e: Vec<i64> = get_list(&parts[0]);
        let a: Vec<i64> = get_list(&parts[1]);
        input.push((e, a));
    }

    let total: i64;
    if is_2nd {
        total = solve2(&input);
    } else {
        total = solve1(&input);
    }

    println!("{}", total);
}
