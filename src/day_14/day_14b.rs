use std::collections::HashMap;

use anyhow::{Context, Result};
use ndarray::Array2;

pub fn calculate_pressure(rocks_map: &Array2<char>) -> Result<usize> {
    // println!("{:?}", rocks_map);
    itertools::process_results(
        rocks_map.rows().into_iter().map(|row| {
            row.iter()
                .enumerate()
                .try_fold(0, |acc, (i, rock)| match rock {
                    'O' => Ok(acc + row.len() - i),
                    _ => Ok(acc),
                })
        }),
        |it| it.sum(),
    )
}

#[inline]
fn cycle(rocks_map: &Array2<char>) -> Array2<char> {
    let mut after_rot = rocks_map.clone();
    let rows = after_rot.dim().0;
    let cols = after_rot.dim().1;

    // println!("{:?}\n\n", after_rot);
    for j in 0..cols {
        let mut indexer = 0;
        for i in 0..rows {
            let val = after_rot[(i, j)];
            match val {
                '#' => indexer = i + 1,
                'O' => {
                    after_rot[(i, j)] = '.';
                    after_rot[(indexer, j)] = 'O';
                    indexer += 1;
                }
                _ => {}
            }
        }
    }

    // println!("{:?}\n\n", after_rot);
    for i in 0..rows {
        let mut indexer = 0;
        for j in 0..cols {
            let val = after_rot[(i, j)];
            match val {
                '#' => indexer = j + 1,
                'O' => {
                    after_rot[(i, j)] = '.';
                    after_rot[(i, indexer)] = 'O';
                    indexer += 1;
                }
                _ => {}
            }
        }
    }

    // println!("{:?}\n\n", after_rot);
    for j in 0..cols {
        let mut indexer = rows - 1;
        for i in (0..rows).rev() {
            let val = after_rot[(i, j)];
            match val {
                '#' if i > 0 => indexer = i - 1,
                'O' => {
                    after_rot[(i, j)] = '.';
                    after_rot[(indexer, j)] = 'O';
                    if i > 0 {
                        indexer -= 1
                    };
                }
                _ => {}
            }
        }
    }

    // println!("{:?}\n\n", after_rot);
    for i in 0..rows {
        let mut indexer = cols - 1;
        for j in (0..cols).rev() {
            let val = after_rot[(i, j)];
            match val {
                '#' if j > 0 => indexer = j - 1,
                'O' => {
                    after_rot[(i, j)] = '.';
                    after_rot[(i, indexer)] = 'O';
                    if j > 0 {
                        indexer -= 1
                    };
                }
                _ => {}
            }
        }
    }

    // println!("{:?}", after_rot);
    return after_rot;
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
    let mut mat =
        Array2::from_shape_vec((rows, cols), flat_chars).with_context(|| "could not build mat")?;
    let mut cache = HashMap::new();
    for i in 1..1_000 {
        let v = cycle(&mat);
        // println!("{} pressure",calculate_pressure(&v.to_owned())?);
        mat = v.clone();
        if let Some(first_occurence) = cache.get(&v) {
            let period = i - first_occurence;
            let wtf = 1_000_000_000 - i;
            let (remaining, _wtf2) = (wtf % period, wtf / period);
            for _ in 0..remaining {
                mat = cycle(&mat);
            }
            break;
        }
        cache.insert(v, i);
    }
    // println!("{:?}\n\n", mat);
    let res = calculate_pressure(&mat.t().to_owned())?;
    Ok(res.to_string())
}
