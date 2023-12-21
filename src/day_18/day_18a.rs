use anyhow::{anyhow, Context, Result};
use itertools::Itertools;
use winnow::{
    ascii::{dec_uint, hex_uint},
    token::one_of,
    PResult, Parser,
};

pub fn solve(input: &String) -> Result<String> {
    let digger_steps: Vec<(char, u32, u64)> = input
        .lines()
        .map(|mut line| digger_step(&mut line).map_err(|parse_err| anyhow!(parse_err)))
        .collect::<Result<Vec<(char, u32, u64)>>>()?;

    // I really should check for backwards stepping but I'm like 3 days behind
    if digger_steps
        .iter()
        .tuple_windows()
        .any(|(step1, step2)| step1.0 == step2.0)
    {
        return Err(anyhow!("two cinsecutive steps in the same direction"));
    }

    let mut left_right_cursor = 0;
    let mut up_down_cursor = 0;
    let mut map = Vec::<((i32, i32), char)>::new();
    digger_steps
        .iter()
        .fold('S', |last_dir, (direction, steps, _)| {
            match direction {
                _ if last_dir == 'S' => {
                    map.push(((up_down_cursor, left_right_cursor), 'S'));
                }

                'L' if last_dir == 'D' => {
                    map.push(((up_down_cursor, left_right_cursor), 'J'));
                }
                'L' if last_dir == 'U' => {
                    map.push(((up_down_cursor, left_right_cursor), '7'));
                }

                'R' if last_dir == 'D' => {
                    map.push(((up_down_cursor, left_right_cursor), 'L'));
                }
                'R' if last_dir == 'U' => {
                    map.push(((up_down_cursor, left_right_cursor), 'F'));
                }

                'U' if last_dir == 'R' => {
                    map.push(((up_down_cursor, left_right_cursor), 'J'));
                }
                'U' if last_dir == 'L' => {
                    map.push(((up_down_cursor, left_right_cursor), 'L'));
                }

                'D' if last_dir == 'R' => {
                    map.push(((up_down_cursor, left_right_cursor), '7'));
                }
                'D' if last_dir == 'L' => {
                    map.push(((up_down_cursor, left_right_cursor), 'F'));
                }
                _ => panic!("invalid direction passed parsing"),
            };

            for _ in 1..*steps {
                match direction {
                    'U' => up_down_cursor -= 1,
                    'D' => up_down_cursor += 1,
                    'L' => left_right_cursor -= 1,
                    'R' => left_right_cursor += 1,
                    _ => panic!("invalid direction passed parsing"),
                };

                let pipe_char = match direction {
                    'U' | 'D' => '|',
                    'L' | 'R' => '-',
                    _ => panic!("invalid direction passed parsing"),
                };

                map.push(((up_down_cursor, left_right_cursor), pipe_char));
            }
            match direction {
                'U' => up_down_cursor -= 1,
                'D' => up_down_cursor += 1,
                'L' => left_right_cursor -= 1,
                'R' => left_right_cursor += 1,
                _ => panic!("invalid direction passed parsing"),
            };

            *direction
        });

    let min_row = map.iter().map(|entry| entry.0 .0).min().context("wtf?")?;
    let min_col = map.iter().map(|entry| entry.0 .1).min().context("wtf?")?;
    let max_row = map.iter().map(|entry| entry.0 .0).max().context("wtf?")?;
    let max_col = map.iter().map(|entry| entry.0 .1).max().context("wtf?")?;

    let mut map_as_pipes =
        vec![vec!['.'; (max_col - min_col + 1) as usize]; (max_row - min_row + 1) as usize];
    map.iter().for_each(|((i, j), c)| {
        map_as_pipes[(*i - min_row) as usize][(*j - min_col) as usize] = *c;
    });

    let val = crate::day_10::day_10b::calc_inside(&mut map_as_pipes)?;
    return Ok((val + map.len() as u32).to_string());
}

fn digger_step<'s>(input: &mut &'s str) -> PResult<(char, u32, u64)> {
    let (dig_direction, _, steps, _, hex_line, _) = (
        one_of(['U', 'R', 'D', 'L']),
        " ",
        dec_uint,
        " (#",
        hex_uint,
        ")",
    )
        .parse_next(input)?;
    Ok((dig_direction, steps, hex_line))
}
