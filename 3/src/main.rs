use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;

fn distance([x1, x2]: [usize; 2], [y1, y2]: [usize; 2]) -> bool {
    if (x1 as i32 - y1 as i32).abs() > 1 || (x2 as i32 - y2 as i32).abs() > 1 {
        return false;
    }

    true
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let mut file = "in".into();

    if args.len() > 1 {
        file = args.remove(1);
    }

    println!("Reading from {file}");

    let contents = fs::read_to_string(file).expect("Could not read data from {file}...");

    // Logic goes here

    let mut nums: Vec<([usize; 3], i32)> = Vec::new();
    let mut gears: HashMap<[usize; 2], Vec<i32>> = HashMap::new();

    for (i, line) in contents.trim().lines().enumerate() {
        let mut maybe_num = "".to_owned();

        for (j, c) in line.chars().enumerate() {
            match c.is_numeric() {
                true => {
                    maybe_num.push_str(c.to_string().as_str());
                }
                false => {
                    if maybe_num.len() > 0 {
                        let num = maybe_num.parse::<i32>().expect("could not parse number");

                        nums.push(([i + 1, j, maybe_num.len()], num));
                        maybe_num = "".to_owned();
                    }

                    if c == '*' {
                        gears.insert([i + 1, j + 1], Vec::new());
                    }

                    if c != '.' {
                        nums.push(([i + 1, j + 1, 1], -1));
                    }
                }
            }
        }

        // Edge case when the number ends in the end
        if maybe_num.len() > 0 {
            let num = maybe_num.parse::<i32>().expect("could not parse number");
            nums.push(([i + 1, line.len(), maybe_num.len()], num));
        }
    }

    let mut num_set: HashSet<(usize, usize, i32)> = HashSet::new();

    for (i, ([x1, x2, xl], xn)) in nums.iter().enumerate() {
        for (j, ([y1, y2, yl], yn)) in nums.iter().enumerate() {
            if i == j {
                continue;
            }

            let mut found = false;

            if *yn == -1 && *xn != -1 {
                for k in 0..*xl {
                    let dist = distance([*x1, *x2 - k], [*y1, *y2]);

                    // println!(
                    //     "{} & {} -> dist([{}, {}], [{}, {}]) = {}",
                    //     *xn,
                    //     *yn,
                    //     *x1,
                    //     *x2 - k,
                    //     *y1,
                    //     *y2,
                    //     dist
                    // );

                    if dist {
                        found = true;
                        num_set.insert((*x1, *x2, *xn));
                    }
                }

                if found {
                    if let Some(v) = gears.get_mut(&[*y1, *y2]) {
                        v.push(*xn)
                    }
                }
            }

            if *xn == -1 && *yn != -1 {
                for k in 0..*yl {
                    let dist = distance([*y1, *y2 - k], [*x1, *x2]);
                    // println!(
                    //     "{} & {} -> dist([{}, {}], [{}, {}]) = {}",
                    //     *xn,
                    //     *yn,
                    //     *y1,
                    //     *y2 - k,
                    //     *x1,
                    //     *x2,
                    //     dist
                    // );

                    if dist {
                        num_set.insert((*y1, *y2, *yn));
                    }
                }

                if found {
                    if let Some(v) = gears.get_mut(&[*x1, *x2]) {
                        v.push(*yn)
                    }
                }
            }
        }
    }

    let res1: i32 = num_set.into_iter().map(|(_, _, x)| x).sum();
    let res2: i32 = gears.values().fold(0, |acc, v| {
        if v.len() == 2 {
            acc + v.iter().fold(1, |v_acc, x| v_acc * x)
        } else {
            acc
        }
    });

    println!("1. {res1}");
    println!("2. {res2}");
}
