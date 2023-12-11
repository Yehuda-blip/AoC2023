use std::{collections::HashSet, usize};

use anyhow::Result;
use itertools::Itertools;

#[derive(PartialEq, Eq, Hash)]
enum Axis {
    Row(usize),
    Col(usize),
}

const EXPANSION: usize = 2;

pub(super) fn solve_with_expansion(input: &String, expansion: usize) -> Result<String> {
    let (mut galaxies, mut by_lon_lat) = (Vec::<(usize, usize)>::new(), HashSet::<Axis>::new());
    input.lines().enumerate().for_each(|(i, line)| {
        line.chars()
            .enumerate()
            .filter(|(_j, c)| *c == '#')
            .for_each(|(j, _c)| {
                galaxies.push((i, j));
                by_lon_lat.insert(Axis::Row(i));
                by_lon_lat.insert(Axis::Col(j));
            })
    });
    let dist = |points: ((usize, usize), (usize, usize))| {
        let expand = |axis| match by_lon_lat.contains(&axis) {
            true => 1,
            false => expansion,
        };
        let ((y1, x1), (y2, x2)) = points;
        ((y1.min(y2))..(y1.max(y2))).fold(0, |acc, row| {
            acc + expand(Axis::Row(row))
        }) + ((x1.min(x2))..(x1.max(x2))).fold(0, |acc, col| {
            acc + expand(Axis::Col(col))
        })
    };
    let combs = galaxies.into_iter().rev().tuple_combinations();
    let distance_total: usize = combs.map(|points| dist(points)).sum();
    return Ok(distance_total.to_string());
}

pub fn solve(input: &String) -> Result<String> {
    solve_with_expansion(input, EXPANSION)
}
