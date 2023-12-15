use anyhow::{anyhow, Context, Result};
use ndarray::Array2;

pub fn calculate_pressure(rocks_map: &Array2<char>) -> Result<usize> {
    println!("{:?}", rocks_map);
    itertools::process_results(
        rocks_map
            .rows()
            .into_iter()
            .map(|row| {
                row.iter().enumerate().try_fold(
                    (0, 0, 0),
                    |(last_cube, rounded_on_cube, load), (i, rock)| match rock {
                        '.' => Ok((last_cube, rounded_on_cube, load)),
                        '#' => Ok((i + 1, 0, load)),
                        'O' => Ok((
                            last_cube,
                            rounded_on_cube + 1,
                            load + row.len() - (last_cube + rounded_on_cube),
                        )),
                        _ => Err(anyhow!("what is this doing on my map?")),
                    },
                )
            })
            .map(|folding| {
                println!("{:?}", folding);
                match folding {
                    Ok((_last_cube, _rounded_on_cube, load)) => Ok(load),
                    Err(e) => Err(e),
                }
            }),
        |it| it.sum(),
    )
}

pub fn solve(input: &String) -> Result<String> {
    let rows = input.lines().count();
    let cols = input
        .lines()
        .next()
        .with_context(|| "no lines")?
        .chars()
        .count();
    let flat_chars = input.lines().flat_map(|line| line.chars()).collect();
    let mat =
        Array2::from_shape_vec((rows, cols), flat_chars).with_context(|| "could not build mat")?;
    let result = calculate_pressure(&mat.t().to_owned())?;
    return Ok(result.to_string());
}
