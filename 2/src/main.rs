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

    println!("{contents}")
}
