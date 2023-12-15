use std::env;
use std::io::{self, Read};

fn main() {

    let args: Vec<String> = env::args().skip(1).collect();
    let is_2nd = args.len() > 0 && args[0] == "2";

    let names = vec![
        "zero",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
    ];

    let mut digits = Vec::new();
    for i in 0..10 {
        digits.push(i.to_string());
    }

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");

    let mut total: i32 = 0;
    for line in input.lines() {
        let mut first_dig: i32 = 0;
        let mut last_dig: i32 = 0;
        let mut min_index: i32 = 123456789;
        let mut max_index: i32 = -1;

        for i in 1..10 {

            let mut update_min = |index: usize| {
                let index = index as i32;
                if index < min_index {
                    min_index = index as i32;
                    first_dig = i as i32;
                }
            };
            let mut update_max = |index: usize| {
                let index = index as i32;
                if index > max_index {
                    max_index = index as i32;
                    last_dig = i as i32;
                }
            };

            if is_2nd {
                let name = names[i];
                match line.find(name) {
                    Some(index) => update_min(index),
                    None => {},
                }
                match line.rfind(name) {
                    Some(index) => update_max(index),
                    None => {},
                }
            }

            let digit = digits[i].clone();
            match line.find(&digit) {
                Some(index) => update_min(index),
                None => {},
            }
            match line.rfind(&digit) {
                Some(index) => update_max(index),
                None => {},
            }
        }

        total += first_dig * 10 + last_dig;
    }
    println!("Total is {}", total);
}
