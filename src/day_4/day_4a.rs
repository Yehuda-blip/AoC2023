use std::collections::HashSet;

use anyhow::{anyhow, Ok, Result};

pub fn solve(input: &String) -> Result<String> {
    let res: u32 = itertools::process_results(input.lines().map(|card| {
        let (targets, scratches) = parse_card(card)?;
        let match_count = count_matches(targets, scratches);
        let score = match match_count {
            0 => 0,
            _ => 1 << (match_count - 1)
        };
        Ok(score)
    }), |it| it.sum())?;
    return Ok(res.to_string());
}

pub(super) fn parse_card(card: &str) -> Result<(Vec<u32>, Vec<u32>)> {
    let (_prefix, card) = card
        .split_once(':')
        .ok_or(anyhow!("bad format, no semicolon in '{}'", card))?;
    let (targets, scratches) = card
        .split_once('|')
        .ok_or(anyhow!("bad format, no divisor char '|' in '{}'", card))?;
    let value_collector = |values_str: &str| {
        let values = itertools::process_results(
            values_str
                .split(' ')
                .filter(|split| !split.is_empty())
                .map(|val| {
                    val.parse()
                        .map_err(|_parse_err| anyhow!("could not parse '{}'", val))
                }),
            |it| it.collect(),
        )?;
        return Ok(values);
    };
    let (targets, scratches) = (value_collector(targets)?, value_collector(scratches)?);
    Ok((targets, scratches))
}

pub(super) fn count_matches(targets: Vec<u32>, scratches: Vec<u32>) -> u32 {
    let targets: HashSet<u32> = targets.into_iter().collect();
    scratches
        .into_iter()
        .filter(|val| targets.contains(val))
        .count() as u32
}
