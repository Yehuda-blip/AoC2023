use anyhow::{anyhow, Result, Context, Error};
const EPSILON: f64 = 0.000001;

#[inline]
pub(super) fn solve_for_hold_times(time: f64, acceleration: f64, distance: f64) -> Result<(f64, f64)> {
    let distance = distance + EPSILON;
    let plusminus = ((acceleration * time).powf(2.0) - (4.0 * acceleration * distance)).sqrt();
    if plusminus.is_nan() || acceleration == 0.0 {
        return Err(anyhow!("time: {}, acceleration: {}, distance: {} are unsolvable for hold times", time, acceleration, distance))
    }
    let denominator = 2.0 * -acceleration;
    let (const_comp, plusminus_comp) = ((-acceleration * time) / denominator, plusminus / denominator);
    let (hold_min, hold_max) = match plusminus_comp > 0.0 {
        true => (const_comp - plusminus_comp, const_comp + plusminus_comp),
        false => (const_comp + plusminus_comp, const_comp - plusminus_comp)
    };
    Ok((hold_min, hold_max))
}

pub fn solve(input: &String) -> Result<String> {
    let (times, distances) = input.split_once("\n").context("could not split to times and distances")?;
    let result = itertools::process_results(times.split_whitespace().zip(distances.split_whitespace()).skip(1).map(|(time_str, dist_str)| {
        let (time, dist) = (time_str.parse()?, dist_str.parse()?);
        let (hold_1, hold_2) = solve_for_hold_times(time, 1.0, dist)?;
        Ok::<f64, Error>(hold_2.floor() - hold_1.ceil() + 1.0)
    }), |it| it.fold(1.0, |acc, val| acc * val))?;
    Ok(result.to_string())
}