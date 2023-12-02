use std::cmp::max;

use super::day_2a::{Color, parse_color_pull};

pub fn solve(input: &String) -> Result<String, String> {
    let max_group = |game_str: &str| {
        let update_max = |(mut max_r, mut max_g, mut max_b), (cube_num, color)| {
            match color {
                Color::Red => {max_r = max(max_r, cube_num)},
                Color::Green => {max_g = max(max_g, cube_num)},
                Color::Blue => {max_b = max(max_b, cube_num)}
            }
            (max_r, max_g, max_b)
        };
        return itertools::process_results(game_str.split([';', ',']).map(parse_color_pull), |it| it.fold((0,0,0), update_max));
    };

    let min_power = |line: &str| {
        let semicolon_pos = line.find(':').ok_or("bad format, no semicolon")?;
        let (max_r, max_g, max_b) = max_group(&line[semicolon_pos+1..])?;
        return Ok::<i32, String>(max_r * max_b * max_g);
    };

    let power_group_sum: i32 = itertools::process_results(input.lines().map(min_power), |it| it.sum())?;
    Ok(power_group_sum.to_string())
}