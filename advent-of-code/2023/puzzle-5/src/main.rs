use std::io::{self, Read};
use std::env;
use core::cmp::min;

fn get_list(s: &str) -> Vec<i64> {
    return s.split(' ')
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
}

fn read_list(lines: &mut std::str::Lines) -> Vec<Vec<i64>> {
    lines.next(); // header

    let mut res: Vec<Vec<i64>> = vec![];
    loop {
        let line_maybe = lines.next();
        if line_maybe.is_none() { break; }

        let line = line_maybe.unwrap();
        if line.is_empty() { break; }

        let row = get_list(line);
        let dst = row[0];
        let src = row[1];
        let len = row[2];
        let new_row = vec![src, dst, len];


        res.push(new_row);
    }

    res.sort_by(|a, b| a[0].cmp(&b[0]));
    return res;
}

fn apply_map(src: i64, map: &Vec<Vec<i64>>) -> i64 {
    for i in 0..map.len() {
        let start = map[i][0];
        let end = start + map[i][2];
        if src < start {
            return src;
        }
        // in the range of row i - 1
        if src < end  {
            let offset = src - start;
            return map[i][1] + offset;
        }
    }
    return src;
}

fn solve1(seeds: Vec<i64>, maps: Vec<Vec<Vec<i64>>>) -> i64 {
    let mut min_seed: i64 = i64::MAX;
    for seed in seeds.iter() {
        let mut curr_seed: i64 = *seed;
        for map in maps.iter() {
            curr_seed = apply_map(curr_seed, &map);
        }
        min_seed = min(min_seed, curr_seed);
    }
    return min_seed;
}

// Point representing one of the endpoints of an interval
#[derive(PartialEq)]
struct Point {
    // x's coordinate
    x: i64,
    // start or end
    is_start: bool,
    // is from map?
    is_map: bool,
    // how much to shift the endpoints
    // ignored for non start points?
    shift: i64,
}

fn solve2(seeds: Vec<i64>, maps: Vec<Vec<Vec<i64>>>) -> i64 {
    // construct starting interval
    assert!(seeds.len() %2 == 0, "should contain pairs");
    let mut ints: Vec<(i64, i64)> = vec![];
    for i in (0..seeds.len()).step_by(2) {
        let s = seeds[i];
        let e = s + seeds[i+1];
        ints.push((s, e));
    }

    println!("intervals: ");
    for i in ints.iter() {
        println!("{} {}", i.0, i.1);
    }

    for map in maps.iter() {
        let mut pts: Vec<Point> = vec![];
        // construct pts from intervals
        for (s, e) in ints.iter() {
            pts.push(Point { x: *s, is_start: true, is_map: false, shift: 0 });
            pts.push(Point { x: *e, is_start: false, is_map: false, shift: 0 });
        }

        // merge with the endpoints for this map
        println!("will merge into: ");
        for row in map {
            let start = row[0];
            let end = start + row[2];
            // [start, end] maps to [start + shift, end + shift]
            let shift = row[1] - row[0];
            println!("{} {}", start, end);
            pts.push(Point { x: start, is_start: true, is_map: true, shift: shift });
            pts.push(Point { x: end, is_start: false, is_map: true, shift: 0 });
        }

        // sort by x, break ties by putting endpoints first.
        pts.sort_by(|a, b| {
            if a.x != b.x {
                return a.x.cmp(&b.x);
            } else {
                // end points come before start points
                return (if a.is_start { 1 } else { 0 }).cmp(&(if b.is_start { 1 } else { 0 }))
            }
        });

        let mut is_seed = false;
        let mut shift = 0;
        let mut prev: Option<&Point> = None;
        let mut new_ints: Vec<(i64, i64)> = vec![];

        println!("split intervals: ");
        for i in 0..pts.len() {
            let pt = &pts[i];
            // 3. ignore empty intervals
            match prev {
                Some(ref prev_v) => {
                    if is_seed && prev_v.x != pt.x {
                        println!("{} {} -> {} {}", prev_v.x, pt.x, prev_v.x + shift, pt.x + shift);
                        new_ints.push((prev_v.x + shift, pt.x + shift));
                    }
                }
                None => ()
            }
            prev = Some(pt);

            if !pt.is_map { // seed interval
                if pt.is_start {
                    is_seed = true;
                } else { // end
                    is_seed = false;
                }
            } else {
                if pt.is_start {
                    shift = pt.shift;
                } else { // end
                    shift = 0;
                }
            }
        }
        ints = new_ints;
        println!("intervals: ");
        for i in ints.iter() {
            println!("{} {}", i.0, i.1);
        }
    }

    let mut min_v: i64 = i64::MAX;
    for i in ints.iter() {
        min_v = min(min_v, i.0);
    }

    return min_v;
}


fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let is_2nd = args.len() > 0 && args[0] == "2";

    // read input
    let mut input_str = String::new();
    io::stdin().read_to_string(&mut input_str).expect("Failed to read input");

    let mut lines = input_str.lines();
    let line = lines.next().unwrap();
    let parts: Vec<&str> = line.split(':').collect();
    let seeds = get_list(parts[1]);

    let mut maps: Vec<Vec<Vec<i64>>> = vec![];
    lines.next(); // empty line
    let n = 7;
    for _i in 0..n {
        maps.push(read_list(&mut lines));
    }

    let r: i64;
    if is_2nd {
        r = solve2(seeds, maps);
    } else {
        r = solve1(seeds, maps);
    }
    println!("answer: {}", r);
}
