use std::io::{self, Read};
use std::env;

fn fits(cs: &Vec<char>, cnts: &Vec<i64>) -> bool {
    let mut actual: Vec<i64> = vec![];
    let mut acc = 0;
    for c in cs {
        if *c == '#' {
            acc += 1;
        } else {
            if acc > 0 {
                actual.push(acc);
            }
            acc = 0;
        }
    }
    if acc > 0 {
        actual.push(acc);
    }
    // println!("{:?} {:?}", actual, cnts);

    if actual.len() != cnts.len() {
        return false;
    }

    for (a, b) in actual.iter().zip(cnts.iter()) {
        if a != b {
            return false;
        }
    }
    true
}

fn brutal(i: usize, cs: &mut Vec<char>, cnts: &Vec<i64>) -> i64 {

    if i == cs.len() {
        if fits(cs, cnts) {
            // println!("{:?} *", cs);
            return 1;
        }
        // println!("{:?}", cs);
        return 0;
    }

    let mut r = 0;
    if cs[i] == '#' || cs[i] == '.' {
        return brutal(i+1, cs, cnts);
    } else {
        cs[i] = '.';
        r += brutal(i+1, cs, cnts);

        cs[i] = '#';
        r += brutal(i+1, cs, cnts);

        cs[i] = '?';
        return r;
    }
}

fn solve1(css: &Vec<Vec<char>>, cntss: &Vec<Vec<i64>>) {
    let mut r = 0;
    for i in 0..css.len() {
        let mut cs = css[i].clone();
        let cnt = brutal(0, &mut cs, &cntss[i]);
        println!("r = {}", cnt);
        r += cnt;
    }
    println!("{}", r);
}

fn solve2_instance(cs: &Vec<char>, cnts: &Vec<i64>) -> i64 {
    let n = cs.len();
    let m = cnts.len();

    // dp[i][j][k] = # of valid configurations of subarray
    // csk[0..i], satisfying cntk[0..j] and whether ending
    // with a # (1 vs. 0)
    //
    let mut dp: Vec<Vec<Vec<i64>>> = vec![vec![vec![0; 2]; m + 1]; n + 1];
    dp[0][0][0] = 1;
    dp[0][0][1] = 1;
    for i in 0..n {
        if cs[i] == '#' {
            break;
        }
        dp[i + 1][0][0] = 1;
    }

    for i in 0..n {
        for j in 0..m {
            let l = cnts[j] as usize; // length of block j

            if cs[i] != '.' && i + 1 >= l {
                let s = i + 1 - l;
                // extend from the previous j if the block is valid
                let mut is_valid = true;

                for k in s..i {
                    if cs[k] == '.' {
                        is_valid = false;
                        break;
                    }
                }
                // println!("s = {}, i = {}, j = {}, val = {}", s, i, j, is_valid);
                if is_valid {
                    dp[i + 1][j + 1][1] = dp[s][j][0];
                }
                // println!("dp[{}][{}][1] = {}", i+1, j+1, dp[i + 1][j + 1][1]);
            }
            if cs[i] != '#' {
                // either extending empty space ~. -> ~..
                // or breaking off a block ~# -> ~#.
                dp[i + 1][j + 1][0] = dp[i][j + 1][0] + dp[i][j + 1][1];
            }
        }
    }

    // ends = 0
    if false {
        println!("ends = 0");
        for i in 0..(n+1) {
            for j in 0..(m+1) {
                for k in 0..2 {
                    print!("i={}, j={}, k={} ==> ", i, j, k);
                    println!("{}, ", dp[i][j][k]);
                }
            }
            println!("==");
        }
    }

    dp[n][m][0] + dp[n][m][1]
}

fn solve2(css: &Vec<Vec<char>>, cntss: &Vec<Vec<i64>>) {
    let mut tot = 0;

    for i in 0..css.len() {
        // extend input
        let mut csk: Vec<char> = vec![];
        let mut cntk: Vec<i64> = vec![];
        for k in 0..5 {
            if k > 0 {
                csk.push('?');
            }
            for c in &css[i] {
                csk.push(*c);
            }

            for cnt in &cntss[i] {
                cntk.push(*cnt);
            }
        }

        let r = solve2_instance(&csk, &cntk);
        tot += r;
        println!("r = {}", r);
    }
    println!("tot = {}", tot);
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let is_2nd = args.len() > 0 && args[0] == "2";

    let mut input_str = String::new();
    io::stdin().read_to_string(&mut input_str).expect("Failed to read input");

    let mut css: Vec<Vec<char>> = vec![];
    let mut cntss: Vec<Vec<i64>> = vec![];
    for line in input_str.lines() {
        let parts: Vec<_> = line.split(' ').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
        let chars: Vec<char> = parts[0].chars().collect();
        let nums: Vec<i64> = parts[1].split(',').map(|n| n.parse::<i64>().unwrap()).collect();
        css.push(chars);
        cntss.push(nums);
    }
    if !is_2nd {
        solve1(&css, &cntss);
    } else {
        solve2(&css, &cntss);
    }
}
