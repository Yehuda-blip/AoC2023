use std::collections::HashMap;

use anyhow::{Context, Ok, Result};
use winnow::{token::take_while, PResult, Parser};

use crate::utils::DOUBLE_LINE_ENDING;

pub(super) const UNMARKED: u32 = 0;

pub(super) enum User {
    Person,
    _Ghost,
}

pub(super) struct Node<'a> {
    pub id: &'a str,
    pub left: &'a str,
    pub right: &'a str,
    pub _marker: u32,
}

pub fn solve(input: &String) -> Result<String> {
    let (directions, locations_list) = input.split_once(DOUBLE_LINE_ENDING).context("bad input")?;
    let (my_locations, desert_map) = construct_map(locations_list, User::Person)?;
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

pub(super) fn construct_map<'a>(
    map_instructions: &'a str,
    who_is_this_for: User,
) -> Result<(Vec<&'a str>, HashMap<&'a str, Node<'a>>)> {
    let map = map_instructions
        .lines()
        .map(|mut line| node(&mut line))
        .collect::<Result<HashMap<&'a str, Node<'a>>>>()?;
    let start_locations = map
        .iter()
        .filter_map(|(location_id, _node)| match who_is_this_for {
            User::Person if *location_id == "AAA" => Some(*location_id),
            User::_Ghost if location_id.chars().last()? == 'A' => Some(*location_id),
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
            _marker: UNMARKED,
        },
    ))
}

fn node_id<'a>(node_str: &mut &'a str) -> PResult<&'a str> {
    take_while(0.., ('A'..='Z', '0'..'9')).parse_next(node_str)
}
