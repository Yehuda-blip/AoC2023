use anyhow::{Context, Result};

use crate::utils::DOUBLE_LINE_ENDING;

use super::day_8a::{construct_map, User};

pub fn solve(input: &String) -> Result<String> {
    let (directions, locations_list) = input
        .split_once(DOUBLE_LINE_ENDING)
        .context("bad instructions")?;

    let (mut start_locations, mut desert_map) = construct_map(locations_list, User::Ghost)?;
    let mut steps = 0;
    let mut res = 0;
    directions
        .chars()
        .cycle()
        .take(1000000000)
        .for_each(|c| {
            start_locations = start_locations
                .iter()
                .map(|loc| {
                    let curr = desert_map.get(loc).unwrap();
                    match c {
                        'R' => curr.right,
                        'L' => curr.left,
                        _ => panic!("AAAAAAA"),
                    }
                })
                .collect();
        });

    return Ok(res.to_string());
}

fn empty_target(locs: &Vec<&str>) -> bool {
    locs.iter().any(|loc| !is_target(loc))
}

fn is_target(node_id: &str) -> bool {
    node_id.chars().last().unwrap() == 'Z'
}
