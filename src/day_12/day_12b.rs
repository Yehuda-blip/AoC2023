use std::collections::HashSet;

use anyhow::{Result, Context};

use crate::day_12::day_12a::options;


pub fn solve(input: &String) -> Result<String> {
    let res: u64 = itertools::process_results(input.lines().map(|line| {
        let (record_str, sequence_str) = line.split_once(' ').context("no space between record and sequences")?;
        println!("{}\n{}", record_str, sequence_str);
        let mut record = record_str.chars().chain(['?'].into_iter()).cycle().take((record_str.len() + 1) * 5 - 1).collect();
        let sequences = sequence_str.split(',').map(|num| num.parse().with_context(|| "bad parse")).collect::<Result<Vec<usize>, anyhow::Error>>()?;
        let len = sequences.len();
        let mut sequences = sequences.into_iter().cycle().take(len * 5).collect();
        println!("{:?}\n{:?}", record, sequences);
        let res = options(&mut record, &mut sequences);
        // println!("{}", res);
        return Ok::<u64, anyhow::Error>(res)
    }), |it| it.sum())?;
    Ok(res.to_string())
}