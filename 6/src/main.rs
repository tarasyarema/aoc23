use std::env;
use std::fs;
use std::iter;
use std::usize;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let mut file = "in".into();

    if args.len() > 1 {
        file = args.remove(1);
    }

    for case in 0..2 {
        let curr_file = format!("{}{}", file.clone(), if case == 0 { "" } else { "2" });
        println!("Reading from {curr_file}");

        let contents =
            fs::read_to_string(curr_file).expect("Could not read data from {curr_file}...");

        // Logic goes here

        let mut lines = contents.lines();

        let time_str: Vec<u64> = lines
            .next()
            .unwrap()
            // XD, the following I'm not proud of, tbh
            .split_once("Time:")
            .unwrap()
            .1
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect();

        let distance_str: Vec<u64> = lines
            .next()
            .unwrap()
            .split_once("Distance:")
            .unwrap()
            .1
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect();

        let races: Vec<(u64, u64)> = time_str.into_iter().zip(distance_str.into_iter()).collect();

        let res = races.into_iter().fold(1, |total_wins, (t, d)| {
            let curr_win =
                iter::repeat(0)
                    .take(t as usize)
                    .enumerate()
                    .fold(0, |curr_wins, (_t_i, _)| {
                        let t_i: u64 = _t_i.try_into().unwrap_or(0);

                        if t_i > 0 && t_i + d / t_i < t {
                            curr_wins + 1
                        } else {
                            curr_wins
                        }
                    });

            total_wins * curr_win
        });

        println!("{}. {}", case + 1, res)
    }
}
