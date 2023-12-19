use anyhow::{Result, Context, anyhow};

use super::day_17a::min_cost;

pub fn solve(input: &String) -> Result<String> {
    let city_map = input
        .lines()
        .map(|line| {
            itertools::process_results(
                line.chars()
                    .map(|c| Ok(c.to_digit(10).context(anyhow!("not a digit"))?)),
                |it| it.collect(),
            )
        })
        .collect::<Result<Vec<Vec<u32>>>>()?;

    let val = min_cost::<4, 10>(&city_map);
    Ok(val.to_string())
}
