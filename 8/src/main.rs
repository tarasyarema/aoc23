use std::collections::HashMap;
use std::env;
use std::fs;
use std::time::Instant;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let mut file = "in".into();

    if args.len() > 1 {
        file = args.remove(1);
    }

    println!("Reading from {file}");

    let contents = fs::read_to_string(file).expect("Could not read data from {file}...");

    // Logic goes here

    let mut seq: Vec<bool> = Vec::new();
    let mut graph: HashMap<&str, (&str, &str)> = HashMap::new();

    let mut roots2: Vec<&str> = Vec::new();

    for (i, line) in contents.lines().enumerate() {
        if line.len() == 0 {
            continue;
        }

        if i == 0 {
            for c in line.trim().chars() {
                if c == 'L' {
                    seq.push(true);
                } else if c == 'R' {
                    seq.push(false);
                }
            }

            continue;
        }

        let (root, nodes) = line.trim().split_once(" = ").unwrap();

        let (l, r) = nodes
            .trim()
            .trim_start_matches('(')
            .trim_end_matches(')')
            .split_once(", ")
            .unwrap();

        graph.insert(root, (l, r));

        if root.ends_with("A") {
            roots2.push(root);
        }
    }

    let mut res1 = 0;
    let mut root = "AAA";

    let start = Instant::now();

    loop {
        for s in seq.clone().into_iter() {
            res1 += 1;

            if s {
                // left
                root = graph.get(root).unwrap().0;
            } else {
                // right
                root = graph.get(root).unwrap().1;
            }

            if root == "ZZZ" {
                break;
            }
        }

        if root == "ZZZ" {
            break;
        }

        if res1 > 100000 {
            panic!("Too many iterations")
        }
    }

    println!("1. {res1} in {:?}", start.elapsed());

    let mut res2: u64 = 0;

    let start2 = Instant::now();
    let r_len = roots2.len();

    loop {
        let mut end = false;

        for s in seq.clone().into_iter() {
            let mut ok = 0;
            res2 += 1;

            roots2 = roots2
                .into_iter()
                .map(|root| {
                    let new_root = if s {
                        // left
                        graph.get(root).unwrap().0
                    } else {
                        // right
                        graph.get(root).unwrap().1
                    };

                    if new_root.ends_with("Z") {
                        ok += 1;
                    }

                    new_root
                })
                .collect();

            if res2 % 100_000_000 == 0 {
                println!(
                    "{} M iterations in elpased {:?}",
                    (res2 as f64) / 1_000_000.0,
                    start2.elapsed()
                );
            }

            if ok == r_len {
                end = true;
                break;
            }
        }

        if end {
            break;
        }
    }

    println!("2. {} in {:?}", res2, start2.elapsed());
}
