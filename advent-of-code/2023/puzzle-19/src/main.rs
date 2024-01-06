use std::env;
use std::io::{self, Read};
use std::collections::HashMap;


struct Cond {
    pos: usize,
    gt: bool,
    v: i64,
    y: String,
}

fn eval(
    item: &Vec<i64>,
    workflow: &str,
    rule_map: &HashMap<String, (Vec<Cond>, String)>
) -> bool {
    if workflow == "A" {
        return true;
    }
    if workflow == "R" {
        return false;
    }

    println!("workflow {}", workflow);

    let rule = rule_map.get(workflow).unwrap();
    for cond in &rule.0 {
        let actual = item[cond.pos];
        if cond.gt && actual > cond.v {
            return eval(item, &cond.y, rule_map);
        } else if !cond.gt && actual < cond.v {
            return eval(item, &cond.y, rule_map);
        }
    }
    // no matching, go to the default
    return eval(item, &rule.1, rule_map);
}

fn count(ints: &mut Vec<(i64, i64)>) -> i64 {
    let mut t = 1;
    for i in 0..ints.len() {
        t *= ints[i].1 - ints[i].0 + 1;
    }
    return t;
}

fn traverse(
    state: &str,
    position: usize,
    rule_map: &HashMap<String, (Vec<Cond>, String)>,
    ints: &mut Vec<(i64, i64)>
) -> i64 {
    println!("traversing {} {}", state, position);
    if state == "A" {
        println!("{:?}", ints);
        return count(ints);
    }

    if state == "R" {
        return 0;
    }

    let rule = rule_map.get(state).unwrap();
    let conds = &rule.0;
    let mut r = 0;
    if position >= conds.len() {
        r += traverse(&rule.1, 0, rule_map, ints);
    } else {
        let cond = &conds[position];
        let interval = ints[cond.pos];
        let pass_int: (i64, i64);
        let fail_int: (i64, i64);
        if cond.gt {
            pass_int = (cond.v + 1, interval.1);
            fail_int = (interval.0, cond.v);
        } else {
            pass_int = (interval.0, cond.v - 1);
            fail_int = (cond.v, interval.1);
        }
        ints[cond.pos] = pass_int;
        // pass
        r += traverse(&cond.y, 0, rule_map, ints);

        // fail
        ints[cond.pos] = fail_int;
        r += traverse(&state, position + 1, rule_map, ints);

        // restore
        ints[cond.pos] = interval;
    }
    r
}

fn solve2(rule_map: &HashMap<String, (Vec<Cond>, String)>) {
    // x m a s
    let mut ints: Vec<(i64, i64)> = vec![(1, 4000); 4];
    let cnt = traverse("in", 0, rule_map, &mut ints);
    println!("cnt {}", cnt);
}

fn solve1(rule_map: &HashMap<String, (Vec<Cond>, String)>, input: &Vec<Vec<i64>>) {
    let mut t = 0;
    for vs in input {
        println!("{:?}", vs);
        if eval(&vs, "in", &rule_map) {
            println!("A");
            t += vs.iter().sum::<i64>();
        } else {
            println!("R");
        }
    }
    println!("total: {}", t);
}

fn main() {
    let mut input_str = String::new();
    io::stdin().read_to_string(&mut input_str).expect("Failed to read input");
    let mut lines = input_str.lines();

    // read edges
    let mut rule_map: HashMap<String, (Vec<Cond>, String)> = HashMap::new();
    let lookup = vec!["x", "m", "a", "s"];
    loop {
        let line = lines.next().unwrap();
        if line == "" { break; }
        let parts = line.split('{').collect::<Vec<&str>>();

        let rule_name = parts[0].to_owned();
        let cond_str = parts[1].get(0..(parts[1].len()-1)).unwrap();
        let conds_str = cond_str.split(',');
        let mut conds = vec![];
        let mut leaf = "";
        for cond_str in conds_str {
            if cond_str.contains(":") {
                let temp = cond_str.split(':').collect::<Vec<&str>>();
                let pred = temp[0];
                let y = temp[1].to_owned();

                if pred.contains("<") {
                    let temp = pred.split('<').collect::<Vec<&str>>();
                    let var = temp[0];
                    let v = temp[1].parse::<i64>().unwrap();
                    let index = lookup.iter().position(|&x| x == var).unwrap();

                    let c = Cond {pos: index, gt: false, v, y: y.clone()};
                    conds.push(c);
                    println!("{} < {} -> {}", index, v, y);
                } else if pred.contains(">") {
                    let temp = pred.split('>').collect::<Vec<&str>>();
                    let var = temp[0];
                    let v = temp[1].parse::<i64>().unwrap();
                    let index = lookup.iter().position(|&x| x == var).unwrap();

                    let c = Cond {pos: index, gt: true, v, y: y.clone()};
                    conds.push(c);
                    println!("{} > {} -> {}", index, v, y);
                }

            } else {
                leaf = cond_str;
            }

        }
        rule_map.insert(rule_name, (conds, leaf.to_owned()));
    }

    // read input
    let mut input: Vec<Vec<i64>> = vec![];
    while let Some(line) = lines.next() {
        let parts = line.trim_matches(&['{', '}'][..]).split(',').collect::<Vec<&str>>();
        let mut vs: Vec<i64> = vec![];
        for part in parts {
            let eqs = part.split('=').collect::<Vec<&str>>();
            vs.push(eqs[1].parse::<i64>().unwrap());
        }
        input.push(vs);
    }

    let args: Vec<String> = env::args().skip(1).collect();
    let is_2nd = args.len() > 0 && args[0] == "2";
    if !is_2nd {
        solve1(&rule_map, &input);
    } else {
        solve2(&rule_map);
    }
}
