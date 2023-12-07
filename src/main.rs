use std::{fs, time};
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
pub mod day_3 {
    pub mod day_3a;
    pub mod day_3b;
}
pub mod day_4 {
    pub mod day_4a;
    pub mod day_4b;
}
pub mod day_5 {
    pub mod day_5a;
    pub mod day_5b;
}
pub mod day_6 {
    pub mod day_6a;
    pub mod day_6b;
}
pub mod day_7 {
    mod camel_hand;
    pub mod day_7a;
    pub mod day_7b;
}

const SOLVER: fn(&String) -> Result<String> = day_7::day_7b::solve;
const INPUT_PATH: &str = "input.txt";

fn main() {
    let input =
        fs::read_to_string(INPUT_PATH).expect(&format!("Could not read input file {}", INPUT_PATH));
    let my_time = time::Instant::now();
    print!("got {:?} in {:?}", SOLVER(&input), my_time.elapsed())
}
