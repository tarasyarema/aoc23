use std::cmp::min;
use std::env;
use std::fs;
use std::slice::Chunks;
use std::time::Instant;
use tokio::task::JoinSet;

#[tokio::main]
async fn main() {
    let mut args: Vec<String> = env::args().collect();
    let mut file = "in".into();

    if args.len() > 1 {
        file = args.remove(1);
    }

    println!("Reading from {file}");

    let contents = fs::read_to_string(file).expect("Could not read data from {file}...");

    // Logic goes here
    let mut seeds: Vec<u64> = Vec::new();
    let mut maps: Vec<Vec<Vec<u64>>> = Vec::new();

    let mut curr_map: Vec<Vec<u64>> = Vec::new();

    for line in contents.lines() {
        let trimmed = line.trim();

        if trimmed.len() == 0 {
            continue;
        }

        if trimmed.clone().contains("seeds:") {
            seeds.extend(
                trimmed
                    .split_once("seeds: ")
                    .unwrap()
                    .1
                    .trim()
                    .split_whitespace()
                    .map(|x| x.parse::<u64>().unwrap()),
            );
        } else if trimmed.clone().contains("map:") {
            if curr_map.len() == 0 {
                continue;
            }

            maps.push(curr_map.clone());
            curr_map.clear();
        } else {
            curr_map.push(
                trimmed
                    .trim()
                    .split_whitespace()
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect(),
            );
        }
    }

    if curr_map.len() > 0 {
        maps.push(curr_map.clone());
    }

    fn get_location_from_maps(seed: u64, maps: &Vec<Vec<Vec<u64>>>) -> u64 {
        let mut location: u64 = seed;

        for map in maps.clone().into_iter() {
            let mut tmp_location = location;

            for row in map.clone() {
                if row.len() != 3 {
                    panic!("row is messed up as len is not 3")
                }

                let a = row[0];
                let b = row[1];
                let d = row[2];

                if location > b + (d - 1) || location < b {
                    continue;
                }

                tmp_location = a + (location - b);
            }

            location = tmp_location;
        }

        location
    }

    fn get_locations(seeds: &Vec<u64>, maps: &Vec<Vec<Vec<u64>>>) -> Vec<u64> {
        seeds
            .into_iter()
            .map(|seed| get_location_from_maps(*seed, maps))
            .collect()
    }

    let min_location = get_locations(&seeds, &maps).into_iter().min().unwrap();
    println!("1. {}", min_location);

    let mut children: JoinSet<u64> = JoinSet::new();
    let fork_start = Instant::now();

    for (i, seed) in seeds.clone().chunks(2).enumerate() {
        let x = seed[0];
        let y = seed[1];

        let r_chunks = (x..(x + y - 1)).collect::<Vec<u64>>();
        let chunks = r_chunks.chunks(if y > 1000 {
            (y / 1000) as usize
        } else {
            y as usize
        });

        let l = chunks.len();

        for (j, c) in chunks.enumerate() {
            let moved_maps = maps.clone();
            let chunk = Vec::from(c);

            children.spawn(async move {
                let first = chunk[0];
                let last = chunk[chunk.len() - 1];

                let start = Instant::now();

                let v = chunk.into_iter().fold(last, |prev_min, seed| {
                    min(prev_min, get_location_from_maps(seed, &moved_maps))
                });

                let duration = start.elapsed();

                println!(
                    "seed {} task {} / {} finished for ({}, {}, {}) with value {} in {:?}",
                    i + 1,
                    j + 1,
                    l,
                    first,
                    last,
                    last - first,
                    v,
                    duration
                );

                v
            });
        }
    }

    let mut min_location_2 = 0;

    println!("Executing {} tasks", children.len());

    while let Some(res) = children.join_next().await {
        let loc = res.unwrap();

        if min_location_2 == 0 {
            min_location_2 = loc;
        } else {
            min_location_2 = min(min_location_2, loc);
        }
    }

    println!("2. {} in {:?}", min_location_2, fork_start.elapsed());
}
