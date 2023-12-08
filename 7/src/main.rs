use std::collections::HashMap;
use std::env;
use std::fs;

fn get_hand_type(hand: Vec<u64>) -> (Vec<u64>, u64) {
    let mut n_map: HashMap<u64, u64> = HashMap::new();
    let mut joks = 0;

    for n in hand.into_iter() {
        if n == 1 {
            joks += 1;
        }

        if let Some(m) = n_map.get_mut(&n) {
            *m = *m + 1;
        } else {
            n_map.insert(n, 1);
        }
    }

    let mut hand_type: Vec<u64> = n_map.values().map(|x| *x).collect();

    hand_type.sort();
    hand_type.reverse();

    (hand_type, joks)
}

fn get_hand_hash(hand_type: Vec<u64>) -> String {
    hand_type
        .clone()
        .into_iter()
        .fold("".to_string(), |s, n| s.to_owned() + &n.to_string())
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

    let mut points: HashMap<String, usize> = HashMap::new();

    let mut hands: HashMap<String, Vec<(Vec<u64>, u64)>> = HashMap::new();
    let mut hands2: HashMap<String, Vec<(Vec<u64>, u64)>> = HashMap::new();

    // XD
    let mut hashes = vec!["5", "41", "32", "311", "221", "2111", "11111"];

    for (i, hash) in hashes.clone().into_iter().enumerate() {
        points.insert(hash.to_string(), i);

        hands.insert(hash.to_string(), Vec::new());
        hands2.insert(hash.to_string(), Vec::new());
    }

    for line in contents.lines() {
        let (hand_str, bid_str) = line.trim().split_once(" ").unwrap();

        let hand: Vec<u64> = hand_str
            .chars()
            .into_iter()
            .map(|c| match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'T' => 10,
                '9' => 9,
                '8' => 8,
                '7' => 7,
                '6' => 6,
                '5' => 5,
                '4' => 4,
                '3' => 3,
                '2' => 2,
                'J' => 1,
                _ => panic!("Wrong char madafaka"),
            })
            .collect();

        let (hand_type, joks) = get_hand_type(hand.clone());
        let bid = bid_str.parse::<u64>().unwrap();

        let hand_type_hash = get_hand_hash(hand_type.clone());
        let curr_point = points.get(&hand_type_hash).unwrap();

        if let Some(xd) = hands.get_mut(&hand_type_hash) {
            (*xd).push((hand.clone(), bid));
        };

        if joks > 0 {
            let mut best_point = *curr_point;

            let mut new_hand: Vec<u64> = Vec::new();
            let mut new_hand_type: Vec<u64> = Vec::new();

            for v in hand.clone().into_iter() {
                if v == 1 {
                    continue;
                }

                let maybe_hand: Vec<u64> = hand
                    .clone()
                    .into_iter()
                    .map(|x| if x == 1 { v } else { x })
                    .collect();

                let (maybe_hand_type, _) = get_hand_type(maybe_hand.clone());
                let maybe_hash = get_hand_hash(maybe_hand_type.clone());

                let curr_point = *(points.get(&maybe_hash).unwrap());

                if curr_point <= best_point {
                    new_hand = maybe_hand.clone();
                    new_hand_type = maybe_hand_type.clone();

                    best_point = curr_point;
                }
            }

            let new_hand_hash = get_hand_hash(new_hand_type.clone());

            if let Some(xd) = hands2.get_mut(&new_hand_hash) {
                (*xd).push((new_hand.clone(), bid));
            };
        } else {
            if let Some(xd) = hands2.get_mut(&hand_type_hash) {
                (*xd).push((hand.clone(), bid));
            };
        }
    }

    hashes.reverse();

    fn get_points(hashes: Vec<&str>, mut handilla: HashMap<String, Vec<(Vec<u64>, u64)>>) -> u64 {
        let mut sum: u64 = 0;
        let mut ix = 1;

        for hash in hashes.clone().into_iter() {
            if let Some(curr) = handilla.get_mut(hash) {
                (*curr).sort();

                for (hand, bid) in (*curr).clone().into_iter() {
                    // println!("{} - {:?} - {} * {}", hash, hand, ix, bid);
                    sum += ix * bid;
                    ix += 1;
                }
            }
        }

        sum
    }

    println!("1. {}", get_points(hashes.clone(), hands));
    println!("2. {}", get_points(hashes, hands2));
}
