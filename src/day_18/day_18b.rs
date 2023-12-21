use anyhow::{Result, anyhow};
use winnow::{
    ascii::{dec_uint, hex_uint},
    token::one_of,
    Parser, error::{ErrMode, ContextError},
};

use super::day_18a::virtual_additions;

pub fn solve(input: &String) -> Result<String> {
    let digger_steps = input
        .lines()
        .map(|mut line| digger_step(&mut line).map_err(|parse_err| anyhow!(parse_err)))
        .collect::<Result<Vec<(char, i64)>>>()?;

    let area = virtual_additions(&digger_steps);
    Ok(area.to_string())
}

fn digger_step<'s>(input: &mut &'s str) -> Result<(char, i64)> {
    let (_, _, _, _, hex_line, _) = (
        one_of(['U', 'R', 'D', 'L']),
        " ",
        dec_uint::<&'s str, u32, ContextError>,
        " (#",
        hex_uint::<&'s str, u64, ContextError>,
        ")",
    )
        .parse_next(input).map_err(|parse_err: ErrMode<ContextError>| anyhow!(parse_err))?;
    
    let (steps, direction_as_num) = (hex_line / 0x10, hex_line % 0x10);
    let direction = match direction_as_num {
        0 => 'R',
        1 => 'D',
        2 => 'L',
        3 => 'U',
        _ => {return Err(anyhow!("winnow errors are confusing..."))}
    };

    Ok((direction, steps as i64))
}