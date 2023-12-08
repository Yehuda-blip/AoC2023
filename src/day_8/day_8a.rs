use std::collections::HashMap;

use anyhow::{Context, Ok, Result};
use winnow::{token::take_while, PResult, Parser};

use crate::utils::DOUBLE_LINE_ENDING;

const UNMARKED: u32 = 0;

enum User {
    Person,
    Ghost,
}

struct Node<'input> {
    id: &'input str,
    left: &'input str,
    right: &'input str,
    marker: u32,
}

pub fn solve(input: &String) -> Result<String> {
    let (directions, map) = input.split_once(DOUBLE_LINE_ENDING).context("bad input")?;
    let (my_locations, desert_map) = construct_map(map, User::Person)?;
    let mut current_location = desert_map
        .get(*my_locations.first().context("could not find start")?)
        .context("start is not in map")?;
    let steps = 1 + directions
        .chars()
        .cycle()
        .take_while(|direction| {
            match direction {
                'R' => current_location = desert_map.get(current_location.right).unwrap(),
                'L' => current_location = desert_map.get(current_location.left).unwrap(),
                _ => panic!("fuck"),
            }
            current_location.id != "ZZZ"
        })
        .count();
    return Ok(steps.to_string());
}

fn construct_map<'a>(
    map_instructions: &'a str,
    who_is_this_for: User,
) -> Result<(Vec<&'a str>, HashMap<&'a str, Node<'a>>)> {
    let map = map_instructions
        .lines()
        .map(|mut line| node(&mut line))
        .collect::<Result<HashMap<&'a str, Node<'a>>>>()?;
    let start_locations = map
        .iter()
        .filter_map(|(location_id, node)| match who_is_this_for {
            User::Person if *location_id == "AAA" => Some(*location_id),
            User::Ghost if location_id.chars().last()? == 'A' => Some(*location_id),
            _ => None,
        })
        .collect();
    Ok((start_locations, map))
}

fn node<'a>(line: &mut &'a str) -> Result<(&'a str, Node<'a>)> {
    let (curr_node, _, _, left, _, right, _) = (node_id, " = ", '(', node_id, ", ", node_id, ')')
        .parse_next(line)
        .map_err(anyhow::Error::msg)?;
    Ok((
        curr_node,
        Node {
            id: curr_node,
            left: left,
            right: right,
            marker: UNMARKED,
        },
    ))
}

fn node_id<'a>(node_str: &mut &'a str) -> PResult<&'a str> {
    take_while(0.., 'A'..='Z').parse_next(node_str)
}
