use std::env;
use std::fs;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let mut file = "in".into();

    if args.len() > 1 {
        file = args.remove(1);
    }

    println!("Reading from {file}");

    let contents = fs::read_to_string(file).expect("Could not read data from {file}...");

    // Logic goes here

    let mut hands: Vec<(Vec<u64>, u64)> = Vec::new();

    for line in contents.lines() {
        let (hand, win_str) = line.trim().split_once(" ").unwrap();

        hands.push((
            hand.chars()
                .into_iter()
                .map(|c| match c {
                    'A' => 14,
                    'K' => 13,
                    'Q' => 12,
                    'J' => 11,
                    'T' => 10,
                    '9' => 9,
                    '8' => 8,
                    '7' => 7,
                    '6' => 6,
                    '5' => 5,
                    '4' => 4,
                    '3' => 3,
                    '2' => 2,
                    _ => panic!("Wrong char madafaka"),
                })
                .collect(),
            win_str.parse::<u64>().unwrap(),
        ));
    }

    dbg!(hands);
}
