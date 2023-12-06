use std::env;
use std::fs;
use std::iter;

fn extract_nums(s: &str) -> Vec<i32> {
    let mut v: Vec<i32> = Vec::new();

    for n_str in s.split_whitespace() {
        match n_str.parse::<i32>() {
            Ok(n) => v.push(n),
            Err(e) => panic!("{e}"),
        }
    }

    v
}

fn points(l: usize) -> i32 {
    if l == 0 {
        0
    } else if l == 1 {
        1
    } else {
        2 * points(l - 1)
    }
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

    let mut cards: Vec<(Vec<i32>, Vec<i32>, Vec<i32>)> = Vec::new();

    for line in contents.lines() {
        let (_, raw) = line.split_once(": ").expect("First split failed");
        let (left, right) = raw.trim().split_once("|").expect("Second split failed");

        let win = extract_nums(left);
        let got = extract_nums(right);

        cards.push((
            win.clone(),
            got.clone(),
            got.into_iter().fold(Vec::new(), |mut acc, x| {
                if win.contains(&x) {
                    acc.push(x);
                    acc
                } else {
                    acc
                }
            }),
        ));
    }

    let res1: Vec<(i32, i32)> = cards
        .clone()
        .into_iter()
        .map(|(_, _, x)| (x.len() as i32, points(x.len())))
        .collect();

    println!(
        "1. {}",
        res1.clone().into_iter().fold(0, |acc, (_, x)| acc + x)
    );

    let mut res2 = Vec::from_iter(iter::repeat(1).take(res1.len()));

    for (i, (x, _)) in res1.clone().into_iter().enumerate() {
        for j in 0..(x as usize) {
            if i + j + 1 > res2.len() {
                continue;
            }

            res2[i + j + 1] += res2[i];
        }
    }

    println!("2. {}", res2.into_iter().sum::<i32>());
}
