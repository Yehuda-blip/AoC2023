use std::collections::HashSet;

use itertools::Itertools;

use anyhow::{anyhow, Context, Result};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn reorient(orientation: Direction, pipe: char) -> Option<Direction> {
    match pipe {
        '|' => match orientation {
            Direction::North => Some(Direction::North),
            Direction::South => Some(Direction::South),
            _ => None,
        },
        '-' => match orientation {
            Direction::East => Some(Direction::East),
            Direction::West => Some(Direction::West),
            _ => None,
        },
        'L' => match orientation {
            Direction::South => Some(Direction::East),
            Direction::West => Some(Direction::North),
            _ => None,
        },
        'J' => match orientation {
            Direction::East => Some(Direction::North),
            Direction::South => Some(Direction::West),
            _ => None,
        },
        '7' => match orientation {
            Direction::North => Some(Direction::West),
            Direction::East => Some(Direction::South),
            _ => None,
        },
        'F' => match orientation {
            Direction::North => Some(Direction::East),
            Direction::West => Some(Direction::South),
            _ => None,
        },
        _ => None,
    }
}

fn step(
    orientation: Direction,
    position: (usize, usize),
    map: &Vec<Vec<char>>,
) -> Option<(Direction, (usize, usize))> {
    let (row, col) = position;
    let pipe = map.get(row)?.get(col)?;
    match reorient(orientation, *pipe) {
        Some(Direction::North) if row > 0 => Some((Direction::North, (row - 1, col))),
        Some(Direction::East) => Some((Direction::East, (row, col + 1))),
        Some(Direction::South) => Some((Direction::South, (row + 1, col))),
        Some(Direction::West) if col > 0 => Some((Direction::West, (row, col - 1))),
        _ => None,
    }
}

fn virtual_pipe_step(
    pipe: char,
    orientation: Direction,
    position: (usize, usize),
) -> Option<(Direction, (usize, usize))> {
    let (row, col) = position;
    match reorient(orientation, pipe) {
        Some(Direction::North) if row > 0 => Some((Direction::North, (row - 1, col))),
        Some(Direction::East) => Some((Direction::East, (row, col + 1))),
        Some(Direction::South) => Some((Direction::South, (row + 1, col))),
        Some(Direction::West) if col > 0 => Some((Direction::West, (row, col - 1))),
        _ => None,
    }
}

pub(super) fn longtitude_latitude_map(map: &str) -> Vec<Vec<char>> {
    map.lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec()
}

fn pipe_as_start_option(
    start_pos: (usize, usize),
    pipe: char,
    map: &Vec<Vec<char>>,
) -> Option<(char, Direction)> {
    [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ]
    .iter()
    .combinations(2)
    .find_map(|comb| match comb[..] {
        [d1, d2] => {
            if d1 == d2 {return None}
            for orientation in [d1, d2] {
                let (orientation, position) = virtual_pipe_step(pipe, *orientation, start_pos)?;
                let _ = step(orientation, position, map)?;
            }
            return Some((pipe, *d1));
        }
        _ => panic!("AAAHHH WTF HAPPENED?"),
    })
}

pub(super) fn find_largest_loop(start_pos: (usize, usize), map: &Vec<Vec<char>>) -> Result<(HashSet<(usize, usize)>, char)> {
    let loops = ['|', '-', 'L', 'J', '7', 'F']
        .iter()
        .filter_map(|start_pipe_option| pipe_as_start_option(start_pos, *start_pipe_option, map))
        .map(|(start_pipe, orientation)| {
            let (mut orientation, mut position) =
                virtual_pipe_step(start_pipe, orientation, start_pos)
                    .context("this was supposed to be a viable starting pipe...")?;
            let mut loop_set = HashSet::<(usize, usize)>::new();
            loop_set.insert(position);
            while start_pos != position {
                let (reorientation, reposition) =
                    step(orientation, position, map).context("path is leading nowhere")?;
                if loop_set.contains(&reposition) {
                    return Err(anyhow!("ye done fucked up mate"));
                }
                loop_set.insert(reposition);
                (position, orientation) = (reposition, reorientation);
            }
            loop_set.insert(start_pos);
            Ok((loop_set, start_pipe))
        })
        .collect::<Result<Vec<(HashSet<(usize, usize)>, char)>>>()?;
    loops
        .into_iter()
        .max_by_key(|loop_set| loop_set.0.len())
        .context("could not find any loops")
}

pub fn solve(input: &String) -> Result<String> {
    let map = longtitude_latitude_map(input);
    let start_pos = (0..map.len())
        .flat_map(|i| (0..map[i].len()).map(move |j| (i, j)))
        .find(|(row, col)| map[*row][*col] == 'S')
        .context("no start position found")?;
    let largets_loop = find_largest_loop(start_pos, &map)?;
    Ok(((largets_loop.0.len() + 1) / 2).to_string())
}
