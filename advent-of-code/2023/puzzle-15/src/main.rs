use std::io::{self, Read};
use std::env;
use std::collections::HashMap;


fn calc_hash(s: &str) -> i64 {
    let mut h: i64 = 0;
    for c in s.chars() {
        h += c as i64;
        h = (h * 17) % 256;
    }
    return h
}

struct Lens {
    label: String,
    focal: i64,
}


fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let is_2nd = args.len() > 0 && args[0] == "2";

    let mut input_str = String::new();
    io::stdin().read_to_string(&mut input_str).expect("Failed to read input");

    let mut lines = input_str.lines();
    let instrs = lines.next().unwrap().split(',');
    let mut t = 0;

    if !is_2nd {
        for part in instrs {
            t += calc_hash(part);
        }
        println!("{}", t);
    } else {

        let mut boxes: HashMap<i64, Vec<Lens>> = HashMap::new();

        for inst in instrs {
            if inst.contains('=') {
                let parts = inst.split('=').clone().collect::<Vec<&str>>();
                let box_id = calc_hash(parts[0]);
                let focal = parts[1].parse::<i64>().unwrap();
                let label = parts[0].to_owned();

                let boxe: &mut Vec<Lens> = boxes.entry(box_id).or_insert(vec![]);

                match boxes.get_mut(&box_id) {
                    Some(boxe) => {
                        let mut found = false;
                        for i in 0..boxe.len() {
                            if boxe[i].label == label {
                                boxe[i].focal = focal;
                                found = true;
                                break;
                            }
                        }
                        if !found {
                            boxe.push(Lens {
                                label: label,
                                focal: focal,
                            });
                        }
                    },
                    None => {
                        boxes.insert(box_id, vec![Lens {
                            label: label,
                            focal: focal,
                        }]);
                    }
                }

            } else {
                let label = inst.split('-').nth(0).unwrap();
                let box_id = calc_hash(label);
                if let Some(boxe) = boxes.get_mut(&box_id) {
                    for i in 0..boxe.len() {
                        let lens = &mut boxe[i];
                        if lens.label == label {
                            boxe.remove(i);
                            break;
                        }
                    }
                }
            }
        }

        let mut t = 0;
        for k in boxes.keys() {
            let boxe = boxes.get(k).unwrap();
            if boxe.len() == 0 {
                continue;
            }
            for i in 0..boxe.len() {
                let s = (k + 1) * ((i + 1) as i64) * boxe[i].focal;
                t += s;
            }
        }
        println!("{}", t);
    }
}
