use std::io::{self, Read};
use std::env;
use std::collections::HashMap;

struct Hand {
    cards: String,
    major: i64,
    minor: i64,
    bid: i64,
}

fn get_number(v: &Vec<usize>) -> i64 {
    return v
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse::<i64>()
        .expect("Should be numeric");
}

fn get_label_value(c: &char) -> i64 {
    match c {
        'A' => return 14,
        'K' => return 13,
        'Q' => return 12,
        'J' => return 11,
        'T' => return 10,
        '*' => return 1, // joker
        _ => return c.to_digit(10).unwrap() as i64
    }
}

fn get_minor_score(v: &Vec<char>) -> i64 {
    // base 15
    let mut score: i64 = 0;
    for c in v {
        let val = get_label_value(c);
        score = score * 15 + val;
    }
    return score;
}

fn get_major_score(hand: &Vec<char>) -> i64 {
    let mut freq: HashMap<&char, usize> = HashMap::new();

    let mut joker_cnt: usize = 0;
    for c in hand {
        // joker added later
        if *c == '*' {
            joker_cnt += 1;
            continue;
        }
        let cnt = *freq.entry(c).or_insert(0) + 1;
        freq.insert(c, cnt);
    }
    let mut freq_sorted = freq.values().map(|x| x.clone()).collect::<Vec<usize>>();

    while freq_sorted.len() < 5 {
        freq_sorted.push(0);
    }
    freq_sorted.sort_by(|a, b| b.cmp(a));

    // it's always worth adding jokers to the most frequent item
    freq_sorted[0] += joker_cnt;
    return get_number(&freq_sorted);
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let is_2nd = args.len() > 0 && args[0] == "2";

    // read input
    let mut input_str = String::new();
    io::stdin().read_to_string(&mut input_str).expect("Failed to read input");

    let mut hands: Vec<Hand> = vec![];
    for line in input_str.lines() {

        if let [card, bid_str] = line.trim().split(' ').collect::<Vec<&str>>()[..] {

            let bid = bid_str.parse::<i64>().unwrap();
            let mut cards = card.to_string();

            if is_2nd {
                cards = cards.replace("J", "*");
            }

            hands.push(Hand {
                cards: cards.clone(),
                major: get_major_score(&cards.chars().collect()),
                minor: get_minor_score(&cards.chars().collect()),
                bid: bid,
            });


        } else {
            panic!();
        }
    }
    hands.sort_by(|a, b| {
        if a.major != b.major {
            return a.major.cmp(&b.major);
        }
        return a.minor.cmp(&b.minor);
    });

    let mut total: i64 = 0;
    for i in 0..hands.len() {
        let h = &hands[i];
        println!("{} {} {}", h.cards, h.major, h.bid);
        total += h.bid * ((i + 1) as i64);
    }

    println!("answer: {}", total);
}
