use std::io::{self, Read};
use std::env;

fn get_list(s: &str) -> Vec<i64> {
    return s.split(' ')
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
}

fn get_number(v: &Vec<i64>) -> i64 {
    return v
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse::<i64>()
        .expect("Should be numeric");
}

fn solve1(times: Vec<i64>, distances: Vec<i64>) -> i64 {
    let mut prod: i64 = 1;
    for i in 0..times.len() {
        let t = times[i];
        let d = distances[i];
        let mut cnt: i64 = 0;
        for s in 0..(t+1) {
            let d2: i64 = s * (t - s);
            if d2 > d {
                cnt += 1;
            }
        }

        if cnt > 0 {
            prod *= cnt;
        }
    }
    return prod;
}

/*
    Idea: distance as a function of s:

    f(s) = st - s^2

    which is a parabola. if we find solve for f(s) = d,
    we find s0 and s1, s0 <= s1. based on the shape of this function
    we can assume f(s0) >= d and f(s1) <= d.

    then the number of solutions is s1 - s0 + 1
*/
fn solve2(times: Vec<i64>, distances: Vec<i64>) -> i64 {
    let t = get_number(&times) as f64;
    let d = get_number(&distances) as f64;
    println!("t={}, d={}", t, d);

    let delta = (t.powf(2.0) - 4.0*d).sqrt();
    let s0 = ((t - delta)/2.0) as i64 + 1;
    println!("{}", s0);

    let s1 = ((t + delta)/2.0) as i64;
    println!("{}", s1);

    return s1 - s0 + 1;
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let is_2nd = args.len() > 0 && args[0] == "2";

    let mut input_str = String::new();
    io::stdin().read_to_string(&mut input_str).expect("Failed to read input");
    let mut lines = input_str.lines();

    let parts: Vec<&str> = lines.next().unwrap().split(':').collect();
    let times = get_list(parts[1]);

    let parts: Vec<&str> = lines.next().unwrap().split(':').collect();
    let distances = get_list(parts[1]);

    let r: i64;
    if is_2nd {
        r = solve2(times, distances);
    } else {
        r = solve1(times, distances);
    }
    println!("answer: {}", r);
}
