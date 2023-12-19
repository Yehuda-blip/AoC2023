use std::collections::HashSet;

use anyhow::{anyhow, Result};
use itertools::Itertools;

use super::day_16a::{step, Direction};

pub fn solve(input: &String) -> Result<String> {
    let platform = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let it = (0..platform.len())
        .map(|i| (Direction::East, (i, 0)))
        .chain((0..platform.len()).map(|i| (Direction::West, (i, platform[0].len() - 1))))
        .chain((0..platform[0].len()).map(|j| (Direction::South, (0, j))))
        .chain((0..platform[0].len()).map(|j| (Direction::North, (platform.len() - 1, j))));
    let m = it.map(|(dir, loc)| {
        let mut s = HashSet::new();
        step(dir, loc, &mut s, &platform);
        let energized = s.into_iter().map(|(_, loc)| loc).collect::<HashSet::<(usize, usize)>>();
        return energized.len();
    }).max();
    match m {
        Some(ener) => Ok(ener.to_string()),
        None => Err(anyhow!("BAD"))
    }
}
