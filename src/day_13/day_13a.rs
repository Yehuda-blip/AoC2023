use anyhow::{anyhow, Context, Ok, Result};
use ndarray::Array2;

use crate::utils::DOUBLE_LINE_ENDING;

fn reflects_over(valley: &Array2<char>, i: usize, j: usize, still_smudged: bool) -> Result<(bool, bool)> {
    if i >= j || j >= valley.dim().0 {
        return Err(anyhow!("bad indexing in reflection"));
    };

    let row_i = valley.row(i);
    let row_j = valley.row(j);
    let mismatched = row_i
        .iter()
        .zip(row_j.iter())
        .filter(|(c1, c2)| {
            c1 != c2
        })
        .count();
    match mismatched {
        0 => Ok((true, still_smudged)),
        1 => Ok((still_smudged, false)),
        _ => Ok((false, false))
    }
}

pub(super) fn find_reflection_point(valley: &Array2<char>, smudging: bool) -> Option<usize> {
    (0..valley.dim().0 - 1 as usize).find(|i| {
        let mut is_reflection;
        let mut still_smudged = smudging;
        let mut j = 0;
        while (*i as isize - j as isize) >= 0 && i + j + 1 < valley.dim().0 {
            (is_reflection, still_smudged) = reflects_over(valley, i - j, i + j + 1, still_smudged).expect("you fed bad indices. this was your responsibilty and you fucked it. disappointment...");
            if !is_reflection {return false};
            j += 1;
        }
        return true && !still_smudged;
    })
}

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
            if let Some(reflection_point) = find_reflection_point(&mat, false) {
               return Ok((reflection_point + 1) * 100);
            } else if let Some(reflection_point) = find_reflection_point(&mat.t().to_owned(), false) {
                return Ok(reflection_point + 1);
            } else {
                return Err(anyhow!("no reflection point found on any axis"));
            }
        }),
        |it| it.sum(),
    )?;
    Ok(valleys.to_string())
}
