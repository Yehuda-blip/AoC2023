use std::str::FromStr;
use anyhow::{Result, Error, anyhow};

use regex::Regex;

const GAME_NUM_CAPTURE: &str = r"Game +(?:(\d+))";

pub(super) enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    fn config(c: Color) -> i32 {
        match c {
            Color::Red => 12,
            Color::Green => 13,
            Color::Blue => 14,
        }
    }
}

pub(super) struct ParseColorError;
impl FromStr for Color {
    type Err = ParseColorError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "blue" => Ok(Color::Blue),
            _ => Err(ParseColorError),
        }
    }
}

pub(super) fn parse_color_pull(color_str: &str) -> Result<(i32, Color)> {
    let value_color: Vec<&str> = color_str
        .split(' ')
        .filter(|s| !s.trim().is_empty())
        .collect();
    if value_color.len() != 2 {
        return Err(anyhow!("bad color pull '{}'", color_str));
    };
    let (cube_num, color) = (
        i32::from_str(value_color[0])
            .map_err(|_e| anyhow!("could not parse '{}' as i32", value_color[0]))?,
        Color::from_str(value_color[1])
            .map_err(|_e| anyhow!("could not parse '{}' as Color", value_color[1]))?,
    );
    return Ok((cube_num, color));
}

pub fn solve(input: &String) -> Result<String> {
    let game_is_valid = |game_str: &str| {
        let color_pull_is_invalid = |(cube_num, color)| {
            Color::config(color) < cube_num
        };

        let pull_is_invalid = |pull_str: &str| {
            itertools::process_results(pull_str.split(',').map(parse_color_pull), |mut it| it.any(color_pull_is_invalid))
        };
        itertools::process_results(game_str.split(';').map(pull_is_invalid), |mut it| !it.any(|b| b))
    };

    let line_to_i32 = |line: &str| {
        let semicolon_pos = line.find(':').ok_or_else(|| anyhow!("bad format, no semicolon"))?;
        let game_num_capture = Regex::new(GAME_NUM_CAPTURE).unwrap();
        let caps = game_num_capture.captures(&line[..semicolon_pos]).ok_or_else(|| anyhow!("bad format, cannot capture game number"))?;
        let game_num = i32::from_str((caps.get(1).unwrap()).as_str()).map_err(|_| anyhow!("invalid game number"))?;

        if game_is_valid(&line[semicolon_pos + 1..])? {
            return Ok::<i32, Error>(game_num)
        }
        return Ok(0);
    };

    Ok(itertools::process_results(input.lines().map(line_to_i32), |it| it.sum::<i32>())?.to_string())
}


#[cfg(test)]
mod tests {
    use crate::day_2::day_2a::*;

    const TEST: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    #[test]
    fn test_solve() {
        assert_eq!(solve(&String::from(TEST)).unwrap(), "8")
    }
}