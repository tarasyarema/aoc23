use std::cmp::max;
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

    let mut _games: Vec<[u8; 3]> = Vec::new();

    let contraints1: [u8; 3] = [12, 13, 14];

    let mut res1: i32 = 0;
    let mut res2: i32 = 0;

    for (i, line) in contents.trim().lines().enumerate() {
        let game_moves: Vec<&str> = line
            .trim()
            .split_once(": ")
            .expect("line has wrong format")
            .1
            .split(";")
            .collect();

        let cubillos = game_moves
            .into_iter()
            .map(|game_move| {
                let trimmed: [u8; 3] = game_move
                    .trim()
                    .split(",")
                    .into_iter()
                    .map(|x| {
                        let (num_str, color) = x
                            .trim()
                            .split_once(" ")
                            .expect("fucked up game move detail");

                        let mut base: [u8; 3] = [0, 0, 0];
                        let num = num_str.to_string().parse::<u8>().expect("bad number");

                        // Followin RGB in terms of order
                        let ix = match color {
                            "red" => 0,
                            "green" => 1,
                            "blue" => 2,
                            _ => panic!("Bad color"),
                        };

                        base[ix] = num;

                        base
                    })
                    .fold([0, 0, 0] as [u8; 3], |acc, x| {
                        let mut base: [u8; 3] = [0, 0, 0];

                        for i in 0..3 {
                            base[i] = acc[i] + x[i];
                        }

                        base
                    });

                trimmed
            })
            .collect::<Vec<[u8; 3]>>();

        match cubillos.iter().fold(true, |acc, x| {
            if acc {
                x.into_iter().enumerate().all(|(j, y)| y <= &contraints1[j])
            } else {
                acc
            }
        }) {
            true => res1 += (i as i32) + 1,
            false => (),
        }

        let power = cubillos
            .iter()
            .fold([0, 0, 0] as [u8; 3], |acc, [x, y, z]| {
                [max(acc[0], *x), max(acc[1], *y), max(acc[2], *z)]
            });

        res2 += power.into_iter().fold(1, |acc, p| acc * (p as i32))
    }

    println!("1. {res1}");
    println!("2. {res2}")
}
