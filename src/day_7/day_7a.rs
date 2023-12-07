use std::str::FromStr;

use anyhow::Result;
use itertools::Itertools;

use super::camel_hand::CamelHand;

pub fn solve(input: &String) -> Result<String> {
    let res = itertools::process_results(input.lines().map(|line| CamelHand::from_str(line)), |it| it.sorted().enumerate().fold(0, |acc, (rank, hand)| {
        acc + (rank as u32 + 1) * hand.get_bid()
    }))?;
    Ok(res.to_string())
}