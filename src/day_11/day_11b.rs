use anyhow::Result;

use super::day_11a::solve_with_expansion;

const EXPANSION: usize = 1000000;

pub fn solve(input: &String) -> Result<String> {
    solve_with_expansion(input, EXPANSION)
}