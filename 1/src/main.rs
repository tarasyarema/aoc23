use std::collections::HashMap;
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

    let word_map: HashMap<&str, i32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let mut nums: Vec<Vec<i32>> = Vec::new();
    let mut nums2: Vec<Vec<i32>> = Vec::new();

    for line in contents.trim().split("\n") {
        let mut line_vec: Vec<i32> = Vec::new();
        let mut line_vec2: Vec<i32> = Vec::new();

        for (i, c) in line.chars().enumerate() {
            match c.to_string().parse::<i32>() {
                Ok(some) => {
                    line_vec.push(some);
                    line_vec2.push(some);
                }
                Err(_err) => {
                    for (key, val) in word_map.iter() {
                        if line.chars().skip(i).collect::<String>().starts_with(key) {
                            line_vec2.push(*val);
                            break;
                        };
                    }
                }
            };
        }

        nums.push(line_vec);
        nums2.push(line_vec2);
    }

    let res = nums
        .iter()
        .map(|x| {
            if x.len() == 0 {
                0
            } else {
                x[0] * 10 + x[x.len() - 1]
            }
        })
        .sum::<i32>();

    let res2 = nums2
        .iter()
        .map(|x| {
            if x.len() == 0 {
                0
            } else {
                x[0] * 10 + x[x.len() - 1]
            }
        })
        .sum::<i32>();

    println!("#1 {res}");
    println!("#2 {res2}")
}
