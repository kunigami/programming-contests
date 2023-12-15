use std::env;
use std::io::{self, Read};
use std::collections::HashMap;

fn main() {

    let args: Vec<String> = env::args().skip(1).collect();
    let is_2nd = args.len() > 0 && args[0] == "2";
    let max_values = HashMap::from([
        ("blue", 14),
        ("green", 13),
        ("red", 12),
    ]);

    let mut digits = Vec::new();
    for i in 0..10 {
        digits.push(i.to_string());
    }

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");

    let mut total: i32 = 0;
    let mut total_2: i32 = 0;
    let mut game_id: i32 = 1;
    for line in input.lines() {
        // [game n, rest]
        let split_result: Vec<&str> = line.split(':').collect();
        let rest = split_result[1].trim();
        let rounds: Vec<&str> = rest.split(';').collect();
        let mut is_valid = true;

        let mut curr_values = HashMap::from([
            ("blue", 0),
            ("green", 0),
            ("red", 0),
        ]);

        for round in &rounds {
            let outcomes: Vec<&str> = round.trim().split(',').collect();
            for outcome in &outcomes {
                let parts: Vec<&str> = outcome.trim().split(' ').collect();
                let count = parts[0].trim().parse::<i32>().expect("Failed to parse the string as i32");
                let color = parts[1].trim();
                match max_values.get(color) {
                    Some(max_value) => {
                        if count > *max_value {
                            is_valid = false;
                        }
                    },
                    None => panic!("invalid input")
                }

                if let Some(curr_value) = curr_values.get_mut(color) {
                    if count > *curr_value {
                        *curr_value = count;
                    }
                }
            }
        }

        if is_valid {
            total += game_id;
        }

        let mut prod: i32 = 1;
        for value in curr_values.values() {
            prod *= value;
        }
        total_2 += prod;
        game_id += 1;
    }
    if is_2nd {
        println!("Total is {}", total_2);
    } else {
        println!("Total is {}", total);
    }
}
