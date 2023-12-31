use anyhow::Result;
use std::{fs, time};

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
pub mod day_8 {
    // did not solve part 2
    pub mod day_8a;
}
pub mod day_9 {
    pub mod day_9a;
    pub mod day_9b;
}
pub mod day_10 {
    pub mod day_10a;
    pub mod day_10b;
}
pub mod day_11 {
    pub mod day_11a;
    pub mod day_11b;
}
pub mod day_12 {
    pub mod day_12a;
    pub mod day_12a_brute_force;
    pub mod day_12b;
}
pub mod day_13 {
    pub mod day_13a;
    pub mod day_13b;
}
pub mod day_14 {
    pub mod day_14a;
    pub mod day_14b;
}
pub mod day_15 {
    pub mod day_15a;
    pub mod day_15b;
}
pub mod day_16 {
    pub mod day_16a;
    pub mod day_16b;
}
pub mod day_17 {
    pub mod day_17a;
    pub mod day_17b;
}

const SOLVER: fn(&String) -> Result<String> = day_17::day_17b::solve;
const INPUT_PATH: &str = "input.txt";

fn main() {
    let input =
        fs::read_to_string(INPUT_PATH).expect(&format!("Could not read input file {}", INPUT_PATH));
    let my_time = time::Instant::now();
    print!("got {:?} in {:?}", SOLVER(&input), my_time.elapsed())
}
