use anyhow::Result;

pub fn solve(input: &String) -> Result<String> {
    let (mut time, mut dist) = (0 as i64, 0 as i64);
    let parse_advance = |c: char, acc: &mut i64| if c.is_digit(10) {*acc = *acc * 10 + c.to_digit(10).expect("literally just checked yhis explicitly") as i64};
    input.lines().zip(vec![&mut time, &mut dist]).for_each(|(line, p)| {
        line.chars().for_each(|c| parse_advance(c, p))
    });
    let (hold1, hold2) = super::day_6a::solve_for_hold_times(time as f64, 1.0, dist as f64)?;
    Ok((hold2.floor() - hold1.ceil() + 1.0).to_string())
}