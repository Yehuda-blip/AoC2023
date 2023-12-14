use crate::utils::DOUBLE_LINE_ENDING;

use anyhow::{Result, Context, anyhow};
use ndarray::Array2;

use super::day_13a::find_reflection_point;

pub fn solve(input: &String) -> Result<String> {
    let valleys: usize = itertools::process_results(
        input.split(DOUBLE_LINE_ENDING).map(|valley| {
            let rows = valley.lines().count();
            let cols = valley
                .lines()
                .next()
                .with_context(|| "no lines")?
                .chars()
                .count();
            let flat_chars = valley.lines().flat_map(|line| line.chars()).collect();
            let mat = Array2::from_shape_vec((rows, cols), flat_chars)
                .with_context(|| "could not build mat")?;
            if let Some(reflection_point) = find_reflection_point(&mat, true) {
                return Ok((reflection_point + 1) * 100);
            } else if let Some(reflection_point) = find_reflection_point(&mat.t().to_owned(), true) {
                return Ok(reflection_point + 1);
            } else {
                return Err(anyhow!("no reflection point found on any axis"));
            }
        }),
        |it| it.sum(),
    )?;
    Ok(valleys.to_string())
}