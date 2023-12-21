use anyhow::{anyhow, Result};
use winnow::{
    ascii::{dec_uint, hex_uint},
    token::one_of,
    PResult, Parser, error::ContextError,
};

pub fn solve(input: &String) -> Result<String> {
    let digger_steps = input
        .lines()
        .map(|mut line| digger_step(&mut line).map_err(|parse_err| anyhow!(parse_err)))
        .collect::<Result<Vec<(char, i64)>>>()?;

    let area = virtual_additions(&digger_steps);
    return Ok(area.to_string());
}

fn digger_step<'s>(input: &mut &'s str) -> PResult<(char, i64)> {
    let (dig_direction, _, steps, _, _, _) = (
        one_of(['U', 'R', 'D', 'L']),
        " ",
        dec_uint::<&'s str, u64, ContextError>,
        " (#",
        hex_uint::<&'s str, u64, ContextError>,
        ")",
    )
        .parse_next(input)?;
    Ok((dig_direction, steps as i64))
}

pub(super) fn virtual_additions(digger_steps: &Vec<(char, i64)>) -> i64 {
    /*
     * This is non trivial (at the very least, this took me several hours to figure out).
     * The basic idea is to sum many integrals of constant fuctions. Some of the integrals
     * have a positive orientation (in this implementation, movements to the right), and
     * the other have a negative orientation (movements to left - integrating from a larger
     * value to a lower one). We can calculate the constant value of each function as the
     * current 'vertical_pos'.
     * The idea here is to relate to the trench as having an infinite vertical position,
     * which is easily offset back to a 0 vertical position. Because the trench closes 
     * a loop, each positive integral (right movement) must have some corresponding 
     * negative integration (which could contain parts of multiple negative integrals).
     * By assuming that each right movement adds infinity * movement to the sum, and each
     * left movement removes infinity * movement from the sum, we can let them cancel each
     * other out naturally and just relate to the horizontal position of the current integral
     * offset to 0. Of course, nothing guarantees that we are calcultaing in the correct horizontal
     * orientation, and so we might get a negative area which will need to tranform into absolute
     * value.
     * In this implementation the descrete nature of the problem, together with the calculation
     * of the circumference as part of the area, mean that we do not add the first cubic meter
     * to the sum of the circumference, and only half of the circumference is computed inside the
     * area (one of up/down and one of left/right), this is accounted for in the return statement.
     */
    let mut vertical_pos = 0;
    let mut area = 0;
    let mut circumference = 0;
    for (direction, step_size) in digger_steps {
        circumference += step_size;
        match direction {
            'U' => vertical_pos += step_size,
            'D' => vertical_pos -= step_size,
            'R' => area += vertical_pos * step_size,
            'L' => area -= vertical_pos * step_size,
            _ => {panic!("bad direction, what happened?")}
        }
    };
    let area = area.abs();

    return area + (circumference / 2) + 1;
}

