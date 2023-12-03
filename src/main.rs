use std::fs;
use anyhow::Result;

pub mod utils;
pub mod day_1 {
    pub mod day_1a;
    pub mod day_1b;
}
pub mod day_2 {
    pub mod day_2a;
    pub mod day_2b;
}

const SOLVER: fn(&String) -> Result<String> = day_1::day_1b::solve;
const INPUT_PATH: &str = "input.txt";

fn main() {
    let input =
        fs::read_to_string(INPUT_PATH).expect(&format!("Could not read input file {}", INPUT_PATH));
    print!("{:?}", SOLVER(&input))
}
